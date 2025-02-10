// @dfns-sdk-rs/src/utils/bigint.rs

pub fn minimize_bigint(value: &[u8]) -> Vec<u8> {
    let extended = std::iter::once(0).chain(value.iter().copied()).collect::<Vec<_>>();
    
    for (i, &byte) in extended.iter().enumerate() {
        match byte {
            0 => continue,
            b if b > 0x7f => return extended[i - 1..].to_vec(),
            _ => return extended[i..].to_vec(),
        }
    }
    
    vec![0]
}