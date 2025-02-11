// @dfns-sdk-rs/src/api/policies/delegated_client.rs

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

pub struct DelegatedPoliciesClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedPoliciesClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn archive_policy_init(
        &self,
        request: ArchivePolicyRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies/:policyId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("policyId".to_string(), request.policy_id.clone());
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

    pub async fn archive_policy_complete(
        &self,
        request: ArchivePolicyRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ArchivePolicyResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies/:policyId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("policyId".to_string(), request.policy_id.clone());
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

    pub async fn create_approval_decision_init(
        &self,
        request: CreateApprovalDecisionRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/v2/policy-approvals/:approvalId/decisions",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("approvalId".to_string(), request.approval_id.clone());
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

    pub async fn create_approval_decision_complete(
        &self,
        request: CreateApprovalDecisionRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateApprovalDecisionResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policy-approvals/:approvalId/decisions",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("approvalId".to_string(), request.approval_id.clone());
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

    pub async fn create_policy_init(
        &self,
        request: CreatePolicyRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies",
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

    pub async fn create_policy_complete(
        &self,
        request: CreatePolicyRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreatePolicyResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies",
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

    pub async fn get_approval(
        &self,
        request: GetApprovalRequest,
    ) -> Result<GetApprovalResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policy-approvals/:approvalId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("approvalId".to_string(), request.approval_id.clone());
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

    pub async fn get_policy(
        &self,
        request: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies/:policyId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("policyId".to_string(), request.policy_id.clone());
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

    pub async fn list_approvals(
        &self,
        request: Option<ListApprovalsRequest>,
    ) -> Result<ListApprovalsResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policy-approvals",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(approver_id) = q.approver_id {
                            map.insert("approverId".to_string(), approver_id);
                        }
                        if let Some(initiator_id) = q.initiator_id {
                            map.insert("initiatorId".to_string(), initiator_id);
                        }
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit);
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
                        if let Some(status) = q.status {
                            map.insert("status".to_string(), status.to_string());
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

    pub async fn list_policies(
        &self,
        request: Option<ListPoliciesRequest>,
    ) -> Result<ListPoliciesResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit);
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
                        if let Some(status) = q.status {
                            map.insert("status".to_string(), status.to_string());
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

    pub async fn update_policy_init(
        &self,
        request: UpdatePolicyRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies/:policyId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("policyId".to_string(), request.policy_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn update_policy_complete(
        &self,
        request: UpdatePolicyRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<UpdatePolicyResponse, DfnsError> {
        let path = build_path_and_query(
            "/v2/policies/:policyId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("policyId".to_string(), request.policy_id.clone());
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
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }
}
