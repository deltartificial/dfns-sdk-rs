// /// @dfns-sdk-rs/tests/unit/api/auth/client_test.rs
// use dfns_sdk_rs::{
//     api::auth::{client::AuthClient, types::*},
//     error::DfnsError,
//     models::generic::{DfnsApiClientOptions, DfnsBaseApiOptions},
//     signer::CredentialSigner,
//     utils::{fetch::simple_fetch, url::build_path_and_query, user_action_fetch::user_action_fetch},
// };
// use mockito;
// use serde_json::json;
// use std::collections::HashMap;
// #[path = "../../../common/mocks.rs"]
// mod mocks;
// use mocks::{create_mock_server, create_mock_with_server};

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn setup_test_client(server: &mut mockito::ServerGuard) -> AuthClient {
//         let base_options = DfnsBaseApiOptions {
//             app_id: "test-app".to_string(),
//             auth_token: None,
//             base_url: Some(server.url()),
//             app_secret: None,
//         };
//         let api_options = DfnsApiClientOptions::new(base_options);
//         AuthClient::new(api_options)
//     }

//     #[tokio::test]
//     async fn test_activate_application() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let app_id = "test-app-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/apps/test-app-id/activate")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .activate_application(ActivateApplicationRequest {
//                 app_id: app_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_activate_credential() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/credentials/activate")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .activate_credential(ActivateCredentialRequest {
//                 body: ActivateCredentialRequestBody {
//                     credential_uuid: "test-cred-id".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_activate_personal_access_token() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let token_id = "test-token-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/pats/test-token-id/activate")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .activate_personal_access_token(ActivatePersonalAccessTokenRequest {
//                 token_id: token_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_activate_service_account() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let service_account_id = "test-sa-id";

//         let _m = create_mock_with_server(
//             &mut server,
//             "PUT",
//             "/auth/service-accounts/test-sa-id/activate",
//         )
//         .with_status(200)
//         .with_body(
//             json!({
//                 "status": "success"
//             })
//             .to_string(),
//         )
//         .create();

//         let result = client
//             .activate_service_account(ActivateServiceAccountRequest {
//                 service_account_id: service_account_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_activate_user() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let user_id = "test-user-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/users/test-user-id/activate")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .activate_user(ActivateUserRequest {
//                 user_id: user_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_archive_application() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let app_id = "test-app-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/apps/test-app-id/archive")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .archive_application(ArchiveApplicationRequest {
//                 app_id: app_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_archive_personal_access_token() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let token_id = "test-token-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/pats/test-token-id/archive")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .archive_personal_access_token(ArchivePersonalAccessTokenRequest {
//                 token_id: token_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_archive_service_account() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let service_account_id = "test-sa-id";

//         let _m = create_mock_with_server(
//             &mut server,
//             "PUT",
//             "/auth/service-accounts/test-sa-id/archive",
//         )
//         .with_status(200)
//         .with_body(
//             json!({
//                 "status": "success"
//             })
//             .to_string(),
//         )
//         .create();

//         let result = client
//             .archive_service_account(ArchiveServiceAccountRequest {
//                 service_account_id: service_account_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_archive_user() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let user_id = "test-user-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/users/test-user-id/archive")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .archive_user(ArchiveUserRequest {
//                 user_id: user_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_application() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/apps")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "appId": "test-app-id",
//                     "name": "Test App"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_application(CreateApplicationRequest {
//                 body: CreateApplicationBody {
//                     name: "Test App".to_string(),
//                     external_id: Some("test-external-id".to_string()),
//                     kind: ActivateApplicationResponseKind::ServerSideApplication,
//                     origin: Some("https://example.com".to_string()),
//                     permission_id: Some("test-permission-id".to_string()),
//                     relying_party_id: Some("test-rp-id".to_string()),
//                     days_valid: Some(365.0),
//                     public_key: Some("test-public-key".to_string()),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_credential() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/credentials")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "credentialId": "test-cred-id"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_credential(CreateCredentialRequest {
//                 body: CreateCredentialBody {
//                     challenge_identifier: "test-challenge".to_string(),
//                     credential_info: CreateCredentialBodyCredentialInfo {
//                         attestation_data: Some("test-attestation".to_string()),
//                         client_data: Some("test-client-data".to_string()),
//                         cred_id: Some("test-cred-id".to_string()),
//                         password: Some("test-password".to_string()),
//                         otp_code: Some("test-otp".to_string()),
//                     },
//                     credential_kind: CredentialKindElement::Key,
//                     credential_name: "Test Credential".to_string(),
//                     encrypted_private_key: Some("test-encrypted-key".to_string()),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_credential_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/credentials/init")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "challenge": "test-challenge"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_credential_challenge(CreateCredentialChallengeRequest {
//                 body: CreateCredentialChallengeRequestBody {
//                     kind: CredentialKindElement::Key,
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_credential_challenge_with_code() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/credentials/code/init")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "challenge": "test-challenge",
//                     "code": "test-code"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_credential_challenge_with_code(CreateCredentialChallengeWithCodeRequest {
//                 body: CreateCredentialChallengeWithCodeRequestBody {
//                     code: "test-code".to_string(),
//                     credential_kind: CredentialKindElement::Key,
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_credential_code() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/credentials/code")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "code": "test-code"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_credential_code(CreateCredentialCodeRequest {
//                 body: CreateCredentialCodeRequestBody {
//                     expiration: Expiration::String("2024-12-31T23:59:59Z".to_string()),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_credential_with_code() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/credentials/code/verify")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "credentialId": "test-cred-id"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_credential_with_code(CreateCredentialWithCodeRequest {
//                 body: CreateCredentialWithCodeBody {
//                     challenge_identifier: "test-challenge".to_string(),
//                     credential_info: CreateCredentialWithCodeBodyCredentialInfo {
//                         attestation_data: Some("test-attestation".to_string()),
//                         client_data: Some("test-client-data".to_string()),
//                         cred_id: Some("test-cred-id".to_string()),
//                         password: None,
//                         otp_code: None,
//                     },
//                     credential_kind: CredentialKindElement::Key,
//                     credential_name: "test-credential".to_string(),
//                     encrypted_private_key: None,
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_delegated_recovery_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/recover/user/delegated")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "challenge": "test-challenge"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_delegated_recovery_challenge(CreateDelegatedRecoveryChallengeRequest {
//                 body: CreateDelegatedRecoveryChallengeRequestBody {
//                     credential_id: "test-cred-id".to_string(),
//                     username: "test-user".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_delegated_registration_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m =
//             create_mock_with_server(&mut server, "POST", "/auth/registration/delegated/restart")
//                 .with_status(200)
//                 .with_body(
//                     json!({
//                         "challenge": "test-challenge"
//                     })
//                     .to_string(),
//                 )
//                 .create();

//         let result = client
//             .recreate_delegated_registration_challenge(
//                 RecreateDelegatedRegistrationChallengeRequest {
//                     body: RecreateDelegatedRegistrationChallengeRequestBody {
//                         email: "test@example.com".to_string(),
//                         external_id: Some("test-external-id".to_string()),
//                         kind: UserInfoKind::CustomerEmployee,
//                     },
//                 },
//             )
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_login_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/login/init")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "challenge": "test-challenge"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_login_challenge(CreateLoginChallengeRequest {
//                 body: CreateLoginChallengeRequestBody {
//                     login_code: Some("test-code".to_string()),
//                     org_id: "test-org".to_string(),
//                     username: "test-user".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_personal_access_token() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/pats")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "tokenId": "test-token-id",
//                     "token": "test-token"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_personal_access_token(CreatePersonalAccessTokenRequest {
//                 body: CreatePersonalAccessTokenRequestBody {
//                     name: "Test Token".to_string(),
//                     days_valid: Some(30.0),
//                     external_id: Some("test-external-id".to_string()),
//                     permission_id: Some("test-permission-id".to_string()),
//                     public_key: "test-public-key".to_string(),
//                     seconds_valid: Some(2592000.0),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_recovery_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/recover/user/init")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "challenge": "test-challenge"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_recovery_challenge(CreateRecoveryChallengeRequest {
//                 body: CreateRecoveryChallengeRequestBody {
//                     credential_id: "test-cred-id".to_string(),
//                     org_id: "test-org-id".to_string(),
//                     username: "test-user".to_string(),
//                     verification_code: "test-verification-code".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_registration_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/registration/init")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "challenge": "test-challenge"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_registration_challenge(CreateRegistrationChallengeRequest {
//                 body: CreateRegistrationChallengeRequestBody {
//                     org_id: "test-org-id".to_string(),
//                     registration_code: "test-registration-code".to_string(),
//                     username: "test-user".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_service_account() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/service-accounts")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "serviceAccountId": "test-sa-id",
//                     "name": "Test Service Account"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_service_account(CreateServiceAccountRequest {
//                 body: CreateServiceAccountRequestBody {
//                     name: "Test Service Account".to_string(),
//                     days_valid: Some(30.0),
//                     external_id: Some("test-external-id".to_string()),
//                     permission_id: Some("test-permission-id".to_string()),
//                     public_key: "test-public-key".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_social_registration_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/registration/social")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "challenge": "test-challenge"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_social_registration_challenge(CreateSocialRegistrationChallengeRequest {
//                 body: CreateSocialRegistrationChallengeRequestBody {
//                     id_token: "test-token".to_string(),
//                     social_login_provider_kind: SocialLoginProviderKind::Oidc,
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_user() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/users")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "userId": "test-user-id",
//                     "username": "test-user"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_user(CreateUserRequest {
//                 body: CreateUserRequestBody {
//                     email: "test@example.com".to_string(),
//                     external_id: Some("test-external-id".to_string()),
//                     kind: CreateUserBodyKind::CustomerEmployee,
//                     public_key: Some("test-public-key".to_string()),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_user_action_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/action/init")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "challenge": "test-challenge"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_user_action_challenge(CreateUserActionChallengeRequest {
//                 body: CreateUserActionChallengeRequestBody {
//                     user_action_http_method: "POST".to_string(),
//                     user_action_http_path: "/test/path".to_string(),
//                     user_action_payload: "test-payload".to_string(),
//                     user_action_server_kind: Some(UserActionServerKind::Api),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_create_user_action_signature() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/action")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "signature": "test-signature"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .create_user_action_signature(CreateUserActionSignatureRequest {
//                 body: CreateUserActionSignatureRequestBody {
//                     challenge_identifier: "test-challenge".to_string(),
//                     first_factor: PurpleFirstFactor {
//                         credential_assertion: Some(TentacledCredentialAssertion {
//                             algorithm: Some("test-algo".to_string()),
//                             authenticator_data: Some("test-auth-data".to_string()),
//                             client_data: "test-client-data".to_string(),
//                             cred_id: "test-cred-id".to_string(),
//                             signature: "test-signature".to_string(),
//                             user_handle: Some("test-user-handle".to_string()),
//                         }),
//                         kind: FirstFactorKind::Key,
//                         password: Some("test-password".to_string()),
//                     },
//                     second_factor: Some(FluffySecondFactor {
//                         credential_assertion: Some(StickyCredentialAssertion {
//                             algorithm: Some("test-algo".to_string()),
//                             authenticator_data: Some("test-auth-data".to_string()),
//                             client_data: "test-client-data".to_string(),
//                             cred_id: "test-cred-id".to_string(),
//                             signature: "test-signature".to_string(),
//                             user_handle: Some("test-user-handle".to_string()),
//                         }),
//                         kind: SecondFactorKind::Totp,
//                         otp_code: Some("123456".to_string()),
//                     }),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_deactivate_application() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let app_id = "test-app-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/apps/test-app-id/deactivate")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .deactivate_application(DeactivateApplicationRequest {
//                 app_id: app_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_deactivate_credential() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/credentials/deactivate")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .deactivate_credential(DeactivateCredentialRequest {
//                 body: DeactivateCredentialRequestBody {
//                     credential_uuid: "test-cred-uuid".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_deactivate_personal_access_token() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let token_id = "test-token-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/pats/test-token-id/deactivate")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .deactivate_personal_access_token(DeactivatePersonalAccessTokenRequest {
//                 token_id: token_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_deactivate_service_account() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let service_account_id = "test-sa-id";

//         let _m = create_mock_with_server(
//             &mut server,
//             "PUT",
//             "/auth/service-accounts/test-sa-id/deactivate",
//         )
//         .with_status(200)
//         .with_body(
//             json!({
//                 "status": "success"
//             })
//             .to_string(),
//         )
//         .create();

//         let result = client
//             .deactivate_service_account(DeactivateServiceAccountRequest {
//                 service_account_id: service_account_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_deactivate_user() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let user_id = "test-user-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/users/test-user-id/deactivate")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .deactivate_user(DeactivateUserRequest {
//                 user_id: user_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_delegated_login() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/login/delegated")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "token": "test-token"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .delegated_login(DelegatedLoginRequest {
//                 body: DelegatedLoginRequestBody {
//                     username: "test-user".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_get_application() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let app_id = "test-app-id";

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/apps/test-app-id")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "appId": "test-app-id",
//                     "name": "Test App"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .get_application(GetApplicationRequest {
//                 app_id: app_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_get_personal_access_token() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let token_id = "test-token-id";

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/pats/test-token-id")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "tokenId": "test-token-id",
//                     "name": "Test Token"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .get_personal_access_token(GetPersonalAccessTokenRequest {
//                 token_id: token_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_get_service_account() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let service_account_id = "test-sa-id";

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/service-accounts/test-sa-id")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "serviceAccountId": "test-sa-id",
//                     "name": "Test Service Account"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .get_service_account(GetServiceAccountRequest {
//                 service_account_id: service_account_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_get_user() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let user_id = "test-user-id";

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/users/test-user-id")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "userId": "test-user-id",
//                     "username": "test-user"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .get_user(GetUserRequest {
//                 user_id: user_id.to_string(),
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_list_applications() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/apps")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "items": []
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client.list_applications().await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_list_credentials() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/credentials")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "items": []
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client.list_credentials().await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_list_personal_access_tokens() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/pats")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "items": []
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client.list_personal_access_tokens().await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_list_service_accounts() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/service-accounts")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "items": []
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client.list_service_accounts().await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_list_users() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "GET", "/auth/users")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "items": []
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client.list_users(None).await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_login() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/login")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "token": "test-token"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .login(LoginRequest {
//                 body: LoginRequestBody {
//                     challenge_identifier: "test-challenge".to_string(),
//                     first_factor: FluffyFirstFactor {
//                         credential_assertion: Some(HilariousCredentialAssertion {
//                             algorithm: Some("test-algo".to_string()),
//                             authenticator_data: Some("test-auth-data".to_string()),
//                             client_data: "test-client-data".to_string(),
//                             cred_id: "test-cred-id".to_string(),
//                             signature: "test-signature".to_string(),
//                             user_handle: Some("test-user-handle".to_string()),
//                         }),
//                         kind: FirstFactorKind::Key,
//                         password: Some("test-password".to_string()),
//                     },
//                     second_factor: Some(StickySecondFactor {
//                         credential_assertion: Some(AmbitiousCredentialAssertion {
//                             algorithm: Some("test-algo".to_string()),
//                             authenticator_data: Some("test-auth-data".to_string()),
//                             client_data: "test-client-data".to_string(),
//                             cred_id: "test-cred-id".to_string(),
//                             signature: "test-signature".to_string(),
//                             user_handle: Some("test-user-handle".to_string()),
//                         }),
//                         kind: SecondFactorKind::Totp,
//                         otp_code: Some("123456".to_string()),
//                     }),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_logout() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/logout")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client.logout(None).await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_recover() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/recover/user")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .recover(RecoverRequest {
//                 body: RecoverRequestBody {
//                     new_credentials: BodyNewCredentials {
//                         first_factor_credential: FluffyFirstFactorCredential {
//                             credential_info: StickyCredentialInfo {
//                                 attestation_data: Some("test-attestation".to_string()),
//                                 client_data: Some("test-client-data".to_string()),
//                                 cred_id: Some("test-cred-id".to_string()),
//                                 password: Some("test-password".to_string()),
//                             },
//                             credential_kind: FirstFactorKind::Key,
//                             credential_name: Some("Test Credential".to_string()),
//                             encrypted_private_key: Some("test-encrypted-key".to_string()),
//                         },
//                         recovery_credential: Some(FluffyRecoveryCredential {
//                             credential_info: IndigoCredentialInfo {
//                                 attestation_data: "test-attestation".to_string(),
//                                 client_data: "test-client-data".to_string(),
//                                 cred_id: "test-cred-id".to_string(),
//                             },
//                             credential_kind: RecoveryCredentialKind::RecoveryKey,
//                             credential_name: Some("Test Recovery".to_string()),
//                             encrypted_private_key: Some("test-encrypted-key".to_string()),
//                         }),
//                         second_factor_credential: Some(FluffySecondFactorCredential {
//                             credential_info: IndecentCredentialInfo {
//                                 attestation_data: Some("test-attestation".to_string()),
//                                 client_data: Some("test-client-data".to_string()),
//                                 cred_id: Some("test-cred-id".to_string()),
//                                 otp_code: Some("123456".to_string()),
//                             },
//                             credential_kind: SecondFactorKind::Totp,
//                             credential_name: Some("Test Second Factor".to_string()),
//                             encrypted_private_key: Some("test-encrypted-key".to_string()),
//                         }),
//                     },
//                     recovery: BodyRecovery {
//                         credential_assertion: MagentaCredentialAssertion {
//                             algorithm: Some("test-algo".to_string()),
//                             client_data: "test-client-data".to_string(),
//                             cred_id: "test-cred-id".to_string(),
//                             signature: "test-signature".to_string(),
//                         },
//                         kind: RecoveryCredentialKind::RecoveryKey,
//                     },
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_recreate_delegated_registration_challenge() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m =
//             create_mock_with_server(&mut server, "POST", "/auth/registration/delegated/restart")
//                 .with_status(200)
//                 .with_body(
//                     json!({
//                         "challenge": "test-challenge"
//                     })
//                     .to_string(),
//                 )
//                 .create();

//         let result = client
//             .recreate_delegated_registration_challenge(
//                 RecreateDelegatedRegistrationChallengeRequest {
//                     body: RecreateDelegatedRegistrationChallengeRequestBody {
//                         email: "test@example.com".to_string(),
//                         external_id: Some("test-external-id".to_string()),
//                         kind: UserInfoKind::CustomerEmployee,
//                     },
//                 },
//             )
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_register() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/registration")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .register(RegisterRequest {
//                 body: RegisterRequestBody {
//                     first_factor_credential: TentacledFirstFactorCredential {
//                         credential_info: MagentaCredentialInfo {
//                             attestation_data: Some("test-attestation".to_string()),
//                             client_data: Some("test-client-data".to_string()),
//                             cred_id: Some("test-cred-id".to_string()),
//                             password: Some("test-password".to_string()),
//                         },
//                         credential_kind: FirstFactorKind::Key,
//                         credential_name: Some("Test Credential".to_string()),
//                         encrypted_private_key: Some("test-encrypted-key".to_string()),
//                     },
//                     recovery_credential: Some(TentacledRecoveryCredential {
//                         credential_info: FriskyCredentialInfo {
//                             attestation_data: "test-attestation".to_string(),
//                             client_data: "test-client-data".to_string(),
//                             cred_id: "test-cred-id".to_string(),
//                         },
//                         credential_kind: RecoveryCredentialKind::RecoveryKey,
//                         credential_name: Some("Test Recovery".to_string()),
//                         encrypted_private_key: Some("test-encrypted-key".to_string()),
//                     }),
//                     second_factor_credential: Some(StickySecondFactorCredential {
//                         credential_info: MischievousCredentialInfo {
//                             attestation_data: Some("test-attestation".to_string()),
//                             client_data: Some("test-client-data".to_string()),
//                             cred_id: Some("test-cred-id".to_string()),
//                             otp_code: Some("123456".to_string()),
//                         },
//                         credential_kind: SecondFactorKind::Totp,
//                         credential_name: Some("Test Second Factor".to_string()),
//                         encrypted_private_key: Some("test-encrypted-key".to_string()),
//                     }),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_register_end_user() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/registration/enduser")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .register_end_user(RegisterEndUserRequest {
//                 body: RegisterEndUserRequestBody {
//                     first_factor_credential: StickyFirstFactorCredential {
//                         credential_info: CredentialInfo3 {
//                             attestation_data: Some("test-attestation".to_string()),
//                             client_data: Some("test-client-data".to_string()),
//                             cred_id: Some("test-cred-id".to_string()),
//                             password: Some("test-password".to_string()),
//                         },
//                         credential_kind: FirstFactorKind::Key,
//                         credential_name: Some("Test Credential".to_string()),
//                         encrypted_private_key: Some("test-encrypted-key".to_string()),
//                     },
//                     recovery_credential: None,
//                     second_factor_credential: None,
//                     wallets: vec![BodyWallet {
//                         name: Some("Test Wallet".to_string()),
//                         network: Network::Ethereum,
//                     }],
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_resend_registration_code() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/registration/code")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .resend_registration_code(ResendRegistrationCodeRequest {
//                 body: ResendRegistrationCodeRequestBody {
//                     org_id: "test-org-id".to_string(),
//                     username: "test-user".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_send_login_code() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/login/code")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .send_login_code(SendLoginCodeRequest {
//                 body: SendLoginCodeRequestBody {
//                     org_id: "test-org-id".to_string(),
//                     username: "test-user".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_send_recovery_code() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/recover/user/code")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "status": "success"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .send_recovery_code(SendRecoveryCodeRequest {
//                 body: SendRecoveryCodeRequestBody {
//                     org_id: "test-org-id".to_string(),
//                     username: "test-user".to_string(),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_social_login() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/login/social")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "token": "test-token"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .social_login(SocialLoginRequest {
//                 body: SocialLoginRequestBody {
//                     id_token: "test-token".to_string(),
//                     social_login_provider_kind: SocialLoginProviderKind::Oidc,
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_update_application() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let app_id = "test-app-id";

//         let _m = create_mock_with_server(&mut server, "POST", "/auth/apps/test-app-id")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "appId": "test-app-id",
//                     "name": "Updated Test App"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .update_application(UpdateApplicationRequest {
//                 app_id: app_id.to_string(),
//                 body: UpdateApplicationRequestBody {
//                     external_id: Some("test-external-id".to_string()),
//                     name: Some("Updated Test App".to_string()),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_update_personal_access_token() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let token_id = "test-token-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/pats/test-token-id")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "tokenId": "test-token-id",
//                     "name": "Updated Test Token"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .update_personal_access_token(UpdatePersonalAccessTokenRequest {
//                 token_id: token_id.to_string(),
//                 body: UpdatePersonalAccessTokenRequestBody {
//                     external_id: Some("test-external-id".to_string()),
//                     name: Some("Updated Test Token".to_string()),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_update_service_account() {
//         let mut server = create_mock_server();
//         let client = setup_test_client(&mut server);
//         let service_account_id = "test-sa-id";

//         let _m = create_mock_with_server(&mut server, "PUT", "/auth/service-accounts/test-sa-id")
//             .with_status(200)
//             .with_body(
//                 json!({
//                     "serviceAccountId": "test-sa-id",
//                     "name": "Updated Test Service Account"
//                 })
//                 .to_string(),
//             )
//             .create();

//         let result = client
//             .update_service_account(UpdateServiceAccountRequest {
//                 service_account_id: service_account_id.to_string(),
//                 body: UpdateServiceAccountRequestBody {
//                     external_id: Some("test-external-id".to_string()),
//                     name: Some("Updated Test Service Account".to_string()),
//                 },
//             })
//             .await;

//         assert!(result.is_ok());
//     }
// }
