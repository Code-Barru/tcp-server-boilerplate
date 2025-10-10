use bincode::{Decode, Encode};
use derive::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0xFF)]
pub struct FileChunk {
    pub chunk_number: u32,
    pub data: Vec<u8>,
}

impl FileChunk {
    pub fn new(chunk_number: u32, data: Vec<u8>) -> Self {
        FileChunk {
            chunk_number,
            data,
        }
    }
}
