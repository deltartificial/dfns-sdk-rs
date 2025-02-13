#[cfg(test)]
mod tests {
    use dfns_sdk_rs::signer::{
        AllowCredential, AllowCredentials, CredentialFactor, CredentialKind, CredentialTransport,
        FirstFactorAssertion, FirstFactorAssertionKind, Type, UserActionChallenge,
        UserVerificationRequirement,
    };

    #[test]
    fn test_credential_transport_serialization() {
        let transports = vec![
            CredentialTransport::Ble,
            CredentialTransport::Internal,
            CredentialTransport::Nfc,
            CredentialTransport::Usb,
        ];

        for transport in transports {
            let serialized = serde_json::to_string(&transport).unwrap();
            let deserialized: CredentialTransport = serde_json::from_str(&serialized).unwrap();
            assert_eq!(transport, deserialized);
        }
    }

    #[test]
    fn test_allow_credential() {
        let cred = AllowCredential {
            id: "test-id".to_string(),
            allow_credential_type: Type::PublicKey,
        };

        let serialized = serde_json::to_value(&cred).unwrap();
        assert_eq!(serialized["id"], "test-id");
        assert_eq!(serialized["type"], "public-key");
    }

    #[test]
    fn test_credential_factor() {
        let factors = vec![
            CredentialFactor::Either,
            CredentialFactor::First,
            CredentialFactor::Second,
        ];

        for factor in factors {
            let serialized = serde_json::to_string(&factor).unwrap();
            let deserialized: CredentialFactor = serde_json::from_str(&serialized).unwrap();
            assert_eq!(factor, deserialized);
        }
    }

    #[test]
    fn test_credential_kind() {
        let kinds = vec![
            CredentialKind::Fido2,
            CredentialKind::Key,
            CredentialKind::Password,
            CredentialKind::PasswordProtectedKey,
            CredentialKind::RecoveryKey,
            CredentialKind::Totp,
        ];

        for kind in kinds {
            let serialized = serde_json::to_string(&kind).unwrap();
            let deserialized: CredentialKind = serde_json::from_str(&serialized).unwrap();
            assert_eq!(kind, deserialized);
        }
    }

    #[test]
    fn test_user_verification_requirement() {
        let requirements = vec![
            UserVerificationRequirement::Discouraged,
            UserVerificationRequirement::Preferred,
            UserVerificationRequirement::Required,
        ];

        for req in requirements {
            let serialized = serde_json::to_string(&req).unwrap();
            let deserialized: UserVerificationRequirement =
                serde_json::from_str(&serialized).unwrap();
            assert_eq!(req, deserialized);
        }
    }

    #[test]
    fn test_first_factor_assertion_serialization() {
        let assertion = FirstFactorAssertion {
            credential_assertion: None,
            kind: FirstFactorAssertionKind::Key,
            password: Some("test-pass".to_string()),
        };

        let serialized = serde_json::to_value(&assertion).unwrap();
        assert_eq!(serialized["kind"].as_str().unwrap().to_lowercase(), "key");
        assert_eq!(serialized["password"], "test-pass");
    }

    #[test]
    fn test_user_action_challenge_serialization() {
        let challenge = UserActionChallenge {
            allow_credentials: AllowCredentials {
                key: vec![],
                webauthn: vec![],
            },
            challenge: "test-challenge".to_string(),
            challenge_identifier: "test-id".to_string(),
            external_authentication_url: "https://test.com".to_string(),
            rp: None,
            supported_credential_kinds: vec![],
            user_verification: UserVerificationRequirement::Required,
        };

        let serialized = serde_json::to_value(&challenge).unwrap();
        assert_eq!(serialized["challenge"], "test-challenge");
        assert_eq!(serialized["challengeIdentifier"], "test-id");
    }
}
