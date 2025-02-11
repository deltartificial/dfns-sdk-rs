// @dfns-sdk-rs/src/api/staking/client.rs

use super::types::*;
use crate::{
    models::generic::DfnsApiClientOptions,
    utils::{
        fetch::simple_fetch,
        url::{build_path_and_query, PathAndQueryParams},
        user_action_fetch::user_action_fetch,
    },
};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct StakingClient {
    api_options: DfnsApiClientOptions,
}

impl StakingClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn create_stake(
        &self,
        request: CreateStakeRequest,
    ) -> Result<CreateStakeResponse, crate::error::DfnsError> {
        let path = build_path_and_query(
            "/staking/stakes",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn create_stake_action(
        &self,
        request: CreateStakeActionRequest,
    ) -> Result<CreateStakeActionResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("stakeId".to_string(), request.stake_id.clone());

        let path = build_path_and_query(
            "/staking/stakes/:stakeId/actions",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn get_stake_rewards(
        &self,
        request: GetStakeRewardsRequest,
    ) -> Result<GetStakeRewardsResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("stakeId".to_string(), request.stake_id.clone());

        let path = build_path_and_query(
            "/staking/stakes/:stakeId/rewards",
            &PathAndQueryParams {
                path: path_params,
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

    pub async fn list_stake_actions(
        &self,
        request: Option<ListStakeActionsRequest>,
    ) -> Result<ListStakeActionsResponse, crate::error::DfnsError> {
        let mut query_params = HashMap::new();
        if let Some(req) = request {
            if let Some(query) = req.query {
                if let Some(limit) = query.limit {
                    query_params.insert("limit".to_string(), limit.to_string());
                }
                if let Some(token) = query.pagination_token {
                    query_params.insert("paginationToken".to_string(), token);
                }
            }
        }

        let path = build_path_and_query(
            "/staking/stakes/:stakeId/actions",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: query_params,
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

    pub async fn list_stakes(
        &self,
        request: Option<ListStakesRequest>,
    ) -> Result<ListStakesResponse, crate::error::DfnsError> {
        let mut query_params = HashMap::new();
        if let Some(req) = request {
            if let Some(query) = req.query {
                if let Some(limit) = query.limit {
                    query_params.insert("limit".to_string(), limit.to_string());
                }
                if let Some(token) = query.pagination_token {
                    query_params.insert("paginationToken".to_string(), token);
                }
            }
        }

        let path = build_path_and_query(
            "/staking/stakes",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: query_params,
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
