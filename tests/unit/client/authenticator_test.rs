/// @dfns-sdk-rs/tests/unit/client/authenticator_test.rs

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;

    use async_trait::async_trait;
    use dfns_sdk_rs::{
        api::auth::types::{
            CreateRegistrationChallengeResponse,
            CreateRegistrationChallengeResponseAuthenticatorSelection,
            CreateRegistrationChallengeResponseRp,
            CreateRegistrationChallengeResponseSupportedCredentialKinds,
            CreateRegistrationChallengeResponseUser,
        },
        client::authenticator::{
            DfnsAuthenticator, DfnsAuthenticatorOptions, LoginRequest, RegisterRequest,
        },
        error::DfnsError,
        signer::{
            CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge,
        },
        store::{
            Challenge, CredentialStore, FirstFactorAttestation,
            FirstFactorAttestationCredentialInfo, FirstFactorAttestationCredentialKind,
        },
    };

    struct MockAuthenticator;

    #[async_trait]
    impl CredentialSigner for MockAuthenticator {
        async fn sign(
            &self,
            _challenge: UserActionChallenge,
        ) -> Result<FirstFactorAssertion, DfnsError> {
            Ok(FirstFactorAssertion {
                kind: FirstFactorAssertionKind::Key,
                credential_assertion: None,
                password: Some("test-password".to_string()),
            })
        }
    }

    impl CredentialStore<FirstFactorAttestation> for MockAuthenticator {
        fn create<'a>(
            &'a self,
            _challenge: Challenge,
        ) -> Pin<Box<dyn Future<Output = Result<FirstFactorAttestation, DfnsError>> + Send + 'a>>
        {
            Box::pin(async move {
                Ok(FirstFactorAttestation {
                    credential_info: FirstFactorAttestationCredentialInfo {
                        attestation_data: Some("test-attestation-data".to_string()),
                        client_data: Some("test-client-data".to_string()),
                        cred_id: Some("test-cred-id".to_string()),
                        password: None,
                    },
                    credential_kind: FirstFactorAttestationCredentialKind::Key,
                })
            })
        }
    }

    fn create_test_options() -> DfnsAuthenticatorOptions<MockAuthenticator> {
        DfnsAuthenticatorOptions {
            app_id: "test-app".to_string(),
            base_url: None,
            app_secret: None,
            signer: MockAuthenticator,
        }
    }

    #[tokio::test]
    async fn test_login() {
        let authenticator = DfnsAuthenticator::new(create_test_options());
        let request = LoginRequest {
            username: "test-user".to_string(),
            org_id: "test-org".to_string(),
        };

        let result = authenticator.login(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register() {
        let authenticator = DfnsAuthenticator::new(create_test_options());
        let request = RegisterRequest {
            org_id: "test-org".to_string(),
            username: "test-user".to_string(),
            registration_code: "test-code".to_string(),
        };

        let result = authenticator.register(request).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_authenticator_options() {
        let options = create_test_options();
        assert_eq!(options.app_id, "test-app");
        assert!(options.base_url.is_none());
        assert!(options.app_secret.is_none());
    }

    #[tokio::test]
    async fn test_mock_signer() {
        let signer = MockAuthenticator;
        let challenge = UserActionChallenge {
            challenge: "test-challenge".to_string(),
            challenge_identifier: "test-id".to_string(),
            external_authentication_url: "test-url".to_string(),
            allow_credentials: dfns_sdk_rs::signer::AllowCredentials {
                key: vec![],
                webauthn: vec![],
            },
            rp: None,
            supported_credential_kinds: vec![],
            user_verification: dfns_sdk_rs::signer::UserVerificationRequirement::Discouraged,
        };

        let result = signer.sign(challenge).await;
        assert!(result.is_ok());
        let assertion = result.unwrap();
        assert_eq!(assertion.kind, FirstFactorAssertionKind::Key);
        assert!(assertion.credential_assertion.is_none());
        assert_eq!(assertion.password, Some("test-password".to_string()));
    }

    #[tokio::test]
    async fn test_mock_credential_store() {
        let store = MockAuthenticator;
        let challenge = Challenge::Registration(CreateRegistrationChallengeResponse {
            attestation: dfns_sdk_rs::api::auth::types::Attestation::Direct,
            authenticator_selection: CreateRegistrationChallengeResponseAuthenticatorSelection {
                authenticator_attachment: None,
                require_resident_key: false,
                resident_key: dfns_sdk_rs::api::auth::types::ResidentKey::Discouraged,
                user_verification: dfns_sdk_rs::api::auth::types::ResidentKey::Discouraged,
            },
            challenge: "test-challenge".to_string(),
            exclude_credentials: vec![],
            otp_url: "test-otp-url".to_string(),
            pub_key_cred_params: vec![],
            rp: Some(CreateRegistrationChallengeResponseRp {
                id: "test-rp-id".to_string(),
                name: "test-rp-name".to_string(),
            }),
            supported_credential_kinds:
                CreateRegistrationChallengeResponseSupportedCredentialKinds {
                    first_factor: vec![],
                    second_factor: vec![],
                },
            temporary_authentication_token: "test-token".to_string(),
            user: CreateRegistrationChallengeResponseUser {
                display_name: "test-display-name".to_string(),
                id: "test-id".to_string(),
                name: "test-name".to_string(),
            },
        });

        let result = store.create(challenge).await;
        assert!(result.is_ok());
        let attestation = result.unwrap();
        assert_eq!(
            attestation.credential_kind,
            FirstFactorAttestationCredentialKind::Key
        );
        assert_eq!(
            attestation.credential_info.attestation_data,
            Some("test-attestation-data".to_string())
        );
        assert_eq!(
            attestation.credential_info.client_data,
            Some("test-client-data".to_string())
        );
        assert_eq!(
            attestation.credential_info.cred_id,
            Some("test-cred-id".to_string())
        );
        assert!(attestation.credential_info.password.is_none());
    }
}
