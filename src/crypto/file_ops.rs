use super::CryptoError;
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn encrypt_file(input_path: &Path, output_path: &Path, password: &str) -> Result<(), CryptoError> {
    let data = std::fs::read(input_path)?;

    // Derive key from password using SHA-256 (simple KDF)
    let mut salt = [0u8; 16];
    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut salt);

    let key = derive_key(password, &salt);
    let mut nonce_bytes = [0u8; 12];
    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, data.as_ref())
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    // Output format: [salt(16)][nonce(12)][ciphertext]
    let mut output = Vec::with_capacity(16 + 12 + ciphertext.len());
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);

    std::fs::write(output_path, &output)?;
    Ok(())
}

pub fn decrypt_file(input_path: &Path, output_path: &Path, password: &str) -> Result<(), CryptoError> {
    let data = std::fs::read(input_path)?;

    if data.len() < 28 {
        return Err(CryptoError::DecryptionFailed(
            "File too small to contain valid encrypted data".into(),
        ));
    }

    let salt = &data[..16];
    let nonce_bytes = &data[16..28];
    let ciphertext = &data[28..];

    let key = derive_key(password, salt);
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

    std::fs::write(output_path, &plaintext)?;
    Ok(())
}

fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    // Simple PBKDF: SHA256(salt || password) iterated
    let mut key = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    key.copy_from_slice(&result);

    // Iterate to strengthen
    for _ in 0..10000 {
        let mut hasher = Sha256::new();
        hasher.update(&key);
        hasher.update(salt);
        let result = hasher.finalize();
        key.copy_from_slice(&result);
    }

    key
}
