mod client;
mod connection;
mod handshake;
mod stream;
mod multiplex;

pub use client::Client;
pub use multiplex::MultiplexManager;
pub(crate) use connection::{Connection, ReadHalf, WriteHalf};
use handshake::perform_handshake;
