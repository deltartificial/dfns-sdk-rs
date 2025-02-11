// @dfns-sdk-rs/src/api/signers/client.rs

use super::types::*;
use crate::{
    models::generic::DfnsApiClientOptions,
    utils::{
        fetch::simple_fetch,
        url::{build_path_and_query, PathAndQueryParams},
    },
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SignersClient {
    api_options: DfnsApiClientOptions,
}

impl SignersClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn list_signers(&self) -> Result<ListSignersResponse, crate::error::DfnsError> {
        let path = build_path_and_query(
            "/signers",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
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
}
