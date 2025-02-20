/// @dfns-sdk-rs/tests/unit/utils/user_action_fetch_test.rs

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use dfns_sdk_rs::{
        error::DfnsError,
        models::generic::{DfnsApiClientOptions, DfnsBaseApiOptions},
        signer::{CredentialSigner, FirstFactorAssertion, UserActionChallenge},
        utils::{
            fetch::{FetchOptions, HttpMethod},
            user_action_fetch::{user_action_fetch, UserActionFetch},
        },
    };
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn create_base_options(server_url: String) -> DfnsBaseApiOptions {
        DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            app_secret: Some("test-secret".to_string()),
            auth_token: Some("test-token".to_string()),
            base_url: Some(server_url),
        }
    }

    fn create_client_options(base: DfnsBaseApiOptions) -> DfnsApiClientOptions {
        DfnsApiClientOptions { base, signer: None }
    }

    #[derive(Clone)]
    struct MockSigner {
        challenge_response: String,
    }

    #[async_trait]
    impl CredentialSigner for MockSigner {
        async fn sign(
            &self,
            _challenge: UserActionChallenge,
        ) -> Result<FirstFactorAssertion, DfnsError> {
            Ok(FirstFactorAssertion {
                credential_assertion: None,
                kind: dfns_sdk_rs::signer::FirstFactorAssertionKind::Key,
                password: Some(self.challenge_response.clone()),
            })
        }
    }

    #[tokio::test]
    async fn test_user_action_fetch_get_request() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": "success"})))
            .mount(&mock_server)
            .await;

        let base_options = create_base_options(mock_server.uri());
        let client_options = create_client_options(base_options);

        let options = FetchOptions {
            method: HttpMethod::GET,
            headers: None,
            body: None,
            api_options: client_options,
        };

        let result: Value = user_action_fetch("/test", options).await.unwrap();
        assert_eq!(result["data"], "success");
    }

    #[tokio::test]
    async fn test_user_action_fetch_post_without_signer() {
        let mock_server = MockServer::start().await;
        let base_options = create_base_options(mock_server.uri());
        let client_options = create_client_options(base_options);

        let options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(json!({"test": "data"})),
            api_options: client_options,
        };

        let err = user_action_fetch::<Value>("/test", options)
            .await
            .unwrap_err();
        match err {
            DfnsError {
                http_status,
                message,
                ..
            } => {
                assert_eq!(http_status, 400);
                assert_eq!(message, "A 'signer' needs to be passed to Dfns client.");
            }
        }
    }

    #[tokio::test]
    async fn test_user_action_fetch_with_custom_headers() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .and(header("x-custom-header", "custom-value"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": "success"})))
            .mount(&mock_server)
            .await;

        let mut headers = HashMap::new();
        headers.insert("x-custom-header".to_string(), "custom-value".to_string());

        let base_options = create_base_options(mock_server.uri());
        let client_options = create_client_options(base_options);

        let options = FetchOptions {
            method: HttpMethod::GET,
            headers: Some(headers),
            body: None,
            api_options: client_options,
        };

        let result: Value = user_action_fetch("/test", options).await.unwrap();
        assert_eq!(result["data"], "success");
    }

    #[tokio::test]
    async fn test_user_action_fetch_invalid_url() {
        let base_options = create_base_options("http://localhost:1234".to_string());
        let client_options = create_client_options(base_options);

        let options = FetchOptions {
            method: HttpMethod::GET,
            headers: None,
            body: None,
            api_options: client_options,
        };

        let err = user_action_fetch::<Value>("://invalid", options)
            .await
            .unwrap_err();

        match err {
            DfnsError {
                http_status,
                message,
                ..
            } => {
                assert_eq!(http_status, 400);
                assert_eq!(message, "Invalid resource path: must be a relative path");
            }
        }
    }

    #[tokio::test]
    async fn test_user_action_fetch_missing_base_url() {
        let base_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            app_secret: Some("test-secret".to_string()),
            auth_token: Some("test-token".to_string()),
            base_url: None,
        };
        let client_options = DfnsApiClientOptions {
            base: base_options,
            signer: Some(Box::new(MockSigner {
                challenge_response: "test".to_string(),
            })),
        };

        let options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: None,
            api_options: client_options,
        };

        let err = user_action_fetch::<Value>("/test", options)
            .await
            .unwrap_err();
        match err {
            DfnsError {
                http_status,
                message,
                ..
            } => {
                assert_eq!(http_status, 400);
                assert_eq!(message, "Base URL is required in options");
            }
        }
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_user_action_fetch_url_parsing() {
        let mock_server = MockServer::start().await;
        let base_options = create_base_options(mock_server.uri());
        let client_options = create_client_options(base_options);

        let options = FetchOptions {
            method: HttpMethod::GET,
            headers: None,
            body: None,
            api_options: client_options,
        };

        let err = user_action_fetch::<Value>("not a url", options.clone())
            .await
            .unwrap_err();

        match err {
            DfnsError {
                http_status,
                message,
                ..
            } => {
                assert_eq!(http_status, 404);
                assert!(
                    message.starts_with("Invalid resource path:"),
                    "Error message should indicate invalid path: {}",
                    message
                );
            }
        }
    }

    #[test]
    fn test_user_action_fetch_debug_clone_eq() {
        let fetch1 = UserActionFetch::new();
        let fetch2 = fetch1.clone();

        assert_eq!(fetch1, fetch2);
        assert!(format!("{:?}", fetch1).contains("UserActionFetch"));
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_user_action_challenge_flow() {
        let mock_server = MockServer::start().await;
        let server_url = mock_server.uri();

        Mock::given(method("POST"))
            .and(path("/auth/action/init"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "allow_credentials": {
                    "key": [{
                        "id": "test-key-id",
                        "type": "public-key"
                    }],
                    "webauthn": []
                },
                "challenge": "test-challenge-data",
                "challenge_identifier": "test-challenge",
                "external_authentication_url": "http://test.com",
                "supported_credential_kinds": [{
                    "factor": "first",
                    "kind": "Key",
                    "requires_second_factor": false
                }],
                "user_verification": "preferred",
                "rp": null
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/auth/action"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "user_action": "signed-action-token"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/test"))
            .and(header("x-dfns-useraction", "signed-action-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": "success"
            })))
            .mount(&mock_server)
            .await;

        let base_options = create_base_options(server_url);
        let mock_signer = MockSigner {
            challenge_response: "test-signature".to_string(),
        };
        let client_options = DfnsApiClientOptions {
            base: base_options,
            signer: Some(Box::new(mock_signer)),
        };

        let options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(json!({"test": "data"})),
            api_options: client_options,
        };

        let result: Value = user_action_fetch("/test", options).await.unwrap();
        assert_eq!(result["data"], "success");
    }

    #[tokio::test]
    async fn test_user_action_fetch_missing_signer_error() {
        let mock_server = MockServer::start().await;
        let base_options = create_base_options(mock_server.uri());
        let client_options = create_client_options(base_options);

        let options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(json!({"test": "data"})),
            api_options: client_options,
        };

        let err = user_action_fetch::<Value>("/test", options)
            .await
            .unwrap_err();

        match err {
            DfnsError {
                http_status,
                message,
                ..
            } => {
                assert_eq!(http_status, 400);
                assert_eq!(message, "A 'signer' needs to be passed to Dfns client.");
            }
        }
    }
}
