/// @dfns-sdk-rs/tests/unit/utils/nonce_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::utils::{base64, nonce::generate_nonce};
    use serde_json::Value;

    #[test]
    fn test_generate_nonce_format() {
        let nonce = generate_nonce();
        let decoded = base64::from_base64_url(&nonce).unwrap();
        let json_str = String::from_utf8(decoded).unwrap();
        let json: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json.is_object(), "Nonce should decode to a JSON object");
        assert!(json.get("uuid").is_some(), "Should contain uuid field");
        assert!(json.get("date").is_some(), "Should contain date field");
    }

    #[test]
    fn test_generate_nonce_uuid_format() {
        let nonce = generate_nonce();
        let decoded = base64::from_base64_url(&nonce).unwrap();
        let json_str = String::from_utf8(decoded).unwrap();
        let json: Value = serde_json::from_str(&json_str).unwrap();

        let uuid = json["uuid"].as_str().unwrap();
        assert_eq!(uuid.len(), 36, "UUID should be 36 characters long");
        assert_eq!(
            uuid.matches('-').count(),
            4,
            "UUID should contain 4 hyphens"
        );
    }

    #[test]
    fn test_generate_nonce_date_format() {
        let nonce = generate_nonce();
        let decoded = base64::from_base64_url(&nonce).unwrap();
        let json_str = String::from_utf8(decoded).unwrap();
        let json: Value = serde_json::from_str(&json_str).unwrap();

        let date = json["date"].as_str().unwrap();
        assert!(
            date.contains('T'),
            "Date should be in RFC3339 format with T separator"
        );
        assert!(
            date.contains('Z') || date.contains('+'),
            "Date should include timezone"
        );
    }

    #[test]
    fn test_generate_nonce_uniqueness() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        assert_ne!(nonce1, nonce2, "Generated nonces should be unique");

        let decoded1 = base64::from_base64_url(&nonce1).unwrap();
        let decoded2 = base64::from_base64_url(&nonce2).unwrap();
        let json1: Value = serde_json::from_str(std::str::from_utf8(&decoded1).unwrap()).unwrap();
        let json2: Value = serde_json::from_str(std::str::from_utf8(&decoded2).unwrap()).unwrap();

        assert_ne!(
            json1["uuid"], json2["uuid"],
            "Generated UUIDs should be unique"
        );
        assert_ne!(
            json1["date"], json2["date"],
            "Generated timestamps should be different"
        );
    }

    #[test]
    fn test_generate_nonce_base64url_compliance() {
        let nonce = generate_nonce();
        assert!(!nonce.contains('+'), "Should not contain + character");
        assert!(!nonce.contains('/'), "Should not contain / character");
        assert!(!nonce.contains('='), "Should not contain padding");
    }

    #[test]
    fn test_generate_nonce_multiple_rapid() {
        let nonces: Vec<String> = (0..1000).map(|_| generate_nonce()).collect();
        let unique_nonces: std::collections::HashSet<_> = nonces.iter().collect();
        assert_eq!(
            nonces.len(),
            unique_nonces.len(),
            "All rapidly generated nonces should be unique"
        );
    }
}
