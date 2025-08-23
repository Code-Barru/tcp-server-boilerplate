use bincode::{Decode, Encode};
use derive::Packet;

/// Packet sent to open a new stream
#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x03)]
pub struct StreamOpen {
    pub stream_id: u32,
}

/// Packet sent to close an existing stream
#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x04)]
pub struct StreamClose {
    pub stream_id: u32,
}

/// Packet containing data for a specific stream
#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x05)]
pub struct StreamData {
    pub stream_id: u32,
    pub data: Vec<u8>,
}

/// Packet indicating an error on a specific stream
#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x06)]
pub struct StreamError {
    pub stream_id: u32,
    pub error: String,
}
