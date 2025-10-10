use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("Got unknown packet code: {0}")]
    UnknownPacket(String),

    #[error("Error while encoding packet: {0}")]
    EncodingError(String),

    #[error("Error while decoding packet: {0}")]
    DecodingError(String),
}
