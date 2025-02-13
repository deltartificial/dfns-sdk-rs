/// @dfns-sdk-rs/tests/unit/utils/string_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::utils::string::{split_string, to_hex};

    #[test]
    fn test_split_string_empty() {
        let result = split_string("", None);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_split_string_shorter_than_max() {
        let input = "Hello World";
        let result = split_string(input, Some(64));
        assert_eq!(result, vec!["Hello World"]);
    }

    #[test]
    fn test_split_string_exact_length() {
        let input = "12345";
        let result = split_string(input, Some(5));
        assert_eq!(result, vec!["12345"]);
    }

    #[test]
    fn test_split_string_multiple_chunks() {
        let input = "abcdefghijklmnop";
        let result = split_string(input, Some(4));
        assert_eq!(result, vec!["abcd", "efgh", "ijkl", "mnop"]);
    }

    #[test]
    fn test_split_string_uneven_chunks() {
        let input = "abcdefghijk";
        let result = split_string(input, Some(4));
        assert_eq!(result, vec!["abcd", "efgh", "ijk"]);
    }

    #[test]
    fn test_split_string_unicode_boundary() {
        let input = "👋🌍😊";
        let result = split_string(input, Some(2));
        assert_eq!(result, vec!["👋🌍", "😊"]);
    }

    #[test]
    fn test_split_string_unicode_mixed() {
        let input = "a👋b🌍c";
        let result = split_string(input, Some(3));
        assert_eq!(result, vec!["a👋b", "🌍c"]);
    }

    #[test]
    fn test_split_string_default_length() {
        let input = "a".repeat(128);
        let result = split_string(&input, None);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 64);
        assert_eq!(result[1].len(), 64);
    }

    #[test]
    fn test_to_hex_empty() {
        let result = to_hex(&[]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_to_hex_single_byte() {
        let result = to_hex(&[0x0F]);
        assert_eq!(result, "0f");
    }

    #[test]
    fn test_to_hex_multiple_bytes() {
        let result = to_hex(&[0x00, 0x0F, 0xF0, 0xFF]);
        assert_eq!(result, "000ff0ff");
    }

    #[test]
    fn test_to_hex_all_values() {
        let input: Vec<u8> = (0..16).collect();
        let result = to_hex(&input);
        assert_eq!(result, "000102030405060708090a0b0c0d0e0f");
    }

    #[test]
    fn test_to_hex_large_input() {
        let input: Vec<u8> = (0..255).collect();
        let result = to_hex(&input);
        assert_eq!(result.len(), 510); // Each byte becomes 2 hex chars
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_to_hex_random_bytes() {
        let input = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let result = to_hex(&input);
        assert_eq!(result, "deadbeef");
    }
}
