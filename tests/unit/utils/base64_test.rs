/// @dfns-sdk-rs/tests/unit/utils/base64_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::utils::base64::{from_base64, from_base64_url, to_base64, to_base64_url};

    #[test]
    fn test_to_base64_empty() {
        let input = b"";
        assert_eq!(to_base64(input), "");
    }

    #[test]
    fn test_to_base64_simple_string() {
        let input = b"Hello, World!";
        assert_eq!(to_base64(input), "SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn test_to_base64_binary_data() {
        let input = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(to_base64(input), "AAECAwQF");
    }

    #[test]
    fn test_from_base64_empty() {
        let result = from_base64("").unwrap();
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_from_base64_valid_string() {
        let result = from_base64("SGVsbG8sIFdvcmxkIQ==").unwrap();
        assert_eq!(result, b"Hello, World!");
    }

    #[test]
    fn test_from_base64_invalid_input() {
        assert!(from_base64("invalid base64!").is_err());
    }

    #[test]
    fn test_to_base64url_empty() {
        let input = b"";
        assert_eq!(to_base64_url(input), "");
    }

    #[test]
    fn test_to_base64url_simple_string() {
        let input = b"Hello, World!";
        assert_eq!(to_base64_url(input), "SGVsbG8sIFdvcmxkIQ");
    }

    #[test]
    fn test_to_base64url_with_special_chars() {
        let input = b"Special?+/Characters";
        assert_eq!(to_base64_url(input), "U3BlY2lhbD8rL0NoYXJhY3RlcnM");
    }

    #[test]
    fn test_from_base64url_empty() {
        let result = from_base64_url("").unwrap();
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_from_base64url_valid_string() {
        let result = from_base64_url("SGVsbG8sIFdvcmxkIQ").unwrap();
        assert_eq!(result, b"Hello, World!");
    }

    #[test]
    fn test_from_base64url_with_padding() {
        let result = from_base64_url("SGVsbG8").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_from_base64url_invalid_input() {
        assert!(from_base64_url("invalid base64 url!").is_err());
    }

    #[test]
    fn test_roundtrip_base64() {
        let original = b"Test string for roundtrip";
        let encoded = to_base64(original);
        let decoded = from_base64(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_roundtrip_base64url() {
        let original = b"Test string for roundtrip with special chars +/?";
        let encoded = to_base64_url(original);
        let decoded = from_base64_url(&encoded).unwrap();
        assert_eq!(decoded, original);
    }
}
