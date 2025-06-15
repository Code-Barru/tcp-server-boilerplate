use super::packet::{Error, Packet};

#[derive(Debug)]
pub struct EncryptionRequest {
    pub key: [u8; 32],
    pub verify_token: u64,
}

impl EncryptionRequest {
    pub const PACKET_SIZE: usize = 41;
    pub fn new(key: [u8; 32], verify_token: u64) -> Self {
        EncryptionRequest { key, verify_token }
    }
}

impl Packet for EncryptionRequest {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x01);
        data.extend_from_slice(&self.key);
        data.extend_from_slice(&self.verify_token.to_be_bytes());
        data
    }

    fn deserialize(data: &[u8]) -> Result<Self, super::packet::Error>
    where
        Self: Sized,
    {
        if data.len() != EncryptionRequest::PACKET_SIZE - 1 {
            return Err(Error::InvalidData);
        }

        let key: [u8; 32] = data
            .get(..32)
            .ok_or(Error::ParseError)?
            .try_into()
            .map_err(|_| Error::ParseError)?;

        let verify_token = u64::from_be_bytes(
            data.get(32..40)
                .ok_or(Error::ParseError)?
                .try_into()
                .map_err(|_| Error::ParseError)?,
        );

        Ok(EncryptionRequest::new(key, verify_token))
    }

    fn packet_code() -> u8 {
        0x01
    }
}
