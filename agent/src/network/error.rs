use thiserror::Error;
use std::io;

use shared::{encryption::EncryptionError, packets::PacketError};



#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unexpected packet")]
    UnexpectedPacket,
    #[error("Error while handling packet: {0}")]
    PacketError(#[from] PacketError),
    #[error("Error while encrypting or decrypting: {0}")]
    CryptError(#[from] EncryptionError),
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Error while converting data")]
    ConvertError,
    #[error("Encrypter Token didn't match, got {got}, expected {expected}")]
    TokenDontMatch{expected: u64, got: u64},
    #[error("Failed to lock mutex")]
    LockError,
}