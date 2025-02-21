#[cfg(test)]
mod tests {
    use dfns_sdk_rs::error::{DfnsError, PolicyPendingError};
    use serde_json::json;
    use std::error::Error;
    use std::fmt::Display;
    use url::Url;

    #[test]
    fn test_dfns_error_creation() {
        let error = DfnsError::new(400, "Bad Request", Some(json!({"detail": "test"})));
        assert_eq!(error.http_status, 400);
        assert_eq!(error.message, "Bad Request");
        assert!(error.context.is_some());
    }

    #[test]
    fn test_dfns_error_display() {
        let error = DfnsError::new(404, "Not Found", None);
        let display = format!("{}", error);
        assert!(display.contains("404"));
        assert!(display.contains("Not Found"));
    }

    #[test]
    fn test_policy_pending_error() {
        let error = PolicyPendingError::new(Some(json!({"status": "pending"})));
        assert_eq!(error.http_status, 202);
        assert!(error.message.contains("policy pending"));
        assert!(error.context.is_some());
    }

    #[test]
    fn test_policy_pending_to_dfns_error() {
        let policy_error = PolicyPendingError::new(None);
        let dfns_error: DfnsError = policy_error.into();
        assert_eq!(dfns_error.http_status, 202);
    }

    #[test]
    fn test_policy_pending_error_display() {
        let error = PolicyPendingError::new(Some(json!({"status": "pending"})));
        let display = format!("{}", error);
        assert!(display.contains("202"));
        assert!(display.contains("policy pending"));
        assert!(display.contains("status"));
    }

    #[test]
    fn test_error_traits() {
        let error = DfnsError::new(400, "test", None);
        let _: &dyn Error = &error;
        let _: &dyn Display = &error;

        let policy_error = PolicyPendingError::new(None);
        let _: &dyn Error = &policy_error;
        let _: &dyn Display = &policy_error;
    }

    #[test]
    fn test_error_conversions() {
        let policy_error = PolicyPendingError::new(None);
        let _: DfnsError = policy_error.into();

        let client = reqwest::Client::new();
        let invalid_url = "ht tp://invalid.com";
        let req_error = client.get(invalid_url).build().unwrap_err();
        let _: DfnsError = req_error.into();

        let parse_error = Url::parse("invalid-url").unwrap_err();
        let _: DfnsError = parse_error.into();

        let header_error = reqwest::header::HeaderValue::from_str("invalid\nheader").unwrap_err();
        let _: DfnsError = header_error.into();

        let json_error = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let _: DfnsError = json_error.into();
    }

    #[test]
    fn test_error_json_formatting() {
        let error = DfnsError::new(
            500,
            "Internal Error",
            Some(json!({"details": "test error"})),
        );
        let display = format!("{}", error);
        assert!(display.contains("\"httpStatus\": 500"));
        assert!(display.contains("\"message\": \"Internal Error\""));
        assert!(display.contains("\"details\": \"test error\""));
    }

    #[tokio::test]
    async fn test_reqwest_error_conversion() {
        let url = Url::parse("http://invalid").unwrap();
        let req_error = reqwest::get(url).await.unwrap_err();
        let dfns_error: DfnsError = req_error.into();
        assert!(dfns_error.message.contains("error"));
    }

    #[test]
    fn test_url_parse_error_conversion() {
        let parse_error = Url::parse("invalid-url").unwrap_err();
        let dfns_error: DfnsError = parse_error.into();
        assert_eq!(dfns_error.http_status, 400);
        assert!(dfns_error.message.contains("URL parsing error"));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let dfns_error: DfnsError = json_error.into();
        assert_eq!(dfns_error.http_status, 400);
        assert!(dfns_error.message.contains("JSON"));
    }
}
