// @dfns-sdk-rs/src/client/delegated_api_client.rs

use crate::{
    api::{
        auth::delegated_client::DelegatedAuthClient,
        exchanges::delegated_client::DelegatedExchangesClient,
        networks::delegated_client::DelegatedNetworksClient,
        permissions::delegated_client::DelegatedPermissionsClient,
        policies::delegated_client::DelegatedPoliciesClient,
        signers::delegated_client::DelegatedSignersClient,
        staking::delegated_client::DelegatedStakingClient,
        wallets::delegated_client::DelegatedWalletsClient,
        webhooks::delegated_client::DelegatedWebhooksClient,
    },
    models::generic::DfnsBaseApiOptions,
};

#[derive(Debug, Clone)]
pub struct DfnsDelegatedApiClientOptions {
    pub base: DfnsBaseApiOptions,
}

pub struct DfnsDelegatedApiClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DfnsDelegatedApiClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub fn auth(&self) -> DelegatedAuthClient {
        DelegatedAuthClient::new(self.api_options.clone())
    }

    pub fn exchanges(&self) -> DelegatedExchangesClient {
        DelegatedExchangesClient::new(self.api_options.clone())
    }

    pub fn networks(&self) -> DelegatedNetworksClient {
        DelegatedNetworksClient::new(self.api_options.clone())
    }

    pub fn permissions(&self) -> DelegatedPermissionsClient {
        DelegatedPermissionsClient::new(self.api_options.clone())
    }

    pub fn policies(&self) -> DelegatedPoliciesClient {
        DelegatedPoliciesClient::new(self.api_options.clone())
    }

    pub fn staking(&self) -> DelegatedStakingClient {
        DelegatedStakingClient::new(self.api_options.clone())
    }

    pub fn signers(&self) -> DelegatedSignersClient {
        DelegatedSignersClient::new(self.api_options.clone())
    }

    pub fn wallets(&self) -> DelegatedWalletsClient {
        DelegatedWalletsClient::new(self.api_options.clone())
    }

    pub fn webhooks(&self) -> DelegatedWebhooksClient {
        DelegatedWebhooksClient::new(self.api_options.clone())
    }
}
