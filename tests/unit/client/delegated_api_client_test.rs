/// @dfns-sdk-rs/tests/unit/client/delegated_api_client_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::{
        client::delegated_api_client::{DfnsDelegatedApiClient, DfnsDelegatedApiClientOptions},
        models::generic::DfnsBaseApiOptions,
    };

    #[test]
    fn test_delegated_api_client_initialization() {
        let base_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        };

        let api_options = DfnsDelegatedApiClientOptions { base: base_options };

        let client = DfnsDelegatedApiClient::new(api_options);
        let _auth = client.auth();
        let _wallets = client.wallets();
    }

    #[test]
    fn test_delegated_api_endpoints_availability() {
        let base_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: None,
            base_url: None,
            app_secret: None,
        };

        let api_options = DfnsDelegatedApiClientOptions { base: base_options };

        let client = DfnsDelegatedApiClient::new(api_options);

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
    fn test_delegated_api_client_with_custom_options() {
        let base_url = "https://api.test.com";
        let auth_token = "test-token";
        let base_options = DfnsBaseApiOptions {
            app_id: "test-app".to_string(),
            auth_token: Some(auth_token.to_string()),
            base_url: Some(base_url.to_string()),
            app_secret: Some("test-secret".to_string()),
        };

        let api_options = DfnsDelegatedApiClientOptions { base: base_options };

        let client = DfnsDelegatedApiClient::new(api_options);
        let _auth = client.auth();
        let _wallets = client.wallets();
    }
}
