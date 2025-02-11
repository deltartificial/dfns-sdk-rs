// @dfns-sdk-rs/src/client/delegated_api_client.rs

use crate::models::generic::DfnsBaseApiOptions;

#[derive(Debug, Clone)]
pub struct DfnsDelegatedApiClientOptions {
    pub base: DfnsBaseApiOptions,
}
