// @dfns-sdk-rs/src/api/permissions/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OperationKind {
    Create,
    Delete,
    Update,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChangeRequestStatus {
    Applied,
    Failed,
    Pending,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PermissionOperation {
    #[serde(rename = "Auth:Action:Sign")]
    AuthActionSign,
    #[serde(rename = "Auth:Apps:Create")]
    AuthAppsCreate,
    #[serde(rename = "Auth:Apps:Read")]
    AuthAppsRead,
    #[serde(rename = "Auth:Apps:Update")]
    AuthAppsUpdate,
    #[serde(rename = "Auth:Creds:Create")]
    AuthCredsCreate,
    #[serde(rename = "Auth:Users:Update")]
    AuthUsersUpdate,
    #[serde(rename = "Wallets:Create")]
    WalletsCreate,
    #[serde(rename = "Wallets:Read")]
    WalletsRead,
    #[serde(rename = "Wallets:Update")]
    WalletsUpdate,
    // Add other operations as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requester {
    pub user_id: String,
    pub token_id: Option<String>,
    pub app_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionChangeRequest {
    pub id: String,
    pub requester: Requester,
    pub status: ChangeRequestStatus,
    pub entity_id: String,
    pub date_created: String,
    pub date_resolved: Option<String>,
    pub approval_id: Option<String>,
    pub kind: String,
    pub operation_kind: OperationKind,
    pub body: PermissionBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionBody {
    pub id: String,
    pub name: String,
    pub status: String,
    pub operations: Vec<String>,
    pub is_immutable: bool,
    pub is_archived: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub operations: Vec<String>,
    pub status: String,
    pub is_immutable: bool,
    pub is_archived: bool,
    pub date_created: String,
    pub date_updated: String,
    pub pending_change_request: Option<PermissionChangeRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivePermissionRequest {
    pub permission_id: String,
    pub is_archived: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAssignmentRequest {
    pub permission_id: String,
    pub identity_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub id: String,
    pub permission_id: String,
    pub identity_id: String,
    pub is_immutable: bool,
    pub date_created: String,
    pub date_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub operations: Vec<PermissionOperation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPermissionsRequest {
    pub limit: Option<String>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPermissionsResponse {
    pub items: Vec<Permission>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermissionRequest {
    pub permission_id: String,
    pub name: Option<String>,
    pub operations: Option<Vec<PermissionOperation>>,
}