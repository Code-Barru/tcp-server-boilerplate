use std::sync::{Arc, Mutex};

use shared::{
    encryption::{decrypt, encrypt},
    packets::Frame,
};

use crate::network::error::NetworkError;

use super::{Connection, ReadHalf, WriteHalf};

#[derive(Debug)]
pub struct Client {
    reader: ReadHalf,
    writer: Arc<Mutex<WriteHalf>>,
    shared_secret: [u8; 32],
}

impl Client {
    pub fn new(addr: &str) -> Result<Self, NetworkError> {
        let connection = Connection::connect(addr)?;
        let (mut reader, mut writer) = connection.split();

        let shared_secret = super::perform_handshake(&mut reader, &mut writer)?;

        Ok(Client {
            reader,
            writer: Arc::new(Mutex::new(writer)),
            shared_secret,
        })
    }

    #[allow(dead_code)]
    pub fn shutdown(&self) -> Result<(), NetworkError> {
        let mut writer = match self.writer.lock() {
            Ok(writer) => writer,
            Err(_) => {
                return Err(NetworkError::LockError);
            }
        };

        writer.write(&[])?;
        writer.flush()?;
        self.writer.lock().unwrap().shutdown()?;
        self.reader.shutdown()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn send_frame(&self, frame: &Frame) -> Result<(), NetworkError> {
        let frame_bytes = frame.serialize()?;
        self.send(&frame_bytes)
    }

    #[allow(dead_code)]
    pub fn receive_frame(&mut self) -> Result<Frame, NetworkError> {
        let frame_bytes = self.receive()?;

        if frame_bytes.is_empty() {
            return Err(NetworkError::ConnectionClosed);
        }

        let frame = Frame::deserialize(&frame_bytes)?;
        Ok(frame)
    }

    #[allow(dead_code)]
    pub fn send(&self, buf: &[u8]) -> Result<(), NetworkError> {
        let (encrypted_buf, nonce) = encrypt(&self.shared_secret, buf)?;

        let mut data = Vec::with_capacity(4 + nonce.len() + encrypted_buf.len());
        let len = encrypted_buf.len() as u32;

        data.extend_from_slice(&len.to_be_bytes());
        data.extend_from_slice(&nonce);
        data.extend_from_slice(&encrypted_buf);

        let mut writer = match self.writer.lock() {
            Ok(writer) => writer,
            Err(_) => {
                return Err(NetworkError::LockError);
            }
        };

        writer.write(&data)?;
        writer.flush()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn receive(&mut self) -> Result<Vec<u8>, NetworkError> {
        let mut len_buf = [0u8; 4];
        self.reader.read_exact(&mut len_buf)?;

        let len = u32::from_be_bytes(len_buf) as usize;
        if len == 0 {
            return Ok(Vec::new());
        }

        let mut nonce_buf = vec![0u8; 12];
        self.reader.read_exact(&mut nonce_buf)?;

        let mut encrypted_buf = vec![0u8; len];
        self.reader.read_exact(&mut encrypted_buf)?;

        let decrypted_data = decrypt(&self.shared_secret, &nonce_buf, &encrypted_buf)?;

        Ok(decrypted_data)
    }

    pub fn split(self) -> (ClientReader, ClientWriter) {
        let reader = ClientReader {
            reader: self.reader,
            shared_secret: self.shared_secret,
        };

        let writer = ClientWriter {
            writer: self.writer,
            shared_secret: self.shared_secret,
        };

        (reader, writer)
    }
}

pub struct ClientReader {
    reader: ReadHalf,
    shared_secret: [u8; 32],
}

impl ClientReader {
    pub fn receive_frame(&mut self) -> Result<Frame, NetworkError> {
        let frame_bytes = self.receive()?;

        if frame_bytes.is_empty() {
            return Err(NetworkError::ConnectionClosed);
        }

        let frame = Frame::deserialize(&frame_bytes)?;
        Ok(frame)
    }

    pub fn receive(&mut self) -> Result<Vec<u8>, NetworkError> {
        let mut len_buf = [0u8; 4];
        self.reader.read_exact(&mut len_buf)?;

        let len = u32::from_be_bytes(len_buf) as usize;
        if len == 0 {
            return Ok(Vec::new());
        }

        let mut nonce_buf = vec![0u8; 12];
        self.reader.read_exact(&mut nonce_buf)?;

        let mut encrypted_buf = vec![0u8; len];
        self.reader.read_exact(&mut encrypted_buf)?;

        let decrypted_data = decrypt(&self.shared_secret, &nonce_buf, &encrypted_buf)?;

        Ok(decrypted_data)
    }

    #[allow(dead_code)]
    pub fn shutdown(&self) -> Result<(), NetworkError> {
        self.reader.shutdown()?;
        Ok(())
    }
}

pub struct ClientWriter {
    writer: Arc<Mutex<WriteHalf>>,
    shared_secret: [u8; 32],
}

impl ClientWriter {
    pub fn send_frame(&self, frame: &Frame) -> Result<(), NetworkError> {
        let frame_bytes = frame.serialize()?;
        self.send(&frame_bytes)
    }

    pub fn send(&self, buf: &[u8]) -> Result<(), NetworkError> {
        let (encrypted_buf, nonce) = encrypt(&self.shared_secret, buf)?;

        let mut data = Vec::with_capacity(4 + nonce.len() + encrypted_buf.len());
        let len = encrypted_buf.len() as u32;

        data.extend_from_slice(&len.to_be_bytes());
        data.extend_from_slice(&nonce);
        data.extend_from_slice(&encrypted_buf);

        let mut writer = match self.writer.lock() {
            Ok(writer) => writer,
            Err(_) => {
                return Err(NetworkError::LockError);
            }
        };

        writer.write(&data)?;
        writer.flush()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn shutdown(&self) -> Result<(), NetworkError> {
        let mut writer = self.writer.lock().map_err(|_| NetworkError::LockError)?;
        writer.write(&[])?;
        writer.flush()?;
        writer.shutdown()?;
        Ok(())
    }
}
