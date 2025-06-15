use std::{io, sync::Arc};

use shared::encryption::encrypt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::Mutex,
};

pub struct Connection {
    reader: OwnedReadHalf,
    writer: Arc<Mutex<OwnedWriteHalf>>,
    shared_secret: [u8; 32],
}

impl Connection {
    pub fn new(
        reader: OwnedReadHalf,
        writer: OwnedWriteHalf,
        shared_secret: [u8; 32],
    ) -> io::Result<Self> {
        Ok(Connection {
            reader,
            writer: Arc::new(Mutex::new(writer)),
            shared_secret,
        })
    }

    pub async fn send(&self, buf: &[u8]) -> Result<(), std::io::Error> {
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

        let mut writer = self.writer.lock().await;

        writer.write(&data).await?;
        writer.flush().await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>, std::io::Error> {
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

        let decrypted_data =
            shared::encryption::decrypt(&self.shared_secret, &nonce_buf, &encrypted_buf).map_err(
                |_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Decryption failed"),
            )?;

        Ok(decrypted_data)
    }

    pub async fn shutdown(&self) -> std::io::Result<()> {
        self.writer.lock().await.shutdown().await
    }
}
