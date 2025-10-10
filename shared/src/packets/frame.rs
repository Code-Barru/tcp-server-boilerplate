use bincode::{Decode, Encode};

use super::{PacketError, PacketType};

#[derive(Debug, Clone, Encode, Decode)]
pub struct Frame {
    pub request_id: u64,
    pub packet_type: PacketType,
    pub is_last: bool,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn new(request_id: u64, packet_type: PacketType, payload: Vec<u8>) -> Self {
        Frame {
            request_id,
            packet_type,
            is_last: true,
            payload,
        }
    }

    pub fn new_with_flag(request_id: u64, packet_type: PacketType, is_last: bool, payload: Vec<u8>) -> Self {
        Frame {
            request_id,
            packet_type,
            is_last,
            payload,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, PacketError> {
        bincode::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| PacketError::EncodingError(e.to_string()))
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, PacketError> {
        let (frame, _) = bincode::decode_from_slice(data, bincode::config::standard())
            .map_err(|e| PacketError::DecodingError(e.to_string()))?;
        Ok(frame)
    }
}
