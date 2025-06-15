use super::{Error, Packet};

#[derive(Debug)]
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

impl Packet for EncryptionResponse {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x02);
        data.extend_from_slice(&self.key);
        data.extend_from_slice(&self.nonce);
        data.extend_from_slice(&self.verify_token);
        data
    }

    fn deserialize(data: &[u8]) -> Result<Self, super::packet::Error>
    where
        Self: Sized,
    {
        if data.len() != EncryptionResponse::PACKET_SIZE - 1 {
            return Err(Error::InvalidData);
        }

        let key = data
            .get(..32)
            .ok_or(Error::ParseError)?
            .try_into()
            .map_err(|_| Error::ParseError)?;

        let nonce = data
            .get(32..44)
            .ok_or(Error::ParseError)?
            .try_into()
            .map_err(|_| Error::ParseError)?;

        let verify_token = data
            .get(44..68)
            .ok_or(Error::ParseError)?
            .try_into()
            .map_err(|_| Error::ParseError)?;

        Ok(EncryptionResponse {
            key,
            nonce,
            verify_token,
        })
    }

    fn packet_code() -> u8 {
        0x02
    }
}
