/// @dfns-sdk-rs/tests/unit/client/base_auth_api_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::{
        client::base_auth_api::{
            BaseAuthApi, CreateUserActionChallengeRequest, CreateUserLoginChallengeRequest,
            CreateUserRecoveryRequest, CreateUserRegistrationChallengeRequest,
            CreateUserRegistrationRequest, NewCredentials, SignUserActionChallengeRequest,
        },
        error::DfnsError,
        models::generic::DfnsBaseApiOptions,
        signer::{
            FirstFactorAssertion, FirstFactorAssertionCredentialAssertion,
            FirstFactorAssertionKind, RecoveryKeyAssertionKind,
        },
        store::{
            FirstFactorAttestation, FirstFactorAttestationCredentialInfo,
            FirstFactorAttestationCredentialKind,
        },
        utils::fetch::HttpMethod,
    };
    use mockall::mock;
    use mockall::predicate::*;
    use std::time::Duration;
    use tokio::time::timeout;

    mock! {
        pub HttpClient {
            async fn request(&self, _url: String, _method: HttpMethod, _headers: Option<std::collections::HashMap<String, String>>, _body: Option<serde_json::Value>) -> Result<serde_json::Value, DfnsError>;
        }
    }

    fn create_test_base_options() -> DfnsBaseApiOptions {
        DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        }
    }

    #[tokio::test]
    async fn test_create_user_action_challenge() {
        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_request()
            .returning(|_, _, _, _| Err(DfnsError::new(500, "Test error", None)));

        let options = create_test_base_options();
        let request = CreateUserActionChallengeRequest {
            user_action_payload: "test-payload".to_string(),
            user_action_http_method: HttpMethod::POST,
            user_action_http_path: "/test/path".to_string(),
            user_action_server_kind: "test-server".to_string(),
        };

        let result = timeout(
            Duration::from_secs(1),
            BaseAuthApi::create_user_action_challenge(request, options),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_sign_user_action_challenge() {
        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_request()
            .returning(|_, _, _, _| Err(DfnsError::new(500, "Test error", None)));

        let options = create_test_base_options();
        let request = SignUserActionChallengeRequest {
            challenge_identifier: "test-challenge".to_string(),
            first_factor: FirstFactorAssertion {
                kind: FirstFactorAssertionKind::Key,
                credential_assertion: Some(FirstFactorAssertionCredentialAssertion {
                    algorithm: Some("ES256".to_string()),
                    client_data: "test-client-data".to_string(),
                    cred_id: "test-cred-id".to_string(),
                    signature: "test-signature".to_string(),
                    authenticator_data: Some("test-auth-data".to_string()),
                    user_handle: Some("test-user-handle".to_string()),
                }),
                password: None,
            },
            second_factor: None,
        };

        let result = timeout(
            Duration::from_secs(1),
            BaseAuthApi::sign_user_action_challenge(request, options),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_create_user_login_challenge() {
        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_request()
            .returning(|_, _, _, _| Err(DfnsError::new(500, "Test error", None)));

        let options = create_test_base_options();
        let request = CreateUserLoginChallengeRequest {
            username: "test-user".to_string(),
            org_id: "test-org".to_string(),
        };

        let result = timeout(
            Duration::from_secs(1),
            BaseAuthApi::create_user_login_challenge(request, options),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_create_user_login() {
        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_request()
            .returning(|_, _, _, _| Err(DfnsError::new(500, "Test error", None)));

        let options = create_test_base_options();
        let request = SignUserActionChallengeRequest {
            challenge_identifier: "test-challenge".to_string(),
            first_factor: FirstFactorAssertion {
                kind: FirstFactorAssertionKind::Key,
                credential_assertion: Some(FirstFactorAssertionCredentialAssertion {
                    algorithm: Some("ES256".to_string()),
                    client_data: "test-client-data".to_string(),
                    cred_id: "test-cred-id".to_string(),
                    signature: "test-signature".to_string(),
                    authenticator_data: Some("test-auth-data".to_string()),
                    user_handle: Some("test-user-handle".to_string()),
                }),
                password: None,
            },
            second_factor: None,
        };

        let result = timeout(
            Duration::from_secs(1),
            BaseAuthApi::create_user_login(request, options),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_create_user_registration_challenge() {
        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_request()
            .returning(|_, _, _, _| Err(DfnsError::new(500, "Test error", None)));

        let options = create_test_base_options();
        let request = CreateUserRegistrationChallengeRequest {
            org_id: "test-org".to_string(),
            username: "test-user".to_string(),
            registration_code: "test-code".to_string(),
        };

        let result = timeout(
            Duration::from_secs(1),
            BaseAuthApi::create_user_registration_challenge(request, options),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_create_user_registration() {
        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_request()
            .returning(|_, _, _, _| Err(DfnsError::new(500, "Test error", None)));

        let options = create_test_base_options();
        let request = CreateUserRegistrationRequest {
            first_factor_credential: FirstFactorAttestation {
                credential_info: FirstFactorAttestationCredentialInfo {
                    attestation_data: Some("test-attestation-data".to_string()),
                    client_data: Some("test-client-data".to_string()),
                    cred_id: Some("test-cred-id".to_string()),
                    password: None,
                },
                credential_kind: FirstFactorAttestationCredentialKind::Key,
            },
            second_factor_credential: None,
            recovery_credential: None,
        };

        let result = timeout(
            Duration::from_secs(1),
            BaseAuthApi::create_user_registration(request, options),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_create_user_recovery() {
        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_request()
            .returning(|_, _, _, _| Err(DfnsError::new(500, "Test error", None)));

        let options = create_test_base_options();
        let request = CreateUserRecoveryRequest {
            recovery: dfns_sdk_rs::signer::RecoveryKeyAssertion {
                kind: RecoveryKeyAssertionKind::RecoveryKey,
                credential_assertion:
                    dfns_sdk_rs::signer::RecoveryKeyAssertionCredentialAssertion {
                        algorithm: Some("ES256".to_string()),
                        client_data: "test-client-data".to_string(),
                        cred_id: "test-cred-id".to_string(),
                        signature: "test-signature".to_string(),
                    },
            },
            new_credentials: NewCredentials {
                first_factor_credential: FirstFactorAttestation {
                    credential_info: FirstFactorAttestationCredentialInfo {
                        attestation_data: Some("test-attestation-data".to_string()),
                        client_data: Some("test-client-data".to_string()),
                        cred_id: Some("test-cred-id".to_string()),
                        password: None,
                    },
                    credential_kind: FirstFactorAttestationCredentialKind::Key,
                },
                second_factor_credential: None,
                recovery_credential: None,
            },
        };

        let result = timeout(
            Duration::from_secs(1),
            BaseAuthApi::create_user_recovery(request, options),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }
}
