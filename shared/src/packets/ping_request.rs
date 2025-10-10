use bincode::{Decode, Encode};
use derive::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x03)]
pub struct PingRequest {
    pub timestamp: u64,
}

impl PingRequest {
    pub fn new(timestamp: u64) -> Self {
        PingRequest { timestamp }
    }
}
