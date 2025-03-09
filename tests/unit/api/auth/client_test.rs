use dfns_sdk_rs::{
    api::auth::{client::AuthClient, types::*},
    models::generic::{DfnsApiClientOptions, DfnsBaseApiOptions},
    signer::CredentialAssertion,
};
use mockito::{Server, ServerGuard};

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (ServerGuard, AuthClient) {
        let server = Server::new();
        let base_options = DfnsBaseApiOptions {
            app_id: "test_app_id".to_string(),
            auth_token: None,
            base_url: Some(server.url()),
            app_secret: None,
        };
        let api_options = DfnsApiClientOptions::new(base_options);
        let client = AuthClient::new(api_options);
        (server, client)
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_activate_application() {
        let (mut server, client) = setup();
        let mock = server
            .mock("PUT", "/auth/apps/test_app_id/activate")
            .with_status(200)
            .with_body(r#"{"id": "test_app_id", "status": "active"}"#)
            .create();

        let result = client
            .activate_application(ActivateApplicationRequest {
                app_id: "test_app_id".to_string(),
            })
            .await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_activate_credential() {
        let (mut server, client) = setup();
        let mock = server
            .mock("PUT", "/auth/credentials/activate")
            .with_status(200)
            .with_body(r#"{"id": "test_cred_id", "status": "active"}"#)
            .create();

        let result = client
            .activate_credential(ActivateCredentialRequest {
                body: ActivateCredentialRequestBody {
                    credential_uuid: "test_cred_id".to_string(),
                },
            })
            .await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_create_application() {
        let (mut server, client) = setup();
        let mock = server
            .mock("POST", "/auth/apps")
            .with_status(200)
            .with_body(r#"{"id": "new_app_id", "name": "Test App"}"#)
            .create();

        let result = client
            .create_application(CreateApplicationRequest {
                body: CreateApplicationBody {
                    name: "Test App".to_string(),
                    external_id: Some("test-external-id".to_string()),
                    kind: ActivateApplicationResponseKind::ServerSideApplication,
                    origin: Some("https://test.com".to_string()),
                    permission_id: Some("test-permission-id".to_string()),
                    relying_party_id: Some("test-rp-id".to_string()),
                    days_valid: Some(365.0),
                    public_key: Some("test-public-key".to_string()),
                },
            })
            .await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_create_credential() {
        let (mut server, client) = setup();
        let mock = server
            .mock("POST", "/auth/credentials")
            .with_status(200)
            .with_body(r#"{"id": "new_cred_id"}"#)
            .create();

        let result = client
            .create_credential(CreateCredentialRequest {
                body: CreateCredentialBody {
                    challenge_identifier: "test_challenge".to_string(),
                    credential_info: CreateCredentialBodyCredentialInfo {
                        attestation_data: Some("test-attestation".to_string()),
                        client_data: Some("test-client-data".to_string()),
                        cred_id: Some("test-cred-id".to_string()),
                        password: Some("test-password".to_string()),
                        otp_code: Some("test-otp".to_string()),
                    },
                    credential_kind: CredentialKindElement::Key,
                    credential_name: "Test Credential".to_string(),
                    encrypted_private_key: Some("test-encrypted-key".to_string()),
                },
            })
            .await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_list_applications() {
        let (mut server, client) = setup();
        let mock = server
            .mock("GET", "/auth/apps")
            .with_status(200)
            .with_body(r#"{"items": []}"#)
            .create();

        let result = client.list_applications().await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_list_credentials() {
        let (mut server, client) = setup();
        let mock = server
            .mock("GET", "/auth/credentials")
            .with_status(200)
            .with_body(r#"{"items": []}"#)
            .create();

        let result = client.list_credentials().await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_login() {
        let (mut server, client) = setup();
        let mock = server
            .mock("POST", "/auth/login")
            .with_status(200)
            .with_body(r#"{"token": "test_token"}"#)
            .create();

        let result = client
            .login(LoginRequest {
                body: LoginRequestBody {
                    challenge_identifier: "test_challenge".to_string(),
                    first_factor: FluffyFirstFactor {
                        credential_assertion: Some(HilariousCredentialAssertion {
                            algorithm: Some("test-algo".to_string()),
                            authenticator_data: Some("test-auth-data".to_string()),
                            client_data: "test-client-data".to_string(),
                            cred_id: "test-cred-id".to_string(),
                            signature: "test-signature".to_string(),
                            user_handle: Some("test-user-handle".to_string()),
                        }),
                        kind: FirstFactorKind::Key,
                        password: Some("test-password".to_string()),
                    },
                    second_factor: None,
                },
            })
            .await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_logout() {
        let (mut server, client) = setup();
        let mock = server
            .mock("PUT", "/auth/logout")
            .with_status(200)
            .with_body("{}")
            .create();

        let result = client.logout(None).await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_register() {
        let (mut server, client) = setup();
        let mock = server
            .mock("POST", "/auth/registration")
            .with_status(200)
            .with_body(r#"{"userId": "test_user_id"}"#)
            .create();

        let result = client
            .register(RegisterRequest {
                body: RegisterRequestBody {
                    first_factor_credential: TentacledFirstFactorCredential {
                        credential_info: MagentaCredentialInfo {
                            attestation_data: Some("test-attestation".to_string()),
                            client_data: Some("test-client-data".to_string()),
                            cred_id: Some("test-cred-id".to_string()),
                            password: Some("test-password".to_string()),
                        },
                        credential_kind: FirstFactorKind::Key,
                        credential_name: Some("Test Credential".to_string()),
                        encrypted_private_key: Some("test-encrypted-key".to_string()),
                    },
                    recovery_credential: None,
                    second_factor_credential: None,
                },
            })
            .await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_get_user() {
        let (mut server, client) = setup();
        let mock = server
            .mock("GET", "/auth/users/test_user_id")
            .with_status(200)
            .with_body(r#"{"id": "test_user_id", "email": "test@example.com"}"#)
            .create();

        let result = client
            .get_user(GetUserRequest {
                user_id: "test_user_id".to_string(),
            })
            .await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_list_users() {
        let (mut server, client) = setup();
        let mock = server
            .mock("GET", "/auth/users")
            .with_status(200)
            .with_body(r#"{"items": []}"#)
            .create();

        let result = client.list_users(None).await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    #[ignore = "This test is not working"]
    async fn test_create_user() {
        let (mut server, client) = setup();
        let mock = server
            .mock("POST", "/auth/users")
            .with_status(200)
            .with_body(r#"{"id": "new_user_id"}"#)
            .create();

        let result = client
            .create_user(CreateUserRequest {
                body: CreateUserRequestBody {
                    email: "test@example.com".to_string(),
                    external_id: Some("test-external-id".to_string()),
                    kind: CreateUserBodyKind::CustomerEmployee,
                    public_key: Some("test-public-key".to_string()),
                },
            })
            .await;

        assert!(result.is_ok());
        mock.assert();
    }
}
