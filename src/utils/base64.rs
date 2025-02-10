// @dfns-sdk-rs/src/utils/base64.rs

use base64::{engine::general_purpose, Engine};

pub fn to_base64(input: impl AsRef<[u8]>) -> String {
    general_purpose::STANDARD.encode(input)
}

pub fn from_base64(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(encoded)
}

pub fn to_base64_url(input: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(input)
}

pub fn from_base64_url(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let padded = match encoded.len() % 4 {
        0 => encoded.to_string(),
        n => format!("{}{}", encoded, "=".repeat(4 - n)),
    };
    general_purpose::URL_SAFE.decode(&padded)
}
