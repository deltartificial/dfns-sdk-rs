/// @dfns-sdk-rs/tests/unit/utils/user_action_fetch_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::{
        error::DfnsError,
        models::generic::{DfnsApiClientOptions, DfnsBaseApiOptions},
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
                assert!(message.contains("signer"));
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
        let base_options = create_base_options("invalid-url".to_string());
        let client_options = create_client_options(base_options);

        let options = FetchOptions {
            method: HttpMethod::GET,
            headers: None,
            body: None,
            api_options: client_options,
        };

        let err = user_action_fetch::<Value>("/test", options)
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            DfnsError {
                http_status: 400,
                ..
            }
        ));
    }

    #[test]
    fn test_user_action_fetch_debug_clone_eq() {
        let fetch1 = UserActionFetch::new();
        let fetch2 = fetch1.clone();

        assert_eq!(fetch1, fetch2);
        assert!(format!("{:?}", fetch1).contains("UserActionFetch"));
    }
}
