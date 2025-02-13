/// @dfns-sdk-rs/tests/unit/models/generic_test.rs  

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::models::generic::{DfnsApiClientOptions, DfnsBaseApiOptions};

    #[test]
    fn test_dfns_base_api_options() {
        let options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: Some("test-token".to_string()),
            base_url: Some("https://test.com".to_string()),
            app_secret: Some("test-secret".to_string()),
        };

        let debug_str = format!("{:?}", options);
        assert!(debug_str.contains("test-app"));
        assert!(debug_str.contains("test-token"));
        assert!(debug_str.contains("test-secret"));

        let cloned = options.clone();
        assert_eq!(options, cloned);

        let serialized = serde_json::to_value(&options).unwrap();
        assert_eq!(serialized["app_id"], "test-app");
        assert_eq!(serialized["auth_token"], "test-token");
        assert_eq!(serialized["base_url"], "https://test.com");
        assert_eq!(serialized["app_secret"], "test-secret");

        // Test with optional fields as None
        let minimal_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        };

        let serialized = serde_json::to_value(&minimal_options).unwrap();
        assert_eq!(serialized["app_id"], "test-app");
        assert!(!serialized.as_object().unwrap().contains_key("auth_token"));
        assert!(!serialized.as_object().unwrap().contains_key("base_url"));
        assert!(!serialized.as_object().unwrap().contains_key("app_secret"));
    }

    #[test]
    fn test_dfns_api_client_options() {
        let base = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        };

        let options = DfnsApiClientOptions::new(base.clone());
        assert_eq!(options.base, base);
        assert!(options.signer.is_none());

        let debug_str = format!("{:?}", options);
        assert!(debug_str.contains("DfnsApiClientOptions"));
        assert!(debug_str.contains("<dyn CredentialSigner>"));

        let cloned = options.clone();
        assert_eq!(cloned.base, options.base);
        assert!(cloned.signer.is_none());

        assert_eq!(options, cloned);

        let serialized = serde_json::to_value(&options).unwrap();
        assert_eq!(serialized["base"]["app_id"], "test-app");
        assert!(!serialized.as_object().unwrap().contains_key("signer"));
    }

    #[test]
    fn test_dfns_api_client_options_with_signer() {
        let base = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        };

        let options = DfnsApiClientOptions::new(base);

        let debug_str = format!("{:?}", options);
        assert!(debug_str.contains("DfnsApiClientOptions"));

        let cloned = options.clone();
        assert_eq!(cloned.base, options.base);
        assert!(cloned.signer.is_none());
    }
}
