/// @dfns-sdk-rs/tests/unit/utils/fetch_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::{
        error::DfnsError,
        models::generic::DfnsBaseApiOptions,
        utils::fetch::{DfnsFetch, Fetch, FetchOptions, HttpMethod},
    };
    use reqwest::Method;
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn create_mock_options(app_id: &str, server_url: String) -> FetchOptions<DfnsBaseApiOptions> {
        FetchOptions {
            method: HttpMethod::GET,
            headers: None,
            body: None,
            api_options: DfnsBaseApiOptions {
                app_id: app_id.to_string(),
                app_secret: Some("test-secret".to_string()),
                auth_token: Some("test-token".to_string()),
                base_url: Some(server_url),
            },
        }
    }

    #[tokio::test]
    async fn test_fetch_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({"data": "test-response"})),
            )
            .mount(&mock_server)
            .await;

        let fetch = DfnsFetch::new();
        let options = create_mock_options("test-app", mock_server.uri());

        let response = fetch.execute("/test", options).await.unwrap();
        assert_eq!(response.status(), 200);

        let body: Value = response.json().await.unwrap();
        assert_eq!(body["data"], "test-response");
    }

    #[tokio::test]
    async fn test_fetch_with_custom_headers() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .and(header("x-custom-header", "custom-value"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let mut headers = HashMap::new();
        headers.insert("x-custom-header".to_string(), "custom-value".to_string());

        let mut options = create_mock_options("test-app", mock_server.uri());
        options.headers = Some(headers);

        let fetch = DfnsFetch::new();
        let response = fetch.execute("/test", options).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_fetch_with_body() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let mut options = create_mock_options("test-app", mock_server.uri());
        options.method = HttpMethod::POST;
        options.body = Some(json!({"key": "value"}));

        let fetch = DfnsFetch::new();
        let response = fetch.execute("/test", options).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_fetch_error_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_json(json!({"error": {"message": "Bad Request"}})),
            )
            .mount(&mock_server)
            .await;

        let fetch = DfnsFetch::new();
        let options = create_mock_options("test-app", mock_server.uri());

        let response = fetch.execute("/test", options).await.unwrap();
        let err = fetch.handle_response(response).await.unwrap_err();

        match err {
            DfnsError {
                http_status,
                message,
                ..
            } => {
                assert_eq!(http_status, 400);
                assert_eq!(message, "Bad Request");
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_policy_pending() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(202).set_body_json(json!({"status": "pending"})))
            .mount(&mock_server)
            .await;

        let fetch = DfnsFetch::new();
        let options = create_mock_options("test-app", mock_server.uri());

        let response = fetch.execute("/test", options).await.unwrap();
        let err = fetch.handle_response(response).await.unwrap_err();

        assert!(matches!(
            err,
            DfnsError {
                http_status: 202,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn test_fetch_invalid_url() {
        let fetch = DfnsFetch::new();
        let mut options = create_mock_options("test-app", "invalid-url".to_string());
        options.api_options.base_url = Some("invalid-url".to_string());

        let err = fetch.execute("/test", options).await.unwrap_err();
        assert!(matches!(err, DfnsError { .. }));
    }

    #[test]
    fn test_http_method_conversion() {
        assert_eq!(Method::from(HttpMethod::GET), Method::GET);
        assert_eq!(Method::from(HttpMethod::POST), Method::POST);
        assert_eq!(Method::from(HttpMethod::PUT), Method::PUT);
        assert_eq!(Method::from(HttpMethod::DELETE), Method::DELETE);
    }

    #[test]
    fn test_dfns_fetch_debug() {
        let fetch = DfnsFetch::new();
        let debug_str = format!("{:?}", fetch);
        assert!(debug_str.contains("DfnsFetch"));
    }

    #[test]
    fn test_dfns_fetch_clone_and_eq() {
        let fetch1 = DfnsFetch::new();
        let fetch2 = fetch1.clone();
        assert_eq!(fetch1, fetch2);
    }
}
