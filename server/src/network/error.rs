use std::io::Error;

use shared::{encryption::EncryptionError, packets::PacketError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unexpected packet")]
    UnexpectedPacket,
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Request timeout")]
    Timeout,
    #[error("Error while handling packet: {0}")]
    PacketError(#[from] PacketError),
    #[error("Error while encrypting or decrypting: {0}")]
    CryptError(#[from] EncryptionError),
    #[error("IO Error: {0}")]
    IoError(#[from] Error),
}
