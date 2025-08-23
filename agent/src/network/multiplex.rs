use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crossbeam::channel;

use shared::{
    error::NetworkError,
    multiplexing::{MIN_DATA_STREAM_ID, StreamId},
    packets::{Packet, Packets, StreamClose, StreamData, StreamOpen, from_packet_bytes},
};

use super::{client::Client, stream::Stream, ReadHalf, WriteHalf};
use shared::encryption::{encrypt, decrypt};

pub struct MultiplexManager {
    reader: Mutex<ReadHalf>,
    writer: Arc<Mutex<WriteHalf>>,
    shared_secret: [u8; 32],
    streams: Arc<Mutex<HashMap<StreamId, channel::Sender<Vec<u8>>>>>,
    next_id: AtomicU32,
    incoming_streams_tx: channel::Sender<Stream>,
    incoming_streams_rx: Arc<Mutex<channel::Receiver<Stream>>>,
}

impl MultiplexManager {
    pub fn new(client: Client) -> Self {
        let (incoming_tx, incoming_rx) = channel::unbounded();

        let (reader, writer, shared_secret) = client.into_parts();

        Self {
            reader: Mutex::new(reader),
            writer,
            shared_secret,
            streams: Arc::new(Mutex::new(HashMap::new())),
            next_id: AtomicU32::new(MIN_DATA_STREAM_ID),
            incoming_streams_tx: incoming_tx,
            incoming_streams_rx: Arc::new(Mutex::new(incoming_rx)),
        }
    }

    pub fn start(self: &Arc<Self>) -> thread::JoinHandle<()> {
        let self_clone = self.clone();
        thread::spawn(move || {
            if let Err(e) = self_clone.receive_loop() {
                eprintln!("Multiplex receive loop error: {e}");
            }
        })
    }

    pub fn open_stream(self: &Arc<Self>) -> Result<Stream, NetworkError> {
        let stream_id = self.next_id.fetch_add(1, Ordering::SeqCst);

        let (stream_tx, stream_rx) = channel::unbounded();

        {
            let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
            if streams.contains_key(&stream_id) {
                return Err(NetworkError::StreamAlreadyExists(stream_id));
            }
            streams.insert(stream_id, stream_tx);
        }

        let open_packet = StreamOpen { stream_id };
        self.send_packet(&open_packet.serialize()?)?;

        Ok(Stream::new(stream_id, self.clone(), stream_rx))
    }

    pub fn accept_stream(&self) -> Result<Stream, NetworkError> {
        let rx = self
            .incoming_streams_rx
            .lock()
            .map_err(|_| NetworkError::LockError)?;
        rx.recv().map_err(|_| NetworkError::ChannelReceiveError)
    }

    pub fn send_on_stream(&self, stream_id: StreamId, data: Vec<u8>) -> Result<(), NetworkError> {
        let packet = StreamData { stream_id, data };
        self.send_packet(&packet.serialize()?)
    }

    pub fn close_stream(&self, stream_id: StreamId) -> Result<(), NetworkError> {
        {
            let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
            streams.remove(&stream_id);
        }
        let close_packet = StreamClose { stream_id };
        self.send_packet(&close_packet.serialize()?)
    }

    fn receive_loop(self: &Arc<Self>) -> Result<(), NetworkError> {
        loop {
            let data = self.receive_packet()?;

            let packet = from_packet_bytes(&data)?;

            println!("");
            match packet {
                Packets::StreamOpen(open) => {
                    self.handle_stream_open(open)?;
                }
                Packets::StreamClose(close) => {
                    self.handle_stream_close(close)?;
                }
                Packets::StreamData(data_packet) => {
                    self.handle_stream_data(data_packet)?;
                }
                Packets::StreamError(error) => {
                    eprintln!("Stream {} error: {}", error.stream_id, error.error);
                }
                _ => {
                    eprintln!("Unexpected packet in multiplex receive loop");
                }
            }
        }
    }

    fn handle_stream_open(self: &Arc<Self>, open: StreamOpen) -> Result<(), NetworkError> {
        let stream_id = open.stream_id;

        let (tx, rx) = channel::unbounded();

        {
            let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
            if streams.contains_key(&stream_id) {
                return Err(NetworkError::StreamAlreadyExists(stream_id));
            }
            streams.insert(stream_id, tx.clone());
        }

        let stream = Stream::new(stream_id, self.clone(), rx);
        self.incoming_streams_tx
            .send(stream)
            .map_err(|_| NetworkError::ChannelSendError)?;
        Ok(())
    }

    fn handle_stream_close(&self, close: StreamClose) -> Result<(), NetworkError> {
        let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
        streams.remove(&close.stream_id);
        Ok(())
    }

    fn handle_stream_data(&self, data: StreamData) -> Result<(), NetworkError> {
        let streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;

        if let Some(tx) = streams.get(&data.stream_id) {
            tx.send(data.data)
                .map_err(|_| NetworkError::ChannelSendError)?;
        } else {
            eprintln!("Received data for unknown stream: {}", data.stream_id);
        }

        Ok(())
    }

    fn send_packet(&self, buf: &[u8]) -> Result<(), NetworkError> {
        let (encrypted_buf, nonce) = encrypt(&self.shared_secret, buf)?;

        let len = encrypted_buf.len() as u32;
        let total_size = 4 + nonce.len() + encrypted_buf.len();
        let mut data = vec![0u8; total_size];

        data[0..4].copy_from_slice(&len.to_be_bytes());
        data[4..16].copy_from_slice(&nonce);
        data[16..].copy_from_slice(&encrypted_buf);

        let mut writer = self.writer.lock().map_err(|_| NetworkError::LockError)?;
        writer.write_all(&data)?;
        writer.flush()?;
        Ok(())
    }

    fn receive_packet(&self) -> Result<Vec<u8>, NetworkError> {
        let mut reader = self.reader.lock().map_err(|_| NetworkError::LockError)?;

        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;

        let len = u32::from_be_bytes(len_buf) as usize;
        if len == 0 {
            return Ok(Vec::new());
        }

        let mut nonce_buf = vec![0u8; 12];
        reader.read_exact(&mut nonce_buf)?;

        let mut encrypted_buf = vec![0u8; len];
        reader.read_exact(&mut encrypted_buf)?;

        let decrypted_data = decrypt(&self.shared_secret, &nonce_buf, &encrypted_buf)?;

        Ok(decrypted_data)
    }
}
