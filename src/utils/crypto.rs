// @dfns-sdk-rs/src/utils/crypto.rs

use crate::utils::{base64, bigint, string};

pub struct CryptoKeyPair {
    pub public_key: Vec<u8>,
}

pub async fn export_public_key_in_pem_format(key: &CryptoKeyPair) -> String {
    let b64_exported = base64::to_base64(&key.public_key);
    let lines = string::split_string(&b64_exported, Some(64));
    format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
        lines.join("\n")
    )
}

pub fn raw_signature_to_asn1(raw_signature: &[u8]) -> Vec<u8> {
    let (r, s) = raw_signature.split_at(32);
    let min_r = bigint::minimize_bigint(r);
    let min_s = bigint::minimize_bigint(s);

    let r_bytes = if !min_r.is_empty() && (min_r[0] & 0x80) != 0 {
        let mut padded = Vec::with_capacity(min_r.len() + 1);
        padded.push(0);
        padded.extend_from_slice(&min_r);
        padded
    } else {
        min_r
    };

    let s_bytes = if !min_s.is_empty() && (min_s[0] & 0x80) != 0 {
        let mut padded = Vec::with_capacity(min_s.len() + 1);
        padded.push(0);
        padded.extend_from_slice(&min_s);
        padded
    } else {
        min_s
    };

    let total_len = r_bytes.len() + s_bytes.len() + 4;

    let mut result = Vec::with_capacity(total_len + 2);
    result.push(0x30);
    result.push(total_len as u8);
    result.push(0x02);
    result.push(r_bytes.len() as u8);
    result.extend_from_slice(&r_bytes);
    result.push(0x02);
    result.push(s_bytes.len() as u8);
    result.extend_from_slice(&s_bytes);

    result
}
