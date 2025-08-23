use std::io;
use thiserror::Error;

use crate::{encryption::EncryptionError, packets::PacketError};

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
    #[error("Encryption token didn't match, got {got}, expected {expected}")]
    TokenDontMatch { expected: u64, got: u64 },
    #[error("Failed to lock mutex")]
    LockError,
    #[error("Stream {0} not found")]
    StreamNotFound(u32),
    #[error("Stream {0} already exists")]
    StreamAlreadyExists(u32),
    #[error("Stream {0} closed")]
    StreamClosed(u32),
    #[error("Failed to send on channel")]
    ChannelSendError,
    #[error("Failed to receive on channel")]
    ChannelReceiveError,
}
