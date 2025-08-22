mod client;
mod connection;
mod handshake;
mod error;

pub use client::Client;
use connection::{Connection, ReadHalf, WriteHalf};
use handshake::perform_handshake;
