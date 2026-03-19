use super::CryptoError;
use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct TlsInfo {
    pub protocol_version: String,
    pub cipher_suite: String,
    pub server_cert_subject: String,
    pub server_cert_issuer: String,
    pub server_cert_not_before: String,
    pub server_cert_not_after: String,
    pub server_cert_serial: String,
    pub server_cert_san: Vec<String>,
    pub cert_chain_length: usize,
    pub peer_address: String,
    pub server_cert_pem: String,
}

impl fmt::Display for TlsInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- Connection Info ---")?;
        writeln!(f, "Peer:             {}", self.peer_address)?;
        writeln!(f, "Protocol:         {}", self.protocol_version)?;
        writeln!(f, "Cipher Suite:     {}", self.cipher_suite)?;
        writeln!(f, "Chain Length:     {}", self.cert_chain_length)?;
        writeln!(f)?;
        writeln!(f, "--- Server Certificate ---")?;
        writeln!(f, "Subject:          {}", self.server_cert_subject)?;
        writeln!(f, "Issuer:           {}", self.server_cert_issuer)?;
        writeln!(f, "Serial:           {}", self.server_cert_serial)?;
        writeln!(f, "Not Before:       {}", self.server_cert_not_before)?;
        writeln!(f, "Not After:        {}", self.server_cert_not_after)?;
        if !self.server_cert_san.is_empty() {
            writeln!(f, "SANs:             {}", self.server_cert_san.join(", "))?;
        }
        Ok(())
    }
}

pub fn connect_tls(host: &str, port: u16) -> Result<TlsInfo, CryptoError> {
    use rustls::pki_types::ServerName;

    let addr = format!("{}:{}", host, port);

    // Build TLS config that trusts system roots
    let root_store = rustls::RootCertStore::from_iter(
        webpki_roots::TLS_SERVER_ROOTS.iter().cloned(),
    );

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let server_name = ServerName::try_from(host.to_string())
        .map_err(|e| CryptoError::CertificateError(format!("Invalid hostname: {}", e)))?;

    let mut conn = rustls::ClientConnection::new(
        std::sync::Arc::new(config),
        server_name,
    )
    .map_err(|e| CryptoError::CertificateError(e.to_string()))?;

    let mut tcp = TcpStream::connect(&addr)
        .map_err(|e| CryptoError::IoError(e))?;
    tcp.set_read_timeout(Some(Duration::from_secs(10)))
        .map_err(|e| CryptoError::IoError(e))?;

    let mut tls = rustls::Stream::new(&mut conn, &mut tcp);

    // Trigger handshake by writing empty or reading
    let _ = tls.write(b"");
    // Read a tiny bit to complete handshake (ignore errors like timeout)
    let mut buf = [0u8; 1];
    let _ = tls.read(&mut buf);

    let protocol_version = conn
        .protocol_version()
        .map(|v| format!("{:?}", v))
        .unwrap_or_else(|| "Unknown".into());

    let cipher_suite = conn
        .negotiated_cipher_suite()
        .map(|cs| format!("{:?}", cs.suite()))
        .unwrap_or_else(|| "Unknown".into());

    let peer_certs = conn.peer_certificates().unwrap_or(&[]);
    let cert_chain_length = peer_certs.len();

    let (subject, issuer, serial, not_before, not_after, san, cert_pem) = if let Some(cert_der) = peer_certs.first() {
        let pem_encoded = pem::encode(&pem::Pem::new("CERTIFICATE", cert_der.as_ref().to_vec()));

        match x509_parser::parse_x509_certificate(cert_der.as_ref()) {
            Ok((_, cert)) => {
                let subject = cert.subject().to_string();
                let issuer = cert.issuer().to_string();
                let serial = cert.raw_serial_as_string();
                let not_before = cert.validity().not_before.to_rfc2822().unwrap_or_default();
                let not_after = cert.validity().not_after.to_rfc2822().unwrap_or_default();
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
                (subject, issuer, serial, not_before, not_after, san, pem_encoded)
            }
            Err(e) => {
                let msg = format!("(parse error: {})", e);
                (msg.clone(), msg.clone(), msg.clone(), msg.clone(), msg, vec![], pem_encoded)
            }
        }
    } else {
        return Err(CryptoError::CertificateError("No peer certificates received".into()));
    };

    Ok(TlsInfo {
        protocol_version,
        cipher_suite,
        server_cert_subject: subject,
        server_cert_issuer: issuer,
        server_cert_not_before: not_before,
        server_cert_not_after: not_after,
        server_cert_serial: serial,
        server_cert_san: san,
        cert_chain_length,
        peer_address: addr,
        server_cert_pem: cert_pem,
    })
}
