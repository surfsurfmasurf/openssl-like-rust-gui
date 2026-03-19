pub mod asymmetric;
pub mod certificates;
pub mod encoding;
pub mod file_ops;
pub mod hashing;
pub mod key_inspect;
pub mod random;
pub mod signatures;
pub mod symmetric;
pub mod tls;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid key length: expected {expected}, got {got}")]
    InvalidKeyLength { expected: usize, got: usize },

    #[error("Invalid hex input: {0}")]
    HexDecode(#[from] hex::FromHexError),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Invalid PEM data: {0}")]
    PemError(String),

    #[error("Certificate error: {0}")]
    CertificateError(String),

    #[error("Key generation failed: {0}")]
    KeyGenError(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("Base64 decode error: {0}")]
    Base64Error(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
