use bincode::{self, Decode, Encode};
use super::packet::{PacketError, Packet};

#[derive(Debug, Encode, Decode)]
pub struct EncryptionRequest {
    pub key: [u8; 32],
    pub verify_token: u64,
}

impl EncryptionRequest {
    pub const PACKET_SIZE: usize = 42;
    pub fn new(key: [u8; 32], verify_token: u64) -> Self {
        EncryptionRequest { key, verify_token }
    }
}

impl Packet for EncryptionRequest {
    fn serialize(&self) -> Result<Vec<u8>, PacketError> {
        let mut data: Vec<u8> = Vec::new();
        let encoded_packet = match bincode::encode_to_vec(self, bincode::config::standard()) {
            Ok(packet) => packet,
            Err(e) => return Err(PacketError::EncodingError(e.to_string()))
        };
        data.push(0x01);
        data.extend(encoded_packet.as_slice());
        Ok(data)
    }

    fn deserialize(data: &[u8]) -> Result<Self, PacketError>
    where
        Self: Sized,
    {
        let (decoded, _) = match bincode::decode_from_slice(data, bincode::config::standard()) {
            Ok(res) => res,
            Err(e) => return Err(PacketError::DecodingError(e.to_string()))
        };

        Ok(decoded)
    }

    fn packet_code() -> u8 {
        0x01
    }
}
