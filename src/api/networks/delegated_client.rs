// @dfns-sdk-rs/src/api/networks/delegated_client.rs

use super::types::*;
use crate::{
    client::delegated_api_client::DfnsDelegatedApiClientOptions,
    error::DfnsError,
    utils::{
        fetch::{simple_fetch, FetchOptions, HttpMethod},
        url::{build_path_and_query, PathAndQueryParams},
    },
};
use serde_json::json;
use std::collections::HashMap;

pub struct DelegatedNetworksClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedNetworksClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn get_fees(
        &self,
        request: Option<GetFeesRequest>,
    ) -> Result<GetFeesResponse, DfnsError> {
        let path = build_path_and_query(
            "/networks/fees",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        map.insert("network".to_string(), q.network.to_string());
                        map
                    })
                    .unwrap_or_default(),
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

    pub async fn read_contract(
        &self,
        request: ReadContractRequest,
    ) -> Result<ReadContractResponse, DfnsError> {
        let path = build_path_and_query(
            "/networks/read-contract",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }
}
