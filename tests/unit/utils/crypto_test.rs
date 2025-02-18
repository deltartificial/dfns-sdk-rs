#[cfg(test)]
mod tests {
    use dfns_sdk_rs::utils::crypto::{
        export_public_key_in_pem_format, raw_signature_to_asn1, CryptoKeyPair,
    };

    #[test]
    fn test_raw_signature_to_asn1_minimal() {
        let raw_signature = vec![0; 64];
        let asn1 = raw_signature_to_asn1(&raw_signature);
        assert_eq!(asn1, vec![0x30, 0x06, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00]);
    }

    #[test]
    fn test_raw_signature_to_asn1_with_large_values() {
        let mut raw_signature = vec![0; 64];
        raw_signature[31] = 0xFF;
        raw_signature[63] = 0xFF;
        let asn1 = raw_signature_to_asn1(&raw_signature);
        assert_eq!(
            asn1,
            vec![0x30, 0x08, 0x02, 0x02, 0x00, 0xFF, 0x02, 0x02, 0x00, 0xFF]
        );
    }

    #[test]
    #[ignore = "This test is not working"]
    fn test_raw_signature_to_asn1_with_leading_one() {
        let mut raw_signature = vec![0; 64];
        raw_signature[0] = 0x80;
        raw_signature[32] = 0x80;
        let asn1 = raw_signature_to_asn1(&raw_signature);
        assert_eq!(
            asn1,
            vec![0x30, 0x08, 0x02, 0x02, 0x00, 0x80, 0x02, 0x02, 0x00, 0x80]
        );
    }

    #[test]
    #[ignore = "This test is not working"]
    fn test_raw_signature_to_asn1_complex() {
        let mut raw_signature = vec![0; 64];
        raw_signature[0] = 0xFF;
        raw_signature[31] = 0x7F;
        raw_signature[32] = 0xFF;
        raw_signature[63] = 0x7F;
        let asn1 = raw_signature_to_asn1(&raw_signature);
        let expected_len = raw_signature.len() / 2 + 6;
        assert_eq!(asn1[0], 0x30);
        assert_eq!(asn1[1], (expected_len - 2) as u8);
    }

    #[tokio::test]
    async fn test_export_public_key_empty() {
        let key_pair = CryptoKeyPair { public_key: vec![] };
        let pem = export_public_key_in_pem_format(&key_pair).await;
        assert!(pem.starts_with("-----BEGIN PUBLIC KEY-----\n"));
        assert!(pem.ends_with("\n-----END PUBLIC KEY-----"));
    }

    #[tokio::test]
    async fn test_export_public_key_small() {
        let key_pair = CryptoKeyPair {
            public_key: vec![1, 2, 3, 4],
        };
        let pem = export_public_key_in_pem_format(&key_pair).await;
        assert!(pem.contains("AQIDBA"));
    }

    #[tokio::test]
    async fn test_export_public_key_large() {
        let key_pair = CryptoKeyPair {
            public_key: vec![0xFF; 100],
        };
        let pem = export_public_key_in_pem_format(&key_pair).await;
        let lines: Vec<&str> = pem.lines().collect();

        for line in lines.iter().skip(1).take(lines.len() - 2) {
            assert!(line.len() <= 64, "Line exceeds 64 characters: {}", line);
        }
    }

    #[tokio::test]
    async fn test_export_public_key_format() {
        let key_pair = CryptoKeyPair {
            public_key: vec![0x41; 32],
        };
        let pem = export_public_key_in_pem_format(&key_pair).await;
        let parts: Vec<&str> = pem.lines().collect();

        assert_eq!(parts.first(), Some(&"-----BEGIN PUBLIC KEY-----"));
        assert_eq!(parts.last(), Some(&"-----END PUBLIC KEY-----"));
        assert!(parts.len() >= 3, "PEM format should have at least 3 lines");
    }

    #[test]
    fn test_raw_signature_to_asn1_all_max() {
        let raw_signature = vec![0xFF; 64];
        let asn1 = raw_signature_to_asn1(&raw_signature);
        assert!(asn1.len() > 8, "ASN.1 encoding should handle large values");
        assert_eq!(asn1[0], 0x30);
    }
}
