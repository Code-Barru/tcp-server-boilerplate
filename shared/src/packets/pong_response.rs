use bincode::{Decode, Encode};
use derive::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x04)]
pub struct PongResponse {
    pub timestamp: u64,
}

impl PongResponse {
    pub fn new(timestamp: u64) -> Self {
        PongResponse { timestamp }
    }
}
