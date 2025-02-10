// @dfns-sdk-rs/src/api/auth/types.rs

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserKind {
    CustomerEmployee,
    DfnsStaff,
    EndUser,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TokenKind {
    Pat,
    ServiceAccount,
    Token,
    Code,
    Recovery,
    Temp,
    Application,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ApplicationKind {
    ServerSideApplication,
    ClientSideApplication,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionAssignment {
    pub permission_name: String,
    pub permission_id: String,
    pub assignment_id: String,
    pub operations: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessToken {
    pub access_token: Option<String>,
    pub date_created: String,
    pub cred_id: String,
    pub is_active: bool,
    pub kind: TokenKind,
    pub linked_user_id: String,
    pub linked_app_id: String,
    pub name: String,
    pub org_id: String,
    pub permission_assignments: Vec<PermissionAssignment>,
    pub public_key: String,
    pub token_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub name: String,
    pub user_id: String,
    pub kind: UserKind,
    pub credential_uuid: String,
    pub org_id: String,
    pub permissions: Option<Vec<String>>,
    pub is_active: bool,
    pub is_service_account: bool,
    pub is_registered: bool,
    pub permission_assignments: Vec<PermissionAssignment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub app_id: String,
    pub kind: ApplicationKind,
    pub org_id: String,
    pub expected_rp_id: Option<String>,
    pub name: String,
    pub is_active: bool,
    pub expected_origin: Option<String>,
    pub permission_assignments: Vec<PermissionAssignment>,
    pub access_tokens: Vec<AccessToken>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateApplicationRequest {
    pub app_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateApplicationResponse {
    #[serde(flatten)]
    pub application: Application,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateCredentialRequest {
    pub credential_uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateCredentialResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivatePersonalAccessTokenRequest {
    pub token_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivatePersonalAccessTokenResponse {
    #[serde(flatten)]
    pub token: AccessToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateServiceAccountRequest {
    pub service_account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateServiceAccountResponse {
    pub user_info: UserInfo,
    pub access_tokens: Vec<AccessToken>,
}