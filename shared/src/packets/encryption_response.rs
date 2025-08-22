use bincode::{Decode, Encode};

use super::{PacketError, Packet};

#[derive(Debug, Encode, Decode)]
pub struct EncryptionResponse {
    pub key: [u8; 32],
    pub nonce: [u8; 12],
    pub verify_token: [u8; 24],
}

impl EncryptionResponse {
    pub const PACKET_SIZE: usize = 69;
    pub fn new(key: [u8; 32], nonce: [u8; 12], verify_token: [u8; 24]) -> Self {
        EncryptionResponse {
            key,
            nonce,
            verify_token,
        }
    }
}

impl Packet for EncryptionResponse {
    fn serialize(&self) -> Result<Vec<u8>, PacketError> {
        let mut data: Vec<u8> = Vec::new();
        let encoded_packet = match bincode::encode_to_vec(self, bincode::config::standard()) {
            Ok(packet) => packet,
            Err(e) => return Err(PacketError::EncodingError(e.to_string()))
        };
        data.push(0x02);
        data.extend(&encoded_packet);
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
        0x02
    }
}
