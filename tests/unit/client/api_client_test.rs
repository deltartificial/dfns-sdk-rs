/// @dfns-sdk-rs/tests/unit/client/api_client_test.rs

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use dfns_sdk_rs::{
        client::api_client::DfnsApiClient,
        error::DfnsError,
        models::generic::DfnsBaseApiOptions,
        signer::{CredentialSigner, FirstFactorAssertion, UserActionChallenge},
    };

    struct MockSigner;

    #[async_trait]
    impl CredentialSigner for MockSigner {
        async fn sign(
            &self,
            _challenge: UserActionChallenge,
        ) -> Result<FirstFactorAssertion, DfnsError> {
            Ok(FirstFactorAssertion {
                kind: dfns_sdk_rs::signer::FirstFactorAssertionKind::Key,
                credential_assertion: Some(
                    dfns_sdk_rs::signer::FirstFactorAssertionCredentialAssertion {
                        algorithm: Some("ES256".to_string()),
                        client_data: "test-client-data".to_string(),
                        cred_id: "test-cred-id".to_string(),
                        signature: "test-signature".to_string(),
                        authenticator_data: Some("test-auth-data".to_string()),
                        user_handle: Some("test-user-handle".to_string()),
                    },
                ),
                password: None,
            })
        }
    }

    #[test]
    fn test_api_client_initialization() {
        let base_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        };

        let client = DfnsApiClient::new(base_options.clone(), None);
        let _auth = client.auth();
        let _wallets = client.wallets();

        let signer = Arc::new(MockSigner);
        let client = DfnsApiClient::new(base_options, Some(signer));
        let _auth = client.auth();
        let _wallets = client.wallets();
    }

    #[test]
    fn test_api_endpoints_availability() {
        let base_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        };
        let client = DfnsApiClient::new(base_options, None);

        let _auth = client.auth();
        let _exchanges = client.exchanges();
        let _networks = client.networks();
        let _permissions = client.permissions();
        let _policies = client.policies();
        let _signers = client.signers();
        let _staking = client.staking();
        let _wallets = client.wallets();
        let _webhooks = client.webhooks();
    }

    #[test]
    fn test_api_client_with_custom_options() {
        let base_url = "https://api.test.com";
        let auth_token = "test-token";
        let base_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: Some(auth_token.to_string()),
            base_url: Some(base_url.to_string()),
            app_secret: Some("test-secret".to_string()),
        };
        let signer = Arc::new(MockSigner);
        let client = DfnsApiClient::new(base_options, Some(signer));

        let _auth = client.auth();
        let _wallets = client.wallets();
    }

    #[test]
    fn test_api_client_cloned_signer() {
        let base_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        };
        let signer = Arc::new(MockSigner);
        let client = DfnsApiClient::new(base_options, Some(signer));

        let _auth = client.auth();
        let _wallets = client.wallets();
    }
}
