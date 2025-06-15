use std::sync::{Arc, Mutex};

use shared::encryption::encrypt;

use super::{Connection, ReadHalf, WriteHalf};

#[derive(Debug)]
pub struct Client {
    reader: ReadHalf,
    writer: Arc<Mutex<WriteHalf>>,
    shared_secret: [u8; 32],
}

impl Client {
    pub fn new(addr: &str) -> std::io::Result<Self> {
        let connection = Connection::connect(addr)?;
        let (mut reader, mut writer) = connection.split();

        let shared_secret = super::perform_handshake(&mut reader, &mut writer)?;

        Ok(Client {
            reader,
            writer: Arc::new(Mutex::new(writer)),
            shared_secret,
        })
    }

    pub fn shutdown(&self) -> std::io::Result<()> {
        self.writer.lock().unwrap().shutdown()?;
        self.reader.shutdown()
    }

    pub fn send(&self, buf: &[u8]) -> Result<(), std::io::Error> {
        let (encrypted_buf, nonce) = match encrypt(&self.shared_secret, buf) {
            Ok(data) => data,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Encryption failed",
                ));
            }
        };

        let mut data = Vec::with_capacity(4 + nonce.len() + encrypted_buf.len());
        let len = encrypted_buf.len() as u32;

        data.extend_from_slice(&len.to_be_bytes());
        data.extend_from_slice(&nonce);
        data.extend_from_slice(&encrypted_buf);

        let mut writer = match self.writer.lock() {
            Ok(writer) => writer,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to lock writer",
                ));
            }
        };

        writer.write(&data)?;
        writer.flush()?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Vec<u8>, std::io::Error> {
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

        let decrypted_data =
            shared::encryption::decrypt(&self.shared_secret, &nonce_buf, &encrypted_buf).map_err(
                |_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Decryption failed"),
            )?;

        Ok(decrypted_data)
    }
}
