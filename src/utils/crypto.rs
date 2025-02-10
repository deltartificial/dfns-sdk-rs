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

    let mut result = Vec::with_capacity(2 + min_r.len() + min_s.len() + 6);
    result.push(0x30);
    result.push((min_r.len() + min_s.len() + 4) as u8);
    result.push(0x02);
    result.push(min_r.len() as u8);
    result.extend_from_slice(&min_r);
    result.push(0x02);
    result.push(min_s.len() as u8);
    result.extend_from_slice(&min_s);

    result
}
