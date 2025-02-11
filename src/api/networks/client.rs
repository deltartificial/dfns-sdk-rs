// @dfns-sdk-rs/src/api/networks/client.rs

use super::types::*;
use crate::{
    error::DfnsError,
    models::generic::DfnsApiClientOptions,
    utils::{fetch::simple_fetch, url::build_path_and_query, user_action_fetch::user_action_fetch},
};
use std::collections::HashMap;

pub struct NetworksClient {
    api_options: DfnsApiClientOptions,
}

impl NetworksClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn get_fees(
        &self,
        request: Option<GetFeesRequest>,
    ) -> Result<GetFeesResponse, DfnsError> {
        let path = build_path_and_query(
            "/networks/fees",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        map.insert("network".to_string(), format!("{:?}", q.network));
                        map
                    })
                    .unwrap_or_default(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }
}
