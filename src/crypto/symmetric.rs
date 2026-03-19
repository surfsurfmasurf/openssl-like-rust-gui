use super::CryptoError;
use aes_gcm::{aead::Aead, Aes128Gcm, Aes256Gcm, KeyInit, Nonce as GcmNonce};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymAlgorithm {
    Aes128,
    Aes192,
    Aes256,
    Des,
    TripleDes,
}

impl SymAlgorithm {
    pub const ALL: &'static [SymAlgorithm] = &[
        SymAlgorithm::Aes128,
        SymAlgorithm::Aes192,
        SymAlgorithm::Aes256,
        SymAlgorithm::Des,
        SymAlgorithm::TripleDes,
    ];

    pub fn key_size(&self) -> usize {
        match self {
            SymAlgorithm::Aes128 => 16,
            SymAlgorithm::Aes192 => 24,
            SymAlgorithm::Aes256 => 32,
            SymAlgorithm::Des => 8,
            SymAlgorithm::TripleDes => 24,
        }
    }

    pub fn iv_size(&self, mode: CipherMode) -> usize {
        match mode {
            CipherMode::Gcm => 12,
            CipherMode::Cbc => match self {
                SymAlgorithm::Des | SymAlgorithm::TripleDes => 8,
                _ => 16,
            },
        }
    }
}

impl fmt::Display for SymAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymAlgorithm::Aes128 => write!(f, "AES-128"),
            SymAlgorithm::Aes192 => write!(f, "AES-192"),
            SymAlgorithm::Aes256 => write!(f, "AES-256"),
            SymAlgorithm::Des => write!(f, "DES"),
            SymAlgorithm::TripleDes => write!(f, "3DES"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherMode {
    Cbc,
    Gcm,
}

impl CipherMode {
    pub const ALL: &'static [CipherMode] = &[CipherMode::Cbc, CipherMode::Gcm];
}

impl fmt::Display for CipherMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherMode::Cbc => write!(f, "CBC"),
            CipherMode::Gcm => write!(f, "GCM"),
        }
    }
}

pub fn encrypt(
    algo: SymAlgorithm,
    mode: CipherMode,
    key: &[u8],
    iv: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    match mode {
        CipherMode::Gcm => encrypt_gcm(algo, key, iv, plaintext),
        CipherMode::Cbc => encrypt_cbc(algo, key, iv, plaintext),
    }
}

pub fn decrypt(
    algo: SymAlgorithm,
    mode: CipherMode,
    key: &[u8],
    iv: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    match mode {
        CipherMode::Gcm => decrypt_gcm(algo, key, iv, ciphertext),
        CipherMode::Cbc => decrypt_cbc(algo, key, iv, ciphertext),
    }
}

fn encrypt_gcm(
    algo: SymAlgorithm,
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let nonce = GcmNonce::from_slice(nonce);
    match algo {
        SymAlgorithm::Aes128 => {
            let cipher = Aes128Gcm::new_from_slice(key)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
            cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))
        }
        SymAlgorithm::Aes256 => {
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
            cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))
        }
        _ => Err(CryptoError::EncryptionFailed(format!(
            "GCM mode not supported for {}",
            algo
        ))),
    }
}

fn decrypt_gcm(
    algo: SymAlgorithm,
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let nonce = GcmNonce::from_slice(nonce);
    match algo {
        SymAlgorithm::Aes128 => {
            let cipher = Aes128Gcm::new_from_slice(key)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
            cipher
                .decrypt(nonce, ciphertext)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
        }
        SymAlgorithm::Aes256 => {
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
            cipher
                .decrypt(nonce, ciphertext)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
        }
        _ => Err(CryptoError::DecryptionFailed(format!(
            "GCM mode not supported for {}",
            algo
        ))),
    }
}

fn encrypt_cbc(
    algo: SymAlgorithm,
    key: &[u8],
    iv: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    use cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};

    match algo {
        SymAlgorithm::Aes128 => {
            type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
            let enc = Aes128CbcEnc::new_from_slices(key, iv)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
            Ok(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
        }
        SymAlgorithm::Aes192 => {
            type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;
            let enc = Aes192CbcEnc::new_from_slices(key, iv)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
            Ok(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
        }
        SymAlgorithm::Aes256 => {
            type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
            let enc = Aes256CbcEnc::new_from_slices(key, iv)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
            Ok(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
        }
        SymAlgorithm::Des => {
            type DesCbcEnc = cbc::Encryptor<des::Des>;
            let enc = DesCbcEnc::new_from_slices(key, iv)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
            Ok(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
        }
        SymAlgorithm::TripleDes => {
            type TdesCbcEnc = cbc::Encryptor<des::TdesEde3>;
            let enc = TdesCbcEnc::new_from_slices(key, iv)
                .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
            Ok(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
        }
    }
}

fn decrypt_cbc(
    algo: SymAlgorithm,
    key: &[u8],
    iv: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    use cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};

    match algo {
        SymAlgorithm::Aes128 => {
            type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
            let dec = Aes128CbcDec::new_from_slices(key, iv)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
            dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
        }
        SymAlgorithm::Aes192 => {
            type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;
            let dec = Aes192CbcDec::new_from_slices(key, iv)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
            dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
        }
        SymAlgorithm::Aes256 => {
            type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
            let dec = Aes256CbcDec::new_from_slices(key, iv)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
            dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
        }
        SymAlgorithm::Des => {
            type DesCbcDec = cbc::Decryptor<des::Des>;
            let dec = DesCbcDec::new_from_slices(key, iv)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
            dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
        }
        SymAlgorithm::TripleDes => {
            type TdesCbcDec = cbc::Decryptor<des::TdesEde3>;
            let dec = TdesCbcDec::new_from_slices(key, iv)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
            dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
        }
    }
}

pub fn generate_key(algo: SymAlgorithm) -> Vec<u8> {
    use rand::RngCore;
    let mut key = vec![0u8; algo.key_size()];
    rand::rngs::OsRng.fill_bytes(&mut key);
    key
}

pub fn generate_iv(algo: SymAlgorithm, mode: CipherMode) -> Vec<u8> {
    use rand::RngCore;
    let mut iv = vec![0u8; algo.iv_size(mode)];
    rand::rngs::OsRng.fill_bytes(&mut iv);
    iv
}
