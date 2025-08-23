use bincode::{self, Decode, Encode};
use derive::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x01)]
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