use bincode::{Decode, Encode};
use crate::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x07)]
pub struct Heartbeat {
    pub alive: bool
}

impl Heartbeat {
    pub fn new() -> Self {
        Heartbeat {
            alive: true
        }
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Heartbeat {
            alive: true
        }
    }
}
