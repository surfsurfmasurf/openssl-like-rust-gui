use super::CryptoError;
use base64::{engine::general_purpose::STANDARD, Engine};

pub fn base64_encode(data: &[u8]) -> String {
    STANDARD.encode(data)
}

pub fn base64_decode(encoded: &str) -> Result<Vec<u8>, CryptoError> {
    STANDARD
        .decode(encoded.trim())
        .map_err(|e| CryptoError::Base64Error(e.to_string()))
}
