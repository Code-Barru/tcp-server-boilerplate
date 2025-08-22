use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use rand::Rng;
use thiserror::Error;

pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), EncryptionError> {
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(key));
    let nonce: [u8; 12] = rand::rng().random();
    let ciphertext = match cipher.encrypt(Nonce::from_slice(&nonce), plaintext) {
        Ok(ct) => ct,
        Err(e) => return Err(EncryptionError::FailedToEncrypt(e.to_string()))
    };
    Ok((ciphertext, nonce.to_vec()))
}

pub fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(key));
    match cipher.decrypt(Nonce::from_slice(nonce), ciphertext) {
        Ok(decrypted) => Ok(decrypted),
        Err(e) => Err(EncryptionError::FailedToDecrypt(e.to_string())),
    }
}

#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Failed to encrypt: {0}")]
    FailedToEncrypt(String),
    #[error("Failed to decrypt: {0}")]
    FailedToDecrypt(String)
}
