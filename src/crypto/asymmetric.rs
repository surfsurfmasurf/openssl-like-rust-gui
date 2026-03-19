use super::CryptoError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsymAlgorithm {
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
}

impl AsymAlgorithm {
    pub const ALL: &'static [AsymAlgorithm] = &[
        AsymAlgorithm::Rsa2048,
        AsymAlgorithm::Rsa4096,
        AsymAlgorithm::EcdsaP256,
        AsymAlgorithm::EcdsaP384,
    ];
}

impl fmt::Display for AsymAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsymAlgorithm::Rsa2048 => write!(f, "RSA-2048"),
            AsymAlgorithm::Rsa4096 => write!(f, "RSA-4096"),
            AsymAlgorithm::EcdsaP256 => write!(f, "ECDSA P-256"),
            AsymAlgorithm::EcdsaP384 => write!(f, "ECDSA P-384"),
        }
    }
}

pub struct KeyPair {
    pub private_pem: String,
    pub public_pem: String,
}

pub fn generate_keypair(algo: AsymAlgorithm) -> Result<KeyPair, CryptoError> {
    match algo {
        AsymAlgorithm::Rsa2048 => generate_rsa(2048),
        AsymAlgorithm::Rsa4096 => generate_rsa(4096),
        AsymAlgorithm::EcdsaP256 => generate_ecdsa_p256(),
        AsymAlgorithm::EcdsaP384 => generate_ecdsa_p384(),
    }
}

fn generate_rsa(bits: usize) -> Result<KeyPair, CryptoError> {
    use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
    use rsa::RsaPrivateKey;

    let mut rng = rand::rngs::OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| CryptoError::KeyGenError(e.to_string()))?;

    let private_pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| CryptoError::KeyGenError(e.to_string()))?
        .to_string();

    let public_pem = private_key
        .to_public_key()
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| CryptoError::KeyGenError(e.to_string()))?;

    Ok(KeyPair {
        private_pem,
        public_pem,
    })
}

fn generate_ecdsa_p256() -> Result<KeyPair, CryptoError> {
    use p256::ecdsa::SigningKey;
    use p256::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};

    let signing_key = SigningKey::random(&mut rand::rngs::OsRng);

    let private_pem = signing_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| CryptoError::KeyGenError(e.to_string()))?
        .to_string();

    let verifying_key = signing_key.verifying_key();
    let public_pem = verifying_key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| CryptoError::KeyGenError(e.to_string()))?;

    Ok(KeyPair {
        private_pem,
        public_pem,
    })
}

fn generate_ecdsa_p384() -> Result<KeyPair, CryptoError> {
    use p384::ecdsa::SigningKey;
    use p384::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};

    let signing_key = SigningKey::random(&mut rand::rngs::OsRng);

    let private_pem = signing_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| CryptoError::KeyGenError(e.to_string()))?
        .to_string();

    let verifying_key = signing_key.verifying_key();
    let public_pem = verifying_key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| CryptoError::KeyGenError(e.to_string()))?;

    Ok(KeyPair {
        private_pem,
        public_pem,
    })
}
