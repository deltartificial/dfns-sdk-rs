// @dfns-sdk-rs/src/api/staking/delegated_client.rs

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

pub struct DelegatedStakingClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedStakingClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn create_stake_init(
        &self,
        request: CreateStakeRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/staking/stakes",
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

    pub async fn create_stake_complete(
        &self,
        request: CreateStakeRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateStakeResponse, DfnsError> {
        let path = build_path_and_query(
            "/staking/stakes",
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

    pub async fn create_stake_action_init(
        &self,
        request: CreateStakeActionRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/staking/stakes/:stakeId/actions",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("stakeId".to_string(), request.stake_id.clone());
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

    pub async fn create_stake_action_complete(
        &self,
        request: CreateStakeActionRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateStakeActionResponse, DfnsError> {
        let path = build_path_and_query(
            "/staking/stakes/:stakeId/actions",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("stakeId".to_string(), request.stake_id.clone());
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

    pub async fn get_stake_rewards(
        &self,
        request: GetStakeRewardsRequest,
    ) -> Result<GetStakeRewardsResponse, DfnsError> {
        let path = build_path_and_query(
            "/staking/stakes/:stakeId/rewards",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("stakeId".to_string(), request.stake_id.clone());
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

    pub async fn list_stake_actions(
        &self,
        stake_id: String,
        request: Option<ListStakeActionsRequest>,
    ) -> Result<ListStakeActionsResponse, DfnsError> {
        let path = build_path_and_query(
            "/staking/stakes/:stakeId/actions",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("stakeId".to_string(), stake_id);
                    map
                },
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

    pub async fn list_stakes(
        &self,
        request: Option<ListStakesRequest>,
    ) -> Result<ListStakesResponse, DfnsError> {
        let path = build_path_and_query(
            "/staking/stakes",
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
