use thiserror::Error;

use super::{EncryptionRequest, EncryptionResponse};

#[derive(Debug)]
pub enum Packets {
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
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
        _ => Err(PacketError::UnknownPacket(packet_code.to_string())),
    }
}
