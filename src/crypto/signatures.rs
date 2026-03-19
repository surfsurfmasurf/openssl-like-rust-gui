use super::CryptoError;
use signature::SignatureEncoding;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigAlgorithm {
    RsaSha256,
    EcdsaP256Sha256,
    EcdsaP384Sha384,
}

impl SigAlgorithm {
    pub const ALL: &'static [SigAlgorithm] = &[
        SigAlgorithm::RsaSha256,
        SigAlgorithm::EcdsaP256Sha256,
        SigAlgorithm::EcdsaP384Sha384,
    ];
}

impl fmt::Display for SigAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigAlgorithm::RsaSha256 => write!(f, "RSA-SHA256"),
            SigAlgorithm::EcdsaP256Sha256 => write!(f, "ECDSA-P256-SHA256"),
            SigAlgorithm::EcdsaP384Sha384 => write!(f, "ECDSA-P384-SHA384"),
        }
    }
}

pub fn sign(algo: SigAlgorithm, private_pem: &str, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    match algo {
        SigAlgorithm::RsaSha256 => sign_rsa(private_pem, data),
        SigAlgorithm::EcdsaP256Sha256 => sign_ecdsa_p256(private_pem, data),
        SigAlgorithm::EcdsaP384Sha384 => sign_ecdsa_p384(private_pem, data),
    }
}

pub fn verify(
    algo: SigAlgorithm,
    public_pem: &str,
    data: &[u8],
    signature: &[u8],
) -> Result<bool, CryptoError> {
    match algo {
        SigAlgorithm::RsaSha256 => verify_rsa(public_pem, data, signature),
        SigAlgorithm::EcdsaP256Sha256 => verify_ecdsa_p256(public_pem, data, signature),
        SigAlgorithm::EcdsaP384Sha384 => verify_ecdsa_p384(public_pem, data, signature),
    }
}

fn sign_rsa(private_pem: &str, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    use rsa::pkcs1v15::SigningKey;
    use rsa::pkcs8::DecodePrivateKey;
    use rsa::signature::Signer;
    use rsa::RsaPrivateKey;
    use sha2::Sha256;

    let private_key = RsaPrivateKey::from_pkcs8_pem(private_pem)
        .map_err(|e| CryptoError::PemError(e.to_string()))?;

    let signing_key = SigningKey::<Sha256>::new(private_key);
    let sig = Signer::sign(&signing_key, data);
    Ok(sig.to_vec())
}

fn verify_rsa(public_pem: &str, data: &[u8], signature: &[u8]) -> Result<bool, CryptoError> {
    use rsa::pkcs1v15::{Signature, VerifyingKey};
    use rsa::pkcs8::DecodePublicKey;
    use rsa::signature::Verifier;
    use rsa::RsaPublicKey;
    use sha2::Sha256;

    let public_key = RsaPublicKey::from_public_key_pem(public_pem)
        .map_err(|e| CryptoError::PemError(e.to_string()))?;

    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    let sig = Signature::try_from(signature)
        .map_err(|e| CryptoError::SignatureError(e.to_string()))?;

    match verifying_key.verify(data, &sig) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

fn sign_ecdsa_p256(private_pem: &str, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    use ecdsa::signature::Signer;
    use p256::ecdsa::{Signature, SigningKey};
    use p256::pkcs8::DecodePrivateKey;

    let signing_key = SigningKey::from_pkcs8_pem(private_pem)
        .map_err(|e| CryptoError::PemError(e.to_string()))?;

    let sig: Signature = signing_key.sign(data);
    Ok(sig.to_vec())
}

fn verify_ecdsa_p256(
    public_pem: &str,
    data: &[u8],
    signature: &[u8],
) -> Result<bool, CryptoError> {
    use ecdsa::signature::Verifier;
    use p256::ecdsa::{Signature, VerifyingKey};
    use p256::pkcs8::DecodePublicKey;

    let verifying_key = VerifyingKey::from_public_key_pem(public_pem)
        .map_err(|e| CryptoError::PemError(e.to_string()))?;

    let sig = Signature::from_slice(signature)
        .map_err(|e| CryptoError::SignatureError(e.to_string()))?;

    match verifying_key.verify(data, &sig) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

fn sign_ecdsa_p384(private_pem: &str, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    use ecdsa::signature::Signer;
    use p384::ecdsa::{Signature, SigningKey};
    use p384::pkcs8::DecodePrivateKey;

    let signing_key = SigningKey::from_pkcs8_pem(private_pem)
        .map_err(|e| CryptoError::PemError(e.to_string()))?;

    let sig: Signature = signing_key.sign(data);
    Ok(sig.to_vec())
}

fn verify_ecdsa_p384(
    public_pem: &str,
    data: &[u8],
    signature: &[u8],
) -> Result<bool, CryptoError> {
    use ecdsa::signature::Verifier;
    use p384::ecdsa::{Signature, VerifyingKey};
    use p384::pkcs8::DecodePublicKey;

    let verifying_key = VerifyingKey::from_public_key_pem(public_pem)
        .map_err(|e| CryptoError::PemError(e.to_string()))?;

    let sig = Signature::from_slice(signature)
        .map_err(|e| CryptoError::SignatureError(e.to_string()))?;

    match verifying_key.verify(data, &sig) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}
