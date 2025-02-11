// @dfns-sdk-rs/src/api/signers/delegated_client.rs

use super::types::*;
use crate::{
    client::delegated_api_client::DfnsDelegatedApiClientOptions,
    error::DfnsError,
    utils::{
        fetch::{simple_fetch, FetchOptions, HttpMethod},
        url::{build_path_and_query, PathAndQueryParams},
    },
};
use std::collections::HashMap;

pub struct DelegatedSignersClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedSignersClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn list_signers(&self) -> Result<ListSignersResponse, DfnsError> {
        let path = build_path_and_query(
            "/signers",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }
}
