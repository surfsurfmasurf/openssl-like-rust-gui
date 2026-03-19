use crate::theme;
use iced::widget::{column, container, scrollable, text, Column};
use iced::{Background, Border, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Msg {}

pub struct State;

impl Default for State {
    fn default() -> Self { State }
}

pub fn view(_state: &State) -> Element<'static, Msg> {
    let title = text("Supported Algorithms").size(24).color(theme::TEXT_DARK);
    let desc = text("List of all cryptographic algorithms available in this tool (like openssl list -cipher-algorithms)")
        .size(13).color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let hash_section = algo_section("Hash Functions", &[
        ("MD5", "128-bit digest, legacy (NOT secure for crypto)"),
        ("SHA-1", "160-bit digest, legacy (NOT secure for crypto)"),
        ("SHA-256", "256-bit digest, SHA-2 family"),
        ("SHA-384", "384-bit digest, SHA-2 family"),
        ("SHA-512", "512-bit digest, SHA-2 family"),
    ]);

    let sym_section = algo_section("Symmetric Ciphers", &[
        ("AES-128-CBC", "128-bit key, CBC mode, PKCS#7 padding"),
        ("AES-192-CBC", "192-bit key, CBC mode, PKCS#7 padding"),
        ("AES-256-CBC", "256-bit key, CBC mode, PKCS#7 padding"),
        ("AES-128-GCM", "128-bit key, GCM authenticated encryption"),
        ("AES-256-GCM", "256-bit key, GCM authenticated encryption"),
        ("DES-CBC", "56-bit key, legacy (NOT secure)"),
        ("3DES-CBC (DES-EDE3)", "168-bit key, legacy"),
    ]);

    let asym_section = algo_section("Asymmetric Algorithms", &[
        ("RSA-2048", "2048-bit RSA key pair generation"),
        ("RSA-4096", "4096-bit RSA key pair generation"),
        ("ECDSA P-256", "secp256r1 / prime256v1 curve"),
        ("ECDSA P-384", "secp384r1 curve"),
    ]);

    let sig_section = algo_section("Signature Algorithms", &[
        ("RSA-PKCS1v15-SHA256", "RSA signature with SHA-256"),
        ("ECDSA-P256-SHA256", "ECDSA with P-256 curve and SHA-256"),
        ("ECDSA-P384-SHA384", "ECDSA with P-384 curve and SHA-384"),
    ]);

    let cert_section = algo_section("Certificate Operations", &[
        ("X.509 Self-Signed", "Generate self-signed certificates via rcgen"),
        ("CSR Generation", "PKCS#10 Certificate Signing Request"),
        ("Certificate Parsing", "View X.509 certificate details via x509-parser"),
    ]);

    let enc_section = algo_section("Encoding & Other", &[
        ("Base64", "RFC 4648 standard encoding/decoding"),
        ("Hex", "Hexadecimal encoding/decoding"),
        ("CSPRNG", "Cryptographically secure random via OS RNG"),
        ("File Encryption", "AES-256-GCM with password-based KDF"),
    ]);

    let tls_section = algo_section("TLS", &[
        ("TLS 1.2 / 1.3", "Client connection test via rustls"),
        ("Certificate Chain", "Inspect server certificate chain"),
    ]);

    let content = Column::new()
        .push(title)
        .push(desc)
        .push(iced::widget::vertical_space().height(15))
        .push(hash_section)
        .push(sym_section)
        .push(asym_section)
        .push(sig_section)
        .push(cert_section)
        .push(enc_section)
        .push(tls_section)
        .spacing(10)
        .width(Length::Fill);

    container(scrollable(content.padding(25)))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(theme::CARD_BG)),
            border: Border { radius: 8.0.into(), ..Border::default() },
            ..container::Style::default()
        })
        .into()
}

fn algo_section<'a>(section_title: &str, items: &[(&str, &str)]) -> Element<'a, Msg> {
    let header = text(section_title.to_string())
        .size(16)
        .color(theme::ACCENT);

    let rows: Vec<Element<'a, Msg>> = items
        .iter()
        .map(|(name, desc)| {
            container(
                iced::widget::row![
                    text(name.to_string())
                        .size(13)
                        .font(iced::Font::MONOSPACE)
                        .color(theme::TEXT_DARK)
                        .width(200),
                    text(desc.to_string())
                        .size(12)
                        .color(iced::Color::from_rgb(0.45, 0.45, 0.5)),
                ]
                .spacing(15),
            )
            .padding([4, 10])
            .width(Length::Fill)
            .into()
        })
        .collect();

    let mut col = Column::new().push(header).spacing(3);
    for r in rows {
        col = col.push(r);
    }

    container(col.padding(12))
        .width(Length::Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.96, 0.97, 0.98))),
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: theme::BORDER,
            },
            ..container::Style::default()
        })
        .into()
}
