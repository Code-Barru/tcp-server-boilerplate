use aes_gcm::aead::OsRng;
use shared::{
    encryption::decrypt,
    error::NetworkError,
    packets::{EncryptionRequest, EncryptionResponse, Packet, Packets, from_packet_bytes},
};
use x25519_dalek::{EphemeralSecret, PublicKey};

pub fn perform_handshake(
    reader: &mut impl std::io::Read,
    writer: &mut impl std::io::Write,
) -> Result<[u8; 32], NetworkError> {
    let private_key = EphemeralSecret::random_from_rng(OsRng);
    let public_key = PublicKey::from(&private_key);
    let verify_token: u64 = rand::random();

    let packet = EncryptionRequest::new(public_key.to_bytes(), verify_token);
    let serialized_packet = packet.serialize()?;
    writer.write_all(&serialized_packet)?;

    let mut response_buffer = vec![0; EncryptionResponse::PACKET_SIZE];
    reader.read_exact(&mut response_buffer)?;
    let response = match from_packet_bytes(&response_buffer) {
        Ok(Packets::EncryptionResponse(packet)) => packet,
        Ok(_) => {
            return Err(NetworkError::UnexpectedPacket);
        }
        Err(e) => {
            return Err(NetworkError::PacketError(e));
        }
    };

    let shared_secret = private_key
        .diffie_hellman(&PublicKey::from(response.key))
        .to_bytes();

    let decrypted_token_bytes = decrypt(&shared_secret, &response.nonce, &response.verify_token)?;
    let decrypted_token = u64::from_be_bytes(
        decrypted_token_bytes
            .try_into()
            .map_err(|_| NetworkError::ConvertError)?,
    );

    if decrypted_token != verify_token {
        return Err(NetworkError::TokenDontMatch {
            got: decrypted_token,
            expected: verify_token,
        });
    }

    Ok(shared_secret)
}
