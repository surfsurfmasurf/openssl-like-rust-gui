use super::CryptoError;

pub struct CertResult {
    pub cert_pem: String,
    pub key_pem: String,
}

pub struct CertInfo {
    pub subject: String,
    pub issuer: String,
    pub serial: String,
    pub not_before: String,
    pub not_after: String,
    pub signature_algorithm: String,
    pub public_key_algorithm: String,
    pub is_ca: bool,
    pub san: Vec<String>,
}

pub fn generate_self_signed(
    common_name: &str,
    organization: &str,
    validity_days: u32,
    san_entries: &[String],
    is_ca: bool,
) -> Result<CertResult, CryptoError> {
    use rcgen::{CertificateParams, DnType, IsCa, KeyPair, SanType};
    use std::time::Duration;

    let mut params = CertificateParams::default();
    params
        .distinguished_name
        .push(DnType::CommonName, common_name);
    params
        .distinguished_name
        .push(DnType::OrganizationName, organization);

    let days = Duration::from_secs(validity_days as u64 * 86400);
    params.not_after = params.not_before + days;

    for san in san_entries {
        let san = san.trim();
        if !san.is_empty() {
            if san.parse::<std::net::IpAddr>().is_ok() {
                params
                    .subject_alt_names
                    .push(SanType::IpAddress(san.parse().unwrap()));
            } else {
                params
                    .subject_alt_names
                    .push(SanType::DnsName(san.try_into().map_err(|e: rcgen::Error| {
                        CryptoError::CertificateError(e.to_string())
                    })?));
            }
        }
    }

    if is_ca {
        params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    }

    let key_pair = KeyPair::generate().map_err(|e| CryptoError::KeyGenError(e.to_string()))?;
    let cert = params
        .self_signed(&key_pair)
        .map_err(|e| CryptoError::CertificateError(e.to_string()))?;

    Ok(CertResult {
        cert_pem: cert.pem(),
        key_pem: key_pair.serialize_pem(),
    })
}

pub fn generate_csr(
    common_name: &str,
    organization: &str,
    san_entries: &[String],
) -> Result<String, CryptoError> {
    use rcgen::{CertificateParams, DnType, KeyPair, SanType};

    let mut params = CertificateParams::default();
    params
        .distinguished_name
        .push(DnType::CommonName, common_name);
    params
        .distinguished_name
        .push(DnType::OrganizationName, organization);

    for san in san_entries {
        let san = san.trim();
        if !san.is_empty() {
            if san.parse::<std::net::IpAddr>().is_ok() {
                params
                    .subject_alt_names
                    .push(SanType::IpAddress(san.parse().unwrap()));
            } else {
                params
                    .subject_alt_names
                    .push(SanType::DnsName(san.try_into().map_err(|e: rcgen::Error| {
                        CryptoError::CertificateError(e.to_string())
                    })?));
            }
        }
    }

    let key_pair = KeyPair::generate().map_err(|e| CryptoError::KeyGenError(e.to_string()))?;
    let csr = params
        .serialize_request(&key_pair)
        .map_err(|e| CryptoError::CertificateError(e.to_string()))?;

    Ok(csr.pem().map_err(|e| CryptoError::CertificateError(e.to_string()))?)
}

pub fn parse_certificate(pem_data: &str) -> Result<CertInfo, CryptoError> {
    let pem = pem::parse(pem_data).map_err(|e| CryptoError::PemError(e.to_string()))?;

    let (_, cert) = x509_parser::parse_x509_certificate(pem.contents())
        .map_err(|e| CryptoError::CertificateError(e.to_string()))?;

    let subject = cert.subject().to_string();
    let issuer = cert.issuer().to_string();
    let serial = cert.raw_serial_as_string();
    let not_before = cert.validity().not_before.to_rfc2822().unwrap_or_default();
    let not_after = cert.validity().not_after.to_rfc2822().unwrap_or_default();
    let sig_algo = format!("{}", cert.signature_algorithm.algorithm);
    let pubkey_algo = format!("{}", cert.public_key().algorithm.algorithm);

    let is_ca = cert
        .basic_constraints()
        .ok()
        .flatten()
        .map(|bc| bc.value.ca)
        .unwrap_or(false);

    let san = cert
        .subject_alternative_name()
        .ok()
        .flatten()
        .map(|ext| {
            ext.value
                .general_names
                .iter()
                .map(|name| format!("{}", name))
                .collect()
        })
        .unwrap_or_default();

    Ok(CertInfo {
        subject,
        issuer,
        serial,
        not_before,
        not_after,
        signature_algorithm: sig_algo,
        public_key_algorithm: pubkey_algo,
        is_ca,
        san,
    })
}
