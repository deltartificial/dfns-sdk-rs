#[cfg(test)]
mod tests {
    use dfns_sdk_rs::{
        api::auth::types::{
            Attestation, AuthenticatorAttachment, CreateCredentialChallengeResponse,
            CreateCredentialChallengeResponseAuthenticatorSelection,
            CreateCredentialChallengeResponseRp, CreateCredentialChallengeResponseUser,
            CreateRegistrationChallengeResponse,
            CreateRegistrationChallengeResponseAuthenticatorSelection,
            CreateRegistrationChallengeResponseRp,
            CreateRegistrationChallengeResponseSupportedCredentialKinds,
            CreateRegistrationChallengeResponseUser, CredentialKindElement, ResidentKey,
        },
        error::DfnsError,
        store::{
            Challenge, CredentialAttestation, CredentialAttestationCredentialInfo,
            CredentialAttestationCredentialKind, CredentialAttestationTrait, CredentialStore,
        },
    };
    use std::future::Future;
    use std::pin::Pin;

    // Create a newtype wrapper for CredentialAttestation
    #[derive(Debug)]
    struct TestCredentialAttestation(CredentialAttestation);

    impl CredentialAttestationTrait for TestCredentialAttestation {
        fn get_kind(&self) -> &str {
            match self.0.credential_kind {
                CredentialAttestationCredentialKind::Key => "Key",
                CredentialAttestationCredentialKind::Fido2 => "Fido2",
                CredentialAttestationCredentialKind::Password => "Password",
                CredentialAttestationCredentialKind::RecoveryKey => "RecoveryKey",
                CredentialAttestationCredentialKind::Totp => "Totp",
            }
        }
    }

    struct MockStore;

    impl CredentialStore<TestCredentialAttestation> for MockStore {
        fn create<'a>(
            &'a self,
            challenge: Challenge,
        ) -> Pin<Box<dyn Future<Output = Result<TestCredentialAttestation, DfnsError>> + Send + 'a>>
        {
            Box::pin(async move {
                match challenge {
                    Challenge::Registration(_) => {
                        Ok(TestCredentialAttestation(CredentialAttestation {
                            credential_info: CredentialAttestationCredentialInfo {
                                attestation_data: Some("test-data".to_string()),
                                client_data: Some("test-client".to_string()),
                                cred_id: Some("test-cred".to_string()),
                                password: None,
                                otp_code: None,
                            },
                            credential_kind: CredentialAttestationCredentialKind::Key,
                            encrypted_private_key: None,
                        }))
                    }
                    Challenge::Credential(_) => Err(DfnsError::new(400, "Invalid challenge", None)),
                }
            })
        }
    }

    #[tokio::test]
    async fn test_credential_store_create_success() {
        let store = MockStore;
        let challenge = Challenge::Registration(CreateRegistrationChallengeResponse {
            attestation: Attestation::Direct,
            authenticator_selection: CreateRegistrationChallengeResponseAuthenticatorSelection {
                authenticator_attachment: Some(AuthenticatorAttachment::Platform),
                require_resident_key: false,
                resident_key: ResidentKey::Discouraged,
                user_verification: ResidentKey::Discouraged,
            },
            challenge: "test".to_string(),
            exclude_credentials: vec![],
            otp_url: "test".to_string(),
            pub_key_cred_params: vec![],
            rp: Some(CreateRegistrationChallengeResponseRp {
                id: "test".to_string(),
                name: "test".to_string(),
            }),
            supported_credential_kinds:
                CreateRegistrationChallengeResponseSupportedCredentialKinds {
                    first_factor: vec![CredentialKindElement::Key],
                    second_factor: vec![],
                },
            temporary_authentication_token: "test".to_string(),
            user: CreateRegistrationChallengeResponseUser {
                display_name: "test".to_string(),
                id: "test".to_string(),
                name: "test".to_string(),
            },
        });

        let result = store.create(challenge).await.unwrap();
        assert_eq!(
            result.0.credential_kind,
            CredentialAttestationCredentialKind::Key
        );
        assert_eq!(
            result.0.credential_info.attestation_data,
            Some("test-data".to_string())
        );
        assert_eq!(result.get_kind(), "Key");
    }

    #[tokio::test]
    async fn test_credential_store_create_failure() {
        let store = MockStore;
        let challenge = Challenge::Credential(CreateCredentialChallengeResponse {
            challenge_identifier: "test".to_string(),
            kind: CredentialKindElement::Key,
            rp: Some(CreateCredentialChallengeResponseRp {
                id: "test".to_string(),
                name: "test".to_string(),
            }),
            temporary_authentication_token: "test".to_string(),
            user: CreateCredentialChallengeResponseUser {
                display_name: "test".to_string(),
                id: "test".to_string(),
                name: "test".to_string(),
            },
            otp_url: Some("test".to_string()),
            attestation: Some(Attestation::Direct),
            authenticator_selection: Some(
                CreateCredentialChallengeResponseAuthenticatorSelection {
                    authenticator_attachment: Some(AuthenticatorAttachment::Platform),
                    require_resident_key: false,
                    resident_key: ResidentKey::Discouraged,
                    user_verification: ResidentKey::Discouraged,
                },
            ),
            challenge: Some("test".to_string()),
            exclude_credentials: None,
            pub_key_cred_params: None,
        });

        let err = store.create(challenge).await.unwrap_err();
        assert_eq!(err.http_status, 400);
        assert_eq!(err.message, "Invalid challenge");
    }

    #[test]
    fn test_challenge_enum() {
        let reg_challenge = Challenge::Registration(CreateRegistrationChallengeResponse {
            attestation: Attestation::Direct,
            authenticator_selection: CreateRegistrationChallengeResponseAuthenticatorSelection {
                authenticator_attachment: Some(AuthenticatorAttachment::Platform),
                require_resident_key: false,
                resident_key: ResidentKey::Discouraged,
                user_verification: ResidentKey::Discouraged,
            },
            challenge: "test".to_string(),
            exclude_credentials: vec![],
            otp_url: "test".to_string(),
            pub_key_cred_params: vec![],
            rp: Some(CreateRegistrationChallengeResponseRp {
                id: "test".to_string(),
                name: "test".to_string(),
            }),
            supported_credential_kinds:
                CreateRegistrationChallengeResponseSupportedCredentialKinds {
                    first_factor: vec![CredentialKindElement::Key],
                    second_factor: vec![],
                },
            temporary_authentication_token: "test".to_string(),
            user: CreateRegistrationChallengeResponseUser {
                display_name: "test".to_string(),
                id: "test".to_string(),
                name: "test".to_string(),
            },
        });

        let cred_challenge = Challenge::Credential(CreateCredentialChallengeResponse {
            challenge_identifier: "test".to_string(),
            kind: CredentialKindElement::Key,
            rp: Some(CreateCredentialChallengeResponseRp {
                id: "test".to_string(),
                name: "test".to_string(),
            }),
            temporary_authentication_token: "test".to_string(),
            user: CreateCredentialChallengeResponseUser {
                display_name: "test".to_string(),
                id: "test".to_string(),
                name: "test".to_string(),
            },
            otp_url: Some("test".to_string()),
            attestation: Some(Attestation::Direct),
            authenticator_selection: Some(
                CreateCredentialChallengeResponseAuthenticatorSelection {
                    authenticator_attachment: Some(AuthenticatorAttachment::Platform),
                    require_resident_key: false,
                    resident_key: ResidentKey::Discouraged,
                    user_verification: ResidentKey::Discouraged,
                },
            ),
            challenge: Some("test".to_string()),
            exclude_credentials: None,
            pub_key_cred_params: None,
        });

        assert!(matches!(reg_challenge, Challenge::Registration(_)));
        assert!(matches!(cred_challenge, Challenge::Credential(_)));
    }
}
