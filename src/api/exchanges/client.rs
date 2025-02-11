// @dfns-sdk-rs/src/api/exchanges/client.rs

use super::types::*;
use crate::{
    error::DfnsError,
    models::generic::DfnsApiClientOptions,
    utils::{fetch::simple_fetch, url::build_path_and_query, user_action_fetch::user_action_fetch},
};
use std::collections::HashMap;

pub struct ExchangesClient {
    api_options: DfnsApiClientOptions,
}

impl ExchangesClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn create_deposit(
        &self,
        request: CreateDepositRequest,
    ) -> Result<CreateDepositResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/deposits",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id);
                    map.insert("accountId".to_string(), request.account_id);
                    map
                },
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

    pub async fn create_exchange(
        &self,
        request: CreateExchangeRequest,
    ) -> Result<CreateExchangeResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges",
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

    pub async fn create_withdrawal(
        &self,
        request: CreateWithdrawalRequest,
    ) -> Result<CreateWithdrawalResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/withdrawals",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id);
                    map.insert("accountId".to_string(), request.account_id);
                    map
                },
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

    pub async fn delete_exchange(
        &self,
        request: DeleteExchangeRequest,
    ) -> Result<DeleteExchangeResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id);
                    map
                },
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::DELETE,
                headers: None,
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn get_exchange(
        &self,
        request: GetExchangeRequest,
    ) -> Result<GetExchangeResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id);
                    map
                },
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

    pub async fn list_account_assets(
        &self,
        request: ListAccountAssetsRequest,
    ) -> Result<ListAccountAssetsResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/assets",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id);
                    map.insert("accountId".to_string(), request.account_id);
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(pagination_token) = q.pagination_token {
                            map.insert("pageToken".to_string(), pagination_token);
                        }
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

    pub async fn list_accounts(
        &self,
        request: ListAccountsRequest,
    ) -> Result<ListAccountsResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id);
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(pagination_token) = q.pagination_token {
                            map.insert("pageToken".to_string(), pagination_token);
                        }
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

    pub async fn list_asset_withdrawal_networks(
        &self,
        request: ListAssetWithdrawalNetworksRequest,
    ) -> Result<ListAssetWithdrawalNetworksResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/assets/:asset/withdrawal-networks",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id);
                    map.insert("accountId".to_string(), request.account_id);
                    map.insert("asset".to_string(), request.asset);
                    map
                },
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

    pub async fn list_exchanges(
        &self,
        request: Option<ListExchangesRequest>,
    ) -> Result<ListExchangesResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(pagination_token) = q.pagination_token {
                            map.insert("pageToken".to_string(), pagination_token);
                        }
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
}
