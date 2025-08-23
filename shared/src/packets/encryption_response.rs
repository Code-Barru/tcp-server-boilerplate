use bincode::{Decode, Encode};
use derive::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x02)]
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
