use bincode::{Decode, Encode};

use super::PacketError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[repr(u8)]
pub enum PacketType {
    EncryptionRequest = 0x01,
    EncryptionResponse = 0x02,
    PingRequest = 0x03,
    PongResponse = 0x04,
    ShellCommandRequest = 0x05,
    ShellCommandResponse = 0x06,
    FileUploadRequest = 0x07,
    FileUploadChunk = 0x08,
    FileUploadResponse = 0x09,
    FileDownloadRequest = 0x0A,
    FileDownloadChunk = 0x0B,
    FileDownloadResponse = 0x0C,
    FileChunkTest = 0xFF,
}

impl PacketType {
    pub fn from_code(code: u8) -> Result<Self, PacketError> {
        match code {
            0x01 => Ok(PacketType::EncryptionRequest),
            0x02 => Ok(PacketType::EncryptionResponse),
            0x03 => Ok(PacketType::PingRequest),
            0x04 => Ok(PacketType::PongResponse),
            0x05 => Ok(PacketType::ShellCommandRequest),
            0x06 => Ok(PacketType::ShellCommandResponse),
            0x07 => Ok(PacketType::FileUploadRequest),
            0x08 => Ok(PacketType::FileUploadChunk),
            0x09 => Ok(PacketType::FileUploadResponse),
            0x0A => Ok(PacketType::FileDownloadRequest),
            0x0B => Ok(PacketType::FileDownloadChunk),
            0x0C => Ok(PacketType::FileDownloadResponse),
            0xFF => Ok(PacketType::FileChunkTest),
            _ => Err(PacketError::UnknownPacket(format!("0x{:02X}", code))),
        }
    }

    pub fn code(&self) -> u8 {
        *self as u8
    }
}
