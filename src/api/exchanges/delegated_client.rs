// @dfns-sdk-rs/src/api/exchanges/delegated_client.rs

use super::types::*;
use crate::{
    client::{
        base_auth_api::{
            BaseAuthApi, CreateUserActionChallengeRequest, SignUserActionChallengeRequest,
        },
        delegated_api_client::DfnsDelegatedApiClientOptions,
    },
    error::DfnsError,
    signer::UserActionChallenge,
    utils::{
        fetch::{simple_fetch, FetchOptions, HttpMethod},
        url::{build_path_and_query, PathAndQueryParams},
    },
};
use serde_json::json;
use std::collections::HashMap;

pub struct DelegatedExchangesClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedExchangesClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn create_deposit_init(
        &self,
        request: CreateDepositRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/deposits",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map.insert("accountId".to_string(), request.account_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_deposit_complete(
        &self,
        request: CreateDepositRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateDepositResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/deposits",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map.insert("accountId".to_string(), request.account_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_exchange_init(
        &self,
        request: CreateExchangeRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/exchanges",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_exchange_complete(
        &self,
        request: CreateExchangeRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateExchangeResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_withdrawal_init(
        &self,
        request: CreateWithdrawalRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/withdrawals",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map.insert("accountId".to_string(), request.account_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_withdrawal_complete(
        &self,
        request: CreateWithdrawalRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateWithdrawalResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/withdrawals",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map.insert("accountId".to_string(), request.account_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn delete_exchange_init(
        &self,
        request: DeleteExchangeRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::DELETE,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn delete_exchange_complete(
        &self,
        request: DeleteExchangeRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DeleteExchangeResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::DELETE,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
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
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map
                },
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

    pub async fn list_account_assets(
        &self,
        request: ListAccountAssetsRequest,
    ) -> Result<ListAccountAssetsResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/assets",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map.insert("accountId".to_string(), request.account_id.clone());
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
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

    pub async fn list_accounts(
        &self,
        request: ListAccountsRequest,
    ) -> Result<ListAccountsResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
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

    pub async fn list_asset_withdrawal_networks(
        &self,
        request: ListAssetWithdrawalNetworksRequest,
    ) -> Result<ListAssetWithdrawalNetworksResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges/:exchangeId/accounts/:accountId/assets/:asset/withdrawal-networks",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("exchangeId".to_string(), request.exchange_id.clone());
                    map.insert("accountId".to_string(), request.account_id.clone());
                    map.insert("asset".to_string(), request.asset.clone());
                    map
                },
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

    pub async fn list_exchanges(
        &self,
        request: Option<ListExchangesRequest>,
    ) -> Result<ListExchangesResponse, DfnsError> {
        let path = build_path_and_query(
            "/exchanges",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
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
}
