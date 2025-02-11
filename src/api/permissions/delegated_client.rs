// @dfns-sdk-rs/src/api/permissions/delegated_client.rs

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

pub struct DelegatedPermissionsClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedPermissionsClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn archive_permission_init(
        &self,
        request: ArchivePermissionRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
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

    pub async fn archive_permission_complete(
        &self,
        request: ArchivePermissionRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ArchivePermissionResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
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

    pub async fn create_assignment_init(
        &self,
        request: CreateAssignmentRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId/assignments",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
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

    pub async fn create_assignment_complete(
        &self,
        request: CreateAssignmentRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateAssignmentResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId/assignments",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
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

    pub async fn create_permission_init(
        &self,
        request: CreatePermissionRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/permissions",
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

    pub async fn create_permission_complete(
        &self,
        request: CreatePermissionRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreatePermissionResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions",
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

    pub async fn delete_assignment_init(
        &self,
        request: DeleteAssignmentRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId/assignments/:assignmentId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
                    map.insert("assignmentId".to_string(), request.assignment_id.clone());
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

    pub async fn delete_assignment_complete(
        &self,
        request: DeleteAssignmentRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DeleteAssignmentResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId/assignments/:assignmentId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
                    map.insert("assignmentId".to_string(), request.assignment_id.clone());
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

    pub async fn get_permission(
        &self,
        request: GetPermissionRequest,
    ) -> Result<GetPermissionResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
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

    pub async fn list_assignments(
        &self,
        request: ListAssignmentsRequest,
    ) -> Result<ListAssignmentsResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId/assignments",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
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

    pub async fn list_permissions(
        &self,
        request: Option<ListPermissionsRequest>,
    ) -> Result<ListPermissionsResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions",
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

    pub async fn update_permission_init(
        &self,
        request: UpdatePermissionRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
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

    pub async fn update_permission_complete(
        &self,
        request: UpdatePermissionRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<UpdatePermissionResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permissionId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permissionId".to_string(), request.permission_id.clone());
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
