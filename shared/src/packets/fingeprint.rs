use super::Packet;

#[derive(Debug)]
pub struct Fingerprint {
    pub computer_name_size: u8,
    pub computer_name: String,
    pub username_size: u8,
    pub username: String,
    pub motherboard_uuid: String,
}

impl Fingerprint {
    pub fn new(computer_name: String, username: String, motherboard_uuid: String) -> Self {
        let computer_name_size = computer_name.len() as u8;
        let username_size = username.len() as u8;

        Fingerprint {
            computer_name_size,
            computer_name,
            username_size,
            username,
            motherboard_uuid,
        }
    }
}

impl Packet for Fingerprint {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x03);
        data.push(self.computer_name_size);
        data.extend_from_slice(self.computer_name.as_bytes());
        data.push(self.username_size);
        data.extend_from_slice(self.username.as_bytes());
        data.extend_from_slice(self.motherboard_uuid.as_bytes());
        data
    }

    fn deserialize(data: &[u8]) -> Result<Self, super::packet::Error>
    where
        Self: Sized,
    {
        let computer_name_size = data[0];
        let computer_name = String::from_utf8(data[1..1 + computer_name_size as usize].to_vec())
            .map_err(|_| super::packet::Error::ParseError)?;
        let username_size = data[1 + computer_name_size as usize];
        let username = String::from_utf8(
            data[2 + computer_name_size as usize
                ..2 + computer_name_size as usize + username_size as usize]
                .to_vec(),
        )
        .map_err(|_| super::packet::Error::ParseError)?;
        let motherboard_uuid = String::from_utf8(
            data[2 + computer_name_size as usize + username_size as usize..].to_vec(),
        )
        .map_err(|_| super::packet::Error::ParseError)?;

        Ok(Fingerprint::new(computer_name, username, motherboard_uuid))
    }

    fn packet_code() -> u8 {
        0x03
    }
}
