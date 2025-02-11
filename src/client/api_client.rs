// @dfns-sdk-rs/src/client/api_client.rs

use crate::{
    api::{
        auth::client::AuthClient, exchanges::client::ExchangesClient,
        networks::client::NetworksClient, permissions::client::PermissionsClient,
        policies::client::PoliciesClient, signers::client::SignersClient,
        staking::client::StakingClient, wallets::client::WalletsClient,
        webhooks::client::WebhooksClient,
    },
    models::generic::{DfnsApiClientOptions, DfnsBaseApiOptions},
    signer::CredentialSigner,
};
use std::sync::Arc;

pub struct DfnsApiClient {
    base_options: DfnsBaseApiOptions,
    signer: Option<Arc<dyn CredentialSigner>>,
}

impl DfnsApiClient {
    pub fn new(
        base_options: DfnsBaseApiOptions,
        signer: Option<Arc<dyn CredentialSigner>>,
    ) -> Self {
        Self {
            base_options,
            signer,
        }
    }

    fn get_api_options(&self) -> DfnsApiClientOptions {
        DfnsApiClientOptions {
            base: self.base_options.clone(),
            signer: self.signer.as_ref().map(|s| {
                let cloned = Arc::clone(s);
                Box::new(ClonedSigner(cloned)) as Box<dyn CredentialSigner>
            }),
        }
    }

    pub fn auth(&self) -> AuthClient {
        AuthClient::new(self.get_api_options())
    }

    pub fn exchanges(&self) -> ExchangesClient {
        ExchangesClient::new(self.get_api_options())
    }

    pub fn networks(&self) -> NetworksClient {
        NetworksClient::new(self.get_api_options())
    }

    pub fn permissions(&self) -> PermissionsClient {
        PermissionsClient::new(self.get_api_options())
    }

    pub fn policies(&self) -> PoliciesClient {
        PoliciesClient::new(self.get_api_options())
    }

    pub fn staking(&self) -> StakingClient {
        StakingClient::new(self.get_api_options())
    }

    pub fn signers(&self) -> SignersClient {
        SignersClient::new(self.get_api_options())
    }

    pub fn wallets(&self) -> WalletsClient {
        WalletsClient::new(self.get_api_options())
    }

    pub fn webhooks(&self) -> WebhooksClient {
        WebhooksClient::new(self.get_api_options())
    }
}

struct ClonedSigner(Arc<dyn CredentialSigner + Send + Sync>);

#[async_trait::async_trait]
impl CredentialSigner for ClonedSigner {
    async fn sign(
        &self,
        challenge: crate::signer::UserActionChallenge,
    ) -> Result<crate::signer::FirstFactorAssertion, crate::error::DfnsError> {
        self.0.sign(challenge).await
    }
}
