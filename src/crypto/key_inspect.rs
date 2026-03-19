use super::CryptoError;
use rsa::traits::PublicKeyParts;

pub struct KeyInfo {
    pub key_type: String,
    pub bit_length: String,
    pub details: Vec<(String, String)>,
}

pub fn inspect_pem(pem_data: &str) -> Result<KeyInfo, CryptoError> {
    let pem_data = pem_data.trim();

    if pem_data.contains("RSA PRIVATE KEY") {
        inspect_rsa_private_key(pem_data)
    } else if pem_data.contains("PRIVATE KEY") {
        inspect_private_key(pem_data)
    } else if pem_data.contains("PUBLIC KEY") {
        inspect_public_key(pem_data)
    } else if pem_data.contains("CERTIFICATE") {
        inspect_certificate_key(pem_data)
    } else {
        Err(CryptoError::PemError(
            "Unrecognized PEM format. Expected PRIVATE KEY, PUBLIC KEY, RSA PRIVATE KEY, or CERTIFICATE".into(),
        ))
    }
}

fn inspect_private_key(pem_data: &str) -> Result<KeyInfo, CryptoError> {
    // Try RSA first
    {
        use rsa::pkcs8::DecodePrivateKey;
        use rsa::RsaPrivateKey;

        if let Ok(key) = RsaPrivateKey::from_pkcs8_pem(pem_data) {
            let bits = key.size() * 8;
            let pub_key = key.to_public_key();
            let e = pub_key.e();
            let n_bytes = pub_key.n().to_bytes_be();

            return Ok(KeyInfo {
                key_type: "RSA Private Key (PKCS#8)".into(),
                bit_length: format!("{} bits", bits),
                details: vec![
                    ("Modulus (n)".into(), format!("{}... ({} bytes)", hex_preview(&n_bytes, 32), n_bytes.len())),
                    ("Public Exponent (e)".into(), format!("{}", e)),
                    ("Key Size".into(), format!("{} bits", bits)),
                ],
            });
        }
    }

    // Try ECDSA P-256
    {
        use p256::pkcs8::DecodePrivateKey;
        if let Ok(_key) = p256::ecdsa::SigningKey::from_pkcs8_pem(pem_data) {
            return Ok(KeyInfo {
                key_type: "ECDSA Private Key (PKCS#8)".into(),
                bit_length: "256 bits".into(),
                details: vec![
                    ("Curve".into(), "P-256 (secp256r1 / prime256v1)".into()),
                    ("Key Size".into(), "256 bits".into()),
                ],
            });
        }
    }

    // Try ECDSA P-384
    {
        use p384::pkcs8::DecodePrivateKey;
        if let Ok(_key) = p384::ecdsa::SigningKey::from_pkcs8_pem(pem_data) {
            return Ok(KeyInfo {
                key_type: "ECDSA Private Key (PKCS#8)".into(),
                bit_length: "384 bits".into(),
                details: vec![
                    ("Curve".into(), "P-384 (secp384r1)".into()),
                    ("Key Size".into(), "384 bits".into()),
                ],
            });
        }
    }

    Err(CryptoError::PemError("Could not parse private key. Supported: RSA, ECDSA P-256, ECDSA P-384".into()))
}

fn inspect_public_key(pem_data: &str) -> Result<KeyInfo, CryptoError> {
    // Try RSA
    {
        use rsa::pkcs8::DecodePublicKey;
        use rsa::RsaPublicKey;

        if let Ok(key) = RsaPublicKey::from_public_key_pem(pem_data) {
            let bits = key.size() * 8;
            let e = key.e();
            let n_bytes = key.n().to_bytes_be();

            return Ok(KeyInfo {
                key_type: "RSA Public Key".into(),
                bit_length: format!("{} bits", bits),
                details: vec![
                    ("Modulus (n)".into(), format!("{}... ({} bytes)", hex_preview(&n_bytes, 32), n_bytes.len())),
                    ("Public Exponent (e)".into(), format!("{}", e)),
                    ("Key Size".into(), format!("{} bits", bits)),
                ],
            });
        }
    }

    // Try ECDSA P-256
    {
        use p256::pkcs8::DecodePublicKey;
        if let Ok(_key) = p256::ecdsa::VerifyingKey::from_public_key_pem(pem_data) {
            return Ok(KeyInfo {
                key_type: "ECDSA Public Key".into(),
                bit_length: "256 bits".into(),
                details: vec![
                    ("Curve".into(), "P-256 (secp256r1 / prime256v1)".into()),
                    ("Key Size".into(), "256 bits".into()),
                ],
            });
        }
    }

    // Try ECDSA P-384
    {
        use p384::pkcs8::DecodePublicKey;
        if let Ok(_key) = p384::ecdsa::VerifyingKey::from_public_key_pem(pem_data) {
            return Ok(KeyInfo {
                key_type: "ECDSA Public Key".into(),
                bit_length: "384 bits".into(),
                details: vec![
                    ("Curve".into(), "P-384 (secp384r1)".into()),
                    ("Key Size".into(), "384 bits".into()),
                ],
            });
        }
    }

    Err(CryptoError::PemError("Could not parse public key. Supported: RSA, ECDSA P-256, ECDSA P-384".into()))
}

fn inspect_rsa_private_key(pem_data: &str) -> Result<KeyInfo, CryptoError> {
    use rsa::pkcs1::DecodeRsaPrivateKey;
    use rsa::RsaPrivateKey;

    let key = RsaPrivateKey::from_pkcs1_pem(pem_data)
        .map_err(|e| CryptoError::PemError(e.to_string()))?;

    let bits = key.size() * 8;
    let pub_key = key.to_public_key();
    let e = pub_key.e();
    let n_bytes = pub_key.n().to_bytes_be();

    Ok(KeyInfo {
        key_type: "RSA Private Key (PKCS#1)".into(),
        bit_length: format!("{} bits", bits),
        details: vec![
            ("Modulus (n)".into(), format!("{}... ({} bytes)", hex_preview(&n_bytes, 32), n_bytes.len())),
            ("Public Exponent (e)".into(), format!("{}", e)),
            ("Key Size".into(), format!("{} bits", bits)),
        ],
    })
}

fn inspect_certificate_key(pem_data: &str) -> Result<KeyInfo, CryptoError> {
    let parsed = pem::parse(pem_data).map_err(|e| CryptoError::PemError(e.to_string()))?;
    let (_, cert) = x509_parser::parse_x509_certificate(parsed.contents())
        .map_err(|e| CryptoError::CertificateError(e.to_string()))?;

    let pubkey = cert.public_key();
    let algo = format!("{}", pubkey.algorithm.algorithm);
    let key_bits = pubkey.raw.len() * 8;

    let mut details = vec![
        ("Subject".into(), cert.subject().to_string()),
        ("Issuer".into(), cert.issuer().to_string()),
        ("Public Key Algorithm".into(), algo),
        ("Public Key Size".into(), format!("~{} bits (raw)", key_bits)),
        ("Signature Algorithm".into(), format!("{}", cert.signature_algorithm.algorithm)),
    ];

    let is_ca = cert.basic_constraints().ok().flatten().map(|bc| bc.value.ca).unwrap_or(false);
    details.push(("Is CA".into(), format!("{}", is_ca)));

    Ok(KeyInfo {
        key_type: "X.509 Certificate".into(),
        bit_length: format!("~{} bits", key_bits),
        details,
    })
}

fn hex_preview(bytes: &[u8], max: usize) -> String {
    let show = if bytes.len() > max { &bytes[..max] } else { bytes };
    hex::encode(show)
}
