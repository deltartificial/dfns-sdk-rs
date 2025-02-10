// @dfns-sdk-rs/src/utils/string.rs

pub fn split_string(text: &str, max_line_length: Option<usize>) -> Vec<String> {
    let length = max_line_length.unwrap_or(64);
    text.chars()
        .collect::<Vec<_>>()
        .chunks(length)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

pub fn to_hex(buffer: &[u8]) -> String {
    buffer.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}