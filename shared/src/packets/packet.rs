use super::{EncryptionRequest, EncryptionResponse, PacketError, PingRequest, PongResponse};

#[derive(Debug)]
pub enum Packets {
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
    PingRequest(PingRequest),
    PongResponse(PongResponse),
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
        0x01 => Ok(Packets::EncryptionRequest(EncryptionRequest::deserialize(
            data,
        )?)),
        0x02 => Ok(Packets::EncryptionResponse(
            EncryptionResponse::deserialize(data)?,
        )),
        0x03 => Ok(Packets::PingRequest(PingRequest::deserialize(data)?)),
        0x04 => Ok(Packets::PongResponse(PongResponse::deserialize(data)?)),
        _ => Err(PacketError::UnknownPacket(format!("0x{:02X}", packet_code))),
    }
}
