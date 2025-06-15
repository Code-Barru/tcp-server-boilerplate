use aes_gcm::aead::OsRng;
use shared::{
    encryption::encrypt,
    packets::{EncryptionRequest, EncryptionResponse, Packet, Packets, from_packet_bytes},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use x25519_dalek::{EphemeralSecret, PublicKey};

pub async fn perform_handshake(
    reader: &mut tokio::net::tcp::OwnedReadHalf,
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
) -> Result<[u8; 32], std::io::Error> {
    // Getting the encryption request from the client
    let mut encryption_request_buffer = [0u8; EncryptionRequest::PACKET_SIZE];
    reader.read_exact(&mut encryption_request_buffer).await?;

    let encryption_request = match from_packet_bytes(&encryption_request_buffer) {
        Ok(Packets::EncryptionRequest(packet)) => packet,
        Ok(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Expected EncryptionRequest packet",
            ));
        }
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to deserialize packet: {:?}", e),
            ));
        }
    };
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public_secret = PublicKey::from(&secret);
    let shared_secret = secret
        .diffie_hellman(&PublicKey::from(encryption_request.key))
        .to_bytes();
    let (verified_token, nonce) = encrypt(
        &shared_secret,
        &encryption_request.verify_token.to_be_bytes(),
    )
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Encryption failed"))?;

    // Rust shenanigans to conver the Vec<u8> to [u8; 24] and [u8; 12]
    let verified_token_array: [u8; 24] = verified_token.as_slice().try_into().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid verified_token length",
        )
    })?;
    let nonce_array: [u8; 12] = nonce.as_slice().try_into().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid nonce length")
    })?;

    let response =
        EncryptionResponse::new(public_secret.to_bytes(), nonce_array, verified_token_array);
    writer.write(&response.serialize()).await?;

    Ok(shared_secret)
}
