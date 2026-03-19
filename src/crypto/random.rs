use rand::RngCore;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomFormat {
    Hex,
    Base64,
    Raw,
}

impl RandomFormat {
    pub const ALL: &'static [RandomFormat] = &[RandomFormat::Hex, RandomFormat::Base64, RandomFormat::Raw];
}

impl fmt::Display for RandomFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandomFormat::Hex => write!(f, "Hex"),
            RandomFormat::Base64 => write!(f, "Base64"),
            RandomFormat::Raw => write!(f, "Decimal Bytes"),
        }
    }
}

pub fn generate_random(byte_count: usize, format: RandomFormat) -> String {
    let mut bytes = vec![0u8; byte_count];
    rand::rngs::OsRng.fill_bytes(&mut bytes);

    match format {
        RandomFormat::Hex => hex::encode(&bytes),
        RandomFormat::Base64 => {
            use base64::{engine::general_purpose::STANDARD, Engine};
            STANDARD.encode(&bytes)
        }
        RandomFormat::Raw => bytes
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(" "),
    }
}
