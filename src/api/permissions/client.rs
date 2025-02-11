// @dfns-sdk-rs/src/api/permissions/client.rs

use super::types::*;
use crate::{
    error::DfnsError,
    models::generic::DfnsApiClientOptions,
    utils::{fetch::simple_fetch, url::build_path_and_query, user_action_fetch::user_action_fetch},
};
use std::collections::HashMap;

pub struct PermissionsClient {
    api_options: DfnsApiClientOptions,
}

impl PermissionsClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn archive_permission(
        &self,
        request: ArchivePermissionRequest,
    ) -> Result<ArchivePermissionResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permission_id/archive",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permission_id".to_string(), request.permission_id);
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

    pub async fn create_assignment(
        &self,
        request: CreateAssignmentRequest,
    ) -> Result<CreateAssignmentResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permission_id/assignments",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permission_id".to_string(), request.permission_id);
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

    pub async fn create_permission(
        &self,
        request: CreatePermissionRequest,
    ) -> Result<CreatePermissionResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions",
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

    pub async fn delete_assignment(
        &self,
        request: DeleteAssignmentRequest,
    ) -> Result<DeleteAssignmentResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permission_id/assignments/:assignment_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permission_id".to_string(), request.permission_id);
                    map.insert("assignment_id".to_string(), request.assignment_id);
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

    pub async fn get_permission(
        &self,
        request: GetPermissionRequest,
    ) -> Result<GetPermissionResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permission_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permission_id".to_string(), request.permission_id);
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

    pub async fn list_assignments(
        &self,
        request: ListAssignmentsRequest,
    ) -> Result<ListAssignmentsResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permission_id/assignments",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permission_id".to_string(), request.permission_id);
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

    pub async fn list_permissions(
        &self,
        request: Option<ListPermissionsRequest>,
    ) -> Result<ListPermissionsResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
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

    pub async fn update_permission(
        &self,
        request: UpdatePermissionRequest,
    ) -> Result<UpdatePermissionResponse, DfnsError> {
        let path = build_path_and_query(
            "/permissions/:permission_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("permission_id".to_string(), request.permission_id);
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
