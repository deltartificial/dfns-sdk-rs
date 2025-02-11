// @dfns-sdk-rs/src/api/policies/client.rs

use super::types::*;
use crate::{
    error::DfnsError,
    models::generic::DfnsApiClientOptions,
    utils::{fetch::simple_fetch, url::build_path_and_query, user_action_fetch::user_action_fetch},
};
use std::collections::HashMap;

pub struct PoliciesClient {
    api_options: DfnsApiClientOptions,
}

impl PoliciesClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn archive_policy(
        &self,
        request: ArchivePolicyRequest,
    ) -> Result<ArchivePolicyResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies/:policy_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("policy_id".to_string(), request.policy_id);
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

    pub async fn create_approval_decision(
        &self,
        request: CreateApprovalDecisionRequest,
    ) -> Result<CreateApprovalDecisionResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policy-approvals/:approval_id/decisions",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("approval_id".to_string(), request.approval_id);
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

    pub async fn create_policy(
        &self,
        request: CreatePolicyRequest,
    ) -> Result<CreatePolicyResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies",
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

    pub async fn get_approval(
        &self,
        request: GetApprovalRequest,
    ) -> Result<GetApprovalResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policy-approvals/:approval_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("approval_id".to_string(), request.approval_id);
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

    pub async fn get_policy(
        &self,
        request: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies/:policy_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("policy_id".to_string(), request.policy_id);
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

    pub async fn list_approvals(
        &self,
        request: Option<ListApprovalsRequest>,
    ) -> Result<ListApprovalsResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policy-approvals",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(pagination_token) = q.pagination_token {
                            map.insert("pageToken".to_string(), pagination_token);
                        }
                        if let Some(approver_id) = q.approver_id {
                            map.insert("approverId".to_string(), approver_id);
                        }
                        if let Some(initiator_id) = q.initiator_id {
                            map.insert("initiatorId".to_string(), initiator_id);
                        }
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(status) = q.status {
                            map.insert("status".to_string(), format!("{:?}", status));
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

    pub async fn list_policies(
        &self,
        request: Option<ListPoliciesRequest>,
    ) -> Result<ListPoliciesResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(pagination_token) = q.pagination_token {
                            map.insert("pageToken".to_string(), pagination_token);
                        }
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(status) = q.status {
                            map.insert("status".to_string(), format!("{:?}", status));
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

    pub async fn update_policy(
        &self,
        request: UpdatePolicyRequest,
    ) -> Result<UpdatePolicyResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies/:policy_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("policy_id".to_string(), request.policy_id);
                    map
                },
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::PUT,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }
}
