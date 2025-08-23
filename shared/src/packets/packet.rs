use thiserror::Error;

use super::{EncryptionRequest, EncryptionResponse, Heartbeat, StreamOpen, StreamClose, StreamData, StreamError};

#[derive(Debug)]
pub enum Packets {
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
    StreamOpen(StreamOpen),
    StreamClose(StreamClose),
    StreamData(StreamData),
    StreamError(StreamError),
    Heartbeat(Heartbeat)
}

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("Got unknown packet code : {0}")]
    UnknownPacket(String),
    #[error("Error while encoding packet: {0}")]
    EncodingError(String),
    #[error("Error while decoding packet: {0}")]
    DecodingError(String)
}

pub trait Packet {
    fn serialize(&self) -> Result<Vec<u8>, PacketError>;
    fn deserialize(data: &[u8]) -> Result<Self, PacketError>
    where
        Self: Sized;
    fn packet_code() -> u8;
}

pub fn from_packet_bytes(data: &[u8]) -> Result<Packets, PacketError> {
    let packet_code = data[0];
    let data = &data[1..];
    match packet_code {
        0x01 => Ok(Packets::EncryptionRequest(
            EncryptionRequest::deserialize(data)?
        )),
        0x02 => Ok(Packets::EncryptionResponse(
            EncryptionResponse::deserialize(data)?
        )),
        0x03 => Ok(Packets::StreamOpen(
            StreamOpen::deserialize(data)?
        )),
        0x04 => Ok(Packets::StreamClose(
            StreamClose::deserialize(data)?
        )),
        0x05 => Ok(Packets::StreamData(
            StreamData::deserialize(data)?
        )),
        0x06 => Ok(Packets::StreamError(
            StreamError::deserialize(data)?
        )),
        0x07 => Ok(Packets::Heartbeat(
            Heartbeat::deserialize(data)?
        )),
        _ => Err(PacketError::UnknownPacket(packet_code.to_string())),
    }
}
