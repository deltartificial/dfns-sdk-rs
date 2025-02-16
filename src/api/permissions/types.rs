// @dfns-sdk-rs/src/api/permissions/types.rs

use serde::{Deserialize, Serialize};

pub type DeleteAssignmentResponse = Option<serde_json::Value>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePermissionBody {
    pub is_archived: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePermissionParams {
    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePermissionResponse {
    pub date_created: String,

    pub date_updated: String,

    pub id: String,

    pub is_archived: bool,

    pub is_immutable: bool,

    pub name: String,

    pub operations: Vec<String>,

    pub status: ArchivePermissionResponseStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArchivePermissionResponseStatus {
    #[default]
    #[serde(rename = "*")]
    None,

    Active,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePermissionRequest {
    pub body: ArchivePermissionRequestBody,

    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePermissionRequestBody {
    pub is_archived: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssignmentBody {
    pub identity_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssignmentParams {
    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssignmentResponse {
    pub date_created: String,

    pub date_updated: String,

    pub id: String,

    pub identity_id: String,

    pub is_immutable: bool,

    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssignmentRequest {
    pub body: CreateAssignmentRequestBody,

    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssignmentRequestBody {
    pub identity_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePermissionBody {
    pub name: String,

    pub operations: Vec<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "Auth:Action:Sign")]
    AuthActionSign,

    #[serde(rename = "Auth:Apps:Create")]
    AuthAppsCreate,

    #[serde(rename = "Auth:Apps:Read")]
    AuthAppsRead,

    #[serde(rename = "Auth:Apps:Update")]
    AuthAppsUpdate,

    #[serde(rename = "Auth:Creds:Code:Create")]
    AuthCredsCodeCreate,

    #[serde(rename = "Auth:Creds:Create")]
    AuthCredsCreate,

    #[serde(rename = "Auth:Creds:Read")]
    AuthCredsRead,

    #[serde(rename = "Auth:Creds:Update")]
    AuthCredsUpdate,

    #[serde(rename = "Auth:Types:Application")]
    AuthTypesApplication,

    #[serde(rename = "Auth:Types:Employee")]
    AuthTypesEmployee,

    #[serde(rename = "Auth:Types:EndUser")]
    AuthTypesEndUser,

    #[serde(rename = "Auth:Types:Pat")]
    AuthTypesPat,

    #[serde(rename = "Auth:Types:ServiceAccount")]
    AuthTypesServiceAccount,

    #[serde(rename = "Auth:Users:Create")]
    AuthUsersCreate,

    #[serde(rename = "Auth:Users:Delegate")]
    AuthUsersDelegate,

    #[serde(rename = "Auth:Users:Read")]
    AuthUsersRead,

    #[serde(rename = "Auth:Users:Update")]
    AuthUsersUpdate,

    #[serde(rename = "Exchanges:Create")]
    ExchangesCreate,

    #[serde(rename = "Exchanges:Delete")]
    ExchangesDelete,

    #[serde(rename = "Exchanges:Deposits:Create")]
    ExchangesDepositsCreate,

    #[serde(rename = "Exchanges:Read")]
    ExchangesRead,

    #[serde(rename = "Exchanges:Withdrawals:Create")]
    ExchangesWithdrawalsCreate,

    #[serde(rename = "Orgs:Read")]
    OrgsRead,

    #[serde(rename = "Orgs:Settings:Read")]
    OrgsSettingsRead,

    #[serde(rename = "Orgs:Settings:Update")]
    OrgsSettingsUpdate,

    #[serde(rename = "Orgs:Update")]
    OrgsUpdate,

    #[serde(rename = "PermissionAssignments:Create")]
    PermissionAssignmentsCreate,

    #[serde(rename = "PermissionAssignments:Read")]
    PermissionAssignmentsRead,

    #[serde(rename = "PermissionAssignments:Revoke")]
    PermissionAssignmentsRevoke,

    #[serde(rename = "PermissionPredicates:Archive")]
    PermissionPredicatesArchive,

    #[serde(rename = "PermissionPredicates:Create")]
    PermissionPredicatesCreate,

    #[serde(rename = "PermissionPredicates:Read")]
    PermissionPredicatesRead,

    #[serde(rename = "PermissionPredicates:Update")]
    PermissionPredicatesUpdate,

    #[serde(rename = "Permissions:Archive")]
    PermissionsArchive,

    #[serde(rename = "Permissions:Create")]
    PermissionsCreate,

    #[serde(rename = "Permissions:Read")]
    PermissionsRead,

    #[serde(rename = "Permissions:Update")]
    PermissionsUpdate,

    #[serde(rename = "Policies:Approvals:Approve")]
    PoliciesApprovalsApprove,

    #[serde(rename = "Policies:Approvals:Read")]
    PoliciesApprovalsRead,

    #[serde(rename = "Policies:Archive")]
    PoliciesArchive,

    #[serde(rename = "Policies:Create")]
    PoliciesCreate,

    #[serde(rename = "Policies:Read")]
    PoliciesRead,

    #[serde(rename = "Policies:Update")]
    PoliciesUpdate,

    #[serde(rename = "Signers:ListSigners")]
    SignersListSigners,

    #[serde(rename = "Stakes:Create")]
    StakesCreate,

    #[serde(rename = "Stakes:Read")]
    StakesRead,

    #[serde(rename = "Stakes:Update")]
    StakesUpdate,

    #[serde(rename = "Wallets:BroadcastTransaction")]
    WalletsBroadcastTransaction,

    #[serde(rename = "Wallets:Create")]
    WalletsCreate,

    #[serde(rename = "Wallets:Delegate")]
    WalletsDelegate,

    #[serde(rename = "Wallets:Export")]
    WalletsExport,

    #[serde(rename = "Wallets:GenerateSignature")]
    WalletsGenerateSignature,

    #[serde(rename = "Wallets:Import")]
    WalletsImport,

    #[serde(rename = "Wallets:Read")]
    WalletsRead,

    #[serde(rename = "Wallets:ReadSignature")]
    WalletsReadSignature,

    #[serde(rename = "Wallets:ReadTransaction")]
    WalletsReadTransaction,

    #[serde(rename = "Wallets:ReadTransfer")]
    WalletsReadTransfer,

    #[serde(rename = "Wallets:Tags:Add")]
    WalletsTagsAdd,

    #[serde(rename = "Wallets:Tags:Delete")]
    WalletsTagsDelete,

    #[serde(rename = "Wallets:TransferAsset")]
    WalletsTransferAsset,

    #[serde(rename = "Wallets:Update")]
    WalletsUpdate,

    #[serde(rename = "Webhooks:Create")]
    WebhooksCreate,

    #[serde(rename = "Webhooks:Delete")]
    WebhooksDelete,

    #[serde(rename = "Webhooks:Events:Read")]
    WebhooksEventsRead,

    #[serde(rename = "Webhooks:Ping")]
    WebhooksPing,

    #[serde(rename = "Webhooks:Read")]
    WebhooksRead,

    #[serde(rename = "Webhooks:Update")]
    WebhooksUpdate,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePermissionResponse {
    pub date_created: String,

    pub date_updated: String,

    pub id: String,

    pub is_archived: bool,

    pub is_immutable: bool,

    pub name: String,

    pub operations: Vec<String>,

    pub status: ArchivePermissionResponseStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    pub body: CreatePermissionRequestBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePermissionRequestBody {
    pub name: String,

    pub operations: Vec<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAssignmentParams {
    pub assignment_id: String,

    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAssignmentRequest {
    pub assignment_id: String,

    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionParams {
    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionResponse {
    pub date_created: String,

    pub date_updated: String,

    pub id: String,

    pub is_archived: bool,

    pub is_immutable: bool,

    pub name: String,

    pub operations: Vec<String>,

    pub pending_change_request: Option<GetPermissionResponsePendingChangeRequest>,

    pub status: ArchivePermissionResponseStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionResponsePendingChangeRequest {
    pub approval_id: Option<String>,

    pub body: PurpleBody,

    pub date_created: String,

    pub date_resolved: Option<String>,

    pub entity_id: String,

    pub id: String,

    pub kind: PurpleKind,

    pub operation_kind: PurpleOperationKind,

    pub requester: PurpleRequester,

    pub status: PendingChangeRequestStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleBody {
    pub id: String,

    pub is_archived: bool,

    pub is_immutable: bool,

    pub name: String,

    pub operations: Vec<String>,

    pub status: ArchivePermissionResponseStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PurpleKind {
    Permission,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PurpleOperationKind {
    Update,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PendingChangeRequestStatus {
    Applied,

    Failed,

    Pending,

    Rejected,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionRequest {
    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssignmentsParams {
    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssignmentsResponse {
    pub items: Vec<ListAssignmentsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssignmentsResponseItem {
    pub date_created: String,

    pub date_updated: String,

    pub id: String,

    pub identity_id: String,

    pub is_immutable: bool,

    pub pending_change_request: Option<PurplePendingChangeRequest>,

    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurplePendingChangeRequest {
    pub approval_id: Option<String>,

    pub body: FluffyBody,

    pub date_created: String,

    pub date_resolved: Option<String>,

    pub entity_id: String,

    pub id: String,

    pub kind: FluffyKind,

    pub operation_kind: FluffyOperationKind,

    pub requester: FluffyRequester,

    pub status: PendingChangeRequestStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyBody {
    pub id: String,

    pub identity_id: String,

    pub is_immutable: bool,

    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FluffyKind {
    Assignment,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FluffyOperationKind {
    Create,

    Delete,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssignmentsRequest {
    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPermissionsQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPermissionsResponse {
    pub items: Vec<ListPermissionsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPermissionsResponseItem {
    pub date_created: String,

    pub date_updated: String,

    pub id: String,

    pub is_archived: bool,

    pub is_immutable: bool,

    pub name: String,

    pub operations: Vec<String>,

    pub pending_change_request: Option<FluffyPendingChangeRequest>,

    pub status: ArchivePermissionResponseStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyPendingChangeRequest {
    pub approval_id: Option<String>,

    pub body: TentacledBody,

    pub date_created: String,

    pub date_resolved: Option<String>,

    pub entity_id: String,

    pub id: String,

    pub kind: PurpleKind,

    pub operation_kind: PurpleOperationKind,

    pub requester: TentacledRequester,

    pub status: PendingChangeRequestStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledBody {
    pub id: String,

    pub is_archived: bool,

    pub is_immutable: bool,

    pub name: String,

    pub operations: Vec<String>,

    pub status: ArchivePermissionResponseStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListPermissionsRequest {
    pub query: Option<Query>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePermissionBody {
    pub name: Option<String>,

    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePermissionParams {
    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePermissionResponse {
    pub date_created: String,

    pub date_updated: String,

    pub id: String,

    pub is_archived: bool,

    pub is_immutable: bool,

    pub name: String,

    pub operations: Vec<String>,

    pub status: ArchivePermissionResponseStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePermissionRequest {
    pub body: UpdatePermissionRequestBody,

    pub permission_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePermissionRequestBody {
    pub name: Option<String>,

    pub operations: Option<Vec<Operation>>,
}
