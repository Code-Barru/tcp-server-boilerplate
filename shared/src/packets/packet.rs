use crate::packets::Fingerprint;

use super::{EncryptionRequest, EncryptionResponse};

#[derive(Debug)]
pub enum Packets {
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
    Fingerprint(Fingerprint),
}

#[derive(Debug)]
pub enum Error {
    UnknownPacket,
    ParseError,
    InvalidData,
}

#[allow(dead_code)]
pub trait Packet {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
    fn packet_code() -> u8;
}

pub fn from_packet_bytes(data: &[u8]) -> Result<Packets, Error> {
    let packet_code = data[0];
    let data = &data[1..];
    match packet_code {
        0x01 => Ok(Packets::EncryptionRequest(EncryptionRequest::deserialize(
            data,
        )?)),
        0x02 => Ok(Packets::EncryptionResponse(
            EncryptionResponse::deserialize(data)?,
        )),
        0x03 => Ok(Packets::Fingerprint(Fingerprint::deserialize(data)?)),
        _ => Err(Error::UnknownPacket),
    }
}
