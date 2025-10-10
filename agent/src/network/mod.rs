mod client;
mod connection;
mod error;
mod handshake;
mod handler;

pub use client::Client;
use connection::{Connection, ReadHalf, WriteHalf};
use handshake::perform_handshake;
pub use handler::dispatch_frame;
