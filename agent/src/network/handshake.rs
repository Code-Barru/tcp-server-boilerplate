use aes_gcm::aead::OsRng;
use shared::{
    encryption::decrypt,
    packets::{EncryptionRequest, EncryptionResponse, Packet, Packets, from_packet_bytes},
};
use x25519_dalek::{EphemeralSecret, PublicKey};

pub fn perform_handshake(
    reader: &mut impl std::io::Read,
    writer: &mut impl std::io::Write,
) -> std::io::Result<[u8; 32]> {
    let private_key = EphemeralSecret::random_from_rng(OsRng);
    let public_key = PublicKey::from(&private_key);
    let verify_token: u64 = rand::random();

    let packet = EncryptionRequest::new(public_key.to_bytes(), verify_token);
    writer.write(&packet.serialize())?;

    let mut response_buffer = vec![0; EncryptionResponse::PACKET_SIZE];
    reader.read_exact(&mut response_buffer)?;
    let response = match from_packet_bytes(&response_buffer) {
        Ok(Packets::EncryptionResponse(packet)) => packet,
        Ok(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Expected EncryptionResponse packet",
            ));
        }
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to deserialize packet: {:?}", e),
            ));
        }
    };

    let shared_secret = private_key
        .diffie_hellman(&PublicKey::from(response.key))
        .to_bytes();

    let decrypted_token = decrypt(&shared_secret, &response.nonce, &response.verify_token)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Decryption failed"))?;

    if decrypted_token != verify_token.to_be_bytes() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Decrypted token does not match the original verify token",
        ));
    }

    Ok(shared_secret)
}
