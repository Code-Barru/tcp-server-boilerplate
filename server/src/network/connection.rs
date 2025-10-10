use std::sync::Arc;

use crate::network::NetworkError;
use shared::{
    encryption::{decrypt, encrypt},
    packets::Frame,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::{Mutex, mpsc},
};

pub struct Connection {
    reader: OwnedReadHalf,
    pub(crate) writer: Arc<Mutex<OwnedWriteHalf>>,
    pub(crate) shared_secret: [u8; 32],
    frame_tx: mpsc::UnboundedSender<Frame>,
}

use super::request_manager::RequestManager;

pub struct ConnectionHandle {
    pub(crate) writer: Arc<Mutex<OwnedWriteHalf>>,
    pub(crate) shared_secret: [u8; 32],
    frame_rx: Arc<Mutex<mpsc::UnboundedReceiver<Frame>>>,
    request_manager: RequestManager,
}

impl Clone for ConnectionHandle {
    fn clone(&self) -> Self {
        ConnectionHandle {
            writer: self.writer.clone(),
            shared_secret: self.shared_secret,
            frame_rx: self.frame_rx.clone(),
            request_manager: self.request_manager.clone(),
        }
    }
}

impl ConnectionHandle {
    pub async fn send_request(
        &self,
        packet_type: shared::packets::PacketType,
        payload: Vec<u8>,
    ) -> Result<super::request_manager::FrameIterator, NetworkError> {
        let request_id = self.request_manager.next_request_id();
        let iterator = self.request_manager.register_request(request_id).await?;

        let frame = Frame::new(request_id, packet_type, payload);
        self.send_frame(&frame).await?;

        Ok(iterator)
    }

    pub async fn send_request_and_wait(
        &self,
        packet_type: shared::packets::PacketType,
        payload: Vec<u8>,
        timeout: std::time::Duration,
    ) -> Result<Frame, NetworkError> {
        let mut iterator = self.send_request(packet_type, payload).await?;

        match tokio::time::timeout(timeout, iterator.next_frame()).await {
            Ok(Some(response)) => Ok(response),
            Ok(None) => Err(NetworkError::ConnectionClosed),
            Err(_) => Err(NetworkError::Timeout),
        }
    }

    pub fn spawn_response_router(&self) {
        let frame_rx = self.frame_rx.clone();
        let request_manager = self.request_manager.clone();
        tokio::spawn(async move {
            loop {
                let frame = {
                    let mut rx = frame_rx.lock().await;
                    match rx.recv().await {
                        Some(f) => f,
                        None => break,
                    }
                };

                if let Err(e) = request_manager.route_response(frame).await {
                    tracing::error!("Failed to route response: {}", e);
                }
            }
            tracing::info!("Response router stopped");
        });
    }

    async fn send_frame(&self, frame: &Frame) -> Result<(), NetworkError> {
        let frame_bytes = frame.serialize()?;

        let (encrypted_buf, nonce) = encrypt(&self.shared_secret, &frame_bytes)?;

        let len = encrypted_buf.len() as u32;
        let total_size = 4 + nonce.len() + encrypted_buf.len();
        let mut data = vec![0u8; total_size];

        data[0..4].copy_from_slice(&len.to_be_bytes());
        data[4..16].copy_from_slice(&nonce);
        data[16..].copy_from_slice(&encrypted_buf);

        let mut writer = self.writer.lock().await;
        writer.write_all(&data).await?;
        writer.flush().await?;

        Ok(())
    }
}

impl Connection {
    pub fn new(
        reader: OwnedReadHalf,
        writer: OwnedWriteHalf,
        shared_secret: [u8; 32],
    ) -> (Self, ConnectionHandle) {
        let (frame_tx, frame_rx) = mpsc::unbounded_channel();

        let writer = Arc::new(Mutex::new(writer));
        let request_manager = RequestManager::new();

        let connection = Connection {
            reader,
            writer: writer.clone(),
            shared_secret,
            frame_tx,
        };

        let handle = ConnectionHandle {
            writer,
            shared_secret,
            frame_rx: Arc::new(Mutex::new(frame_rx)),
            request_manager,
        };

        (connection, handle)
    }

    pub async fn run(mut self) -> Result<(), NetworkError> {
        loop {
            let frame = match self.receive_frame().await {
                Ok(frame) => frame,
                Err(NetworkError::ConnectionClosed) => {
                    tracing::info!("Connection closed by client");
                    break;
                }
                Err(e) => {
                    tracing::error!("Failed to receive frame: {}", e);
                    return Err(e);
                }
            };

            if self.frame_tx.send(frame).is_err() {
                tracing::error!("Failed to send frame to handler - channel closed");
                break;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn send_frame(&self, frame: &Frame) -> Result<(), NetworkError> {
        let frame_bytes = frame.serialize()?;
        self.send(&frame_bytes).await
    }

    pub async fn receive_frame(&mut self) -> Result<Frame, NetworkError> {
        let frame_bytes = self.receive().await?;

        if frame_bytes.is_empty() {
            return Err(NetworkError::ConnectionClosed);
        }

        let frame = Frame::deserialize(&frame_bytes)?;
        Ok(frame)
    }

    #[allow(dead_code)]
    pub async fn send(&self, buf: &[u8]) -> Result<(), NetworkError> {
        let (encrypted_buf, nonce) = encrypt(&self.shared_secret, buf)?;

        let len = encrypted_buf.len() as u32;
        let total_size = 4 + nonce.len() + encrypted_buf.len();
        let mut data = vec![0u8; total_size];

        data[0..4].copy_from_slice(&len.to_be_bytes());
        data[4..16].copy_from_slice(&nonce);
        data[16..].copy_from_slice(&encrypted_buf);

        let mut writer = self.writer.lock().await;

        writer.write_all(&data).await?;
        writer.flush().await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>, NetworkError> {
        let mut len_buf = [0u8; 4];
        self.reader.read_exact(&mut len_buf).await?;

        let len = u32::from_be_bytes(len_buf) as usize;
        if len == 0 {
            return Ok(Vec::new());
        }

        let mut nonce_buf = vec![0u8; 12];
        self.reader.read_exact(&mut nonce_buf).await?;

        let mut encrypted_buf = vec![0u8; len];
        self.reader.read_exact(&mut encrypted_buf).await?;

        let decrypted_data = decrypt(&self.shared_secret, &nonce_buf, &encrypted_buf)?;

        Ok(decrypted_data)
    }

    #[allow(dead_code)]
    pub async fn shutdown(&self) -> std::io::Result<()> {
        self.writer.lock().await.shutdown().await
    }
}
