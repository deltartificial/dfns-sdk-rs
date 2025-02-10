// @dfns-sdk-rs/src/api/auth/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateApplicationParams {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateApplicationResponse {
    pub access_tokens: Vec<ActivateApplicationResponseAccessToken>,

    pub app_id: String,

    pub expected_origin: Option<String>,

    pub expected_rp_id: Option<String>,

    pub is_active: bool,

    pub kind: ActivateApplicationResponseKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<ActivateApplicationResponsePermissionAssignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateApplicationResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PurplePermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessTokenKind {
    Application,

    Code,

    Pat,

    Recovery,

    #[serde(rename = "ServiceAccount")]
    ServiceAccount,

    Temp,

    Token,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurplePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivateApplicationResponseKind {
    #[serde(rename = "ClientSideApplication")]
    ClientSideApplication,

    #[serde(rename = "ServerSideApplication")]
    ServerSideApplication,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateApplicationResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateApplicationRequest {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateCredentialBody {
    pub credential_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateCredentialResponse {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateCredentialRequest {
    pub body: ActivateCredentialRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateCredentialRequestBody {
    pub credential_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivatePersonalAccessTokenParams {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivatePersonalAccessTokenResponse {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<ActivatePersonalAccessTokenResponsePermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivatePersonalAccessTokenResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivatePersonalAccessTokenRequest {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateServiceAccountParams {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateServiceAccountResponse {
    pub access_tokens: Vec<ActivateServiceAccountResponseAccessToken>,

    pub user_info: ActivateServiceAccountResponseUserInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateServiceAccountResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<FluffyPermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateServiceAccountResponseUserInfo {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<TentacledPermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserInfoKind {
    #[serde(rename = "CustomerEmployee")]
    CustomerEmployee,

    #[serde(rename = "DfnsStaff")]
    DfnsStaff,

    #[serde(rename = "EndUser")]
    EndUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateServiceAccountRequest {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateUserParams {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateUserResponse {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<ActivateUserResponsePermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateUserResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateUserRequest {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveApplicationParams {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveApplicationResponse {
    pub access_tokens: Vec<ArchiveApplicationResponseAccessToken>,

    pub app_id: String,

    pub expected_origin: Option<String>,

    pub expected_rp_id: Option<String>,

    pub is_active: bool,

    pub kind: ActivateApplicationResponseKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<ArchiveApplicationResponsePermissionAssignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveApplicationResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<StickyPermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveApplicationResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveApplicationRequest {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePersonalAccessTokenParams {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePersonalAccessTokenResponse {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<ArchivePersonalAccessTokenResponsePermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePersonalAccessTokenResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePersonalAccessTokenRequest {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveServiceAccountParams {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveServiceAccountResponse {
    pub access_tokens: Vec<ArchiveServiceAccountResponseAccessToken>,

    pub user_info: ArchiveServiceAccountResponseUserInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveServiceAccountResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<IndigoPermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveServiceAccountResponseUserInfo {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<IndecentPermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveServiceAccountRequest {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveUserParams {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveUserResponse {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<ArchiveUserResponsePermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveUserResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveUserRequest {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApplicationResponse {
    pub access_tokens: Vec<CreateApplicationResponseAccessToken>,

    pub app_id: String,

    pub expected_origin: Option<String>,

    pub expected_rp_id: Option<String>,

    pub is_active: bool,

    pub kind: ActivateApplicationResponseKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<CreateApplicationResponsePermissionAssignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApplicationResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<HilariousPermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApplicationResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApplicationRequest {
    pub body: CreateApplicationBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApplicationBody {
    pub external_id: Option<String>,

    pub kind: ActivateApplicationResponseKind,

    pub name: String,

    pub origin: Option<String>,

    pub permission_id: Option<String>,

    pub relying_party_id: Option<String>,

    pub days_valid: Option<f64>,

    pub public_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialResponse {
    pub credential_id: String,

    pub credential_uuid: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: CredentialKindElement,

    pub name: String,

    pub origin: String,

    pub public_key: String,

    pub relying_party_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialKindElement {
    Fido2,

    Key,

    Password,

    #[serde(rename = "PasswordProtectedKey")]
    PasswordProtectedKey,

    #[serde(rename = "RecoveryKey")]
    RecoveryKey,

    Totp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialRequest {
    pub body: CreateCredentialBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialBody {
    pub challenge_identifier: String,

    pub credential_info: CreateCredentialBodyCredentialInfo,

    pub credential_kind: CredentialKindElement,

    pub credential_name: String,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialBodyCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeBody {
    pub kind: CredentialKindElement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialChallengeResponse {
    pub challenge_identifier: String,

    pub kind: CredentialKindElement,

    pub rp: Option<CreateCredentialChallengeResponseRp>,

    pub temporary_authentication_token: String,

    pub user: CreateCredentialChallengeResponseUser,

    pub otp_url: Option<String>,

    pub attestation: Option<Attestation>,

    pub authenticator_selection: Option<CreateCredentialChallengeResponseAuthenticatorSelection>,

    pub challenge: Option<String>,

    pub exclude_credentials: Option<Vec<CreateCredentialChallengeResponseExcludeCredential>>,

    pub pub_key_cred_params: Option<Vec<CreateCredentialChallengeResponsePubKeyCredParam>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Attestation {
    Direct,

    Enterprise,

    Indirect,

    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialChallengeResponseAuthenticatorSelection {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    pub require_resident_key: bool,

    pub resident_key: ResidentKey,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthenticatorAttachment {
    #[serde(rename = "cross-platform")]
    CrossPlatform,

    Platform,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidentKey {
    Discouraged,

    Preferred,

    Required,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeResponseExcludeCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub exclude_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Type {
    #[serde(rename = "public-key")]
    PublicKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeResponsePubKeyCredParam {
    pub alg: f64,

    #[serde(rename = "type")]
    pub pub_key_cred_param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialChallengeResponseUser {
    pub display_name: String,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeRequest {
    pub body: CreateCredentialChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeRequestBody {
    pub kind: CredentialKindElement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialChallengeWithCodeBody {
    pub code: String,

    pub credential_kind: CredentialKindElement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialChallengeWithCodeResponse {
    pub challenge_identifier: String,

    pub kind: CredentialKindElement,

    pub rp: Option<CreateCredentialChallengeWithCodeResponseRp>,

    pub temporary_authentication_token: String,

    pub user: CreateCredentialChallengeWithCodeResponseUser,

    pub otp_url: Option<String>,

    pub attestation: Option<Attestation>,

    pub authenticator_selection:
        Option<CreateCredentialChallengeWithCodeResponseAuthenticatorSelection>,

    pub challenge: Option<String>,

    pub exclude_credentials:
        Option<Vec<CreateCredentialChallengeWithCodeResponseExcludeCredential>>,

    pub pub_key_cred_params: Option<Vec<CreateCredentialChallengeWithCodeResponsePubKeyCredParam>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialChallengeWithCodeResponseAuthenticatorSelection {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    pub require_resident_key: bool,

    pub resident_key: ResidentKey,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeWithCodeResponseExcludeCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub exclude_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeWithCodeResponsePubKeyCredParam {
    pub alg: f64,

    #[serde(rename = "type")]
    pub pub_key_cred_param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeWithCodeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialChallengeWithCodeResponseUser {
    pub display_name: String,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialChallengeWithCodeRequest {
    pub body: CreateCredentialChallengeWithCodeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialChallengeWithCodeRequestBody {
    pub code: String,

    pub credential_kind: CredentialKindElement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialCodeBody {
    /// Code expiration, as an ISO-8601 datetime string or a unix timestamp
    pub expiration: Expiration,
}

/// Code expiration, as an ISO-8601 datetime string or a unix timestamp
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expiration {
    Double(f64),

    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialCodeResponse {
    pub code: String,

    pub expiration: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialCodeRequest {
    pub body: CreateCredentialCodeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialCodeRequestBody {
    /// Code expiration, as an ISO-8601 datetime string or a unix timestamp
    pub expiration: Expiration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialWithCodeResponse {
    pub credential_id: String,

    pub credential_uuid: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: CredentialKindElement,

    pub name: String,

    pub origin: String,

    pub public_key: String,

    pub relying_party_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCredentialWithCodeRequest {
    pub body: CreateCredentialWithCodeBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialWithCodeBody {
    pub challenge_identifier: String,

    pub credential_info: CreateCredentialWithCodeBodyCredentialInfo,

    pub credential_kind: CredentialKindElement,

    pub credential_name: String,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialWithCodeBodyCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRecoveryChallengeBody {
    pub credential_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRecoveryChallengeResponse {
    pub allowed_recovery_credentials:
        Vec<CreateDelegatedRecoveryChallengeResponseAllowedRecoveryCredential>,

    pub attestation: Attestation,

    pub authenticator_selection: CreateDelegatedRecoveryChallengeResponseAuthenticatorSelection,

    pub challenge: String,

    pub exclude_credentials: Vec<CreateDelegatedRecoveryChallengeResponseExcludeCredential>,

    pub otp_url: String,

    pub pub_key_cred_params: Vec<CreateDelegatedRecoveryChallengeResponsePubKeyCredParam>,

    pub rp: Option<CreateDelegatedRecoveryChallengeResponseRp>,

    pub supported_credential_kinds:
        CreateDelegatedRecoveryChallengeResponseSupportedCredentialKinds,

    pub temporary_authentication_token: String,

    pub user: CreateDelegatedRecoveryChallengeResponseUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRecoveryChallengeResponseAllowedRecoveryCredential {
    pub encrypted_recovery_key: String,

    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRecoveryChallengeResponseAuthenticatorSelection {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    pub require_resident_key: bool,

    pub resident_key: ResidentKey,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDelegatedRecoveryChallengeResponseExcludeCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub exclude_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDelegatedRecoveryChallengeResponsePubKeyCredParam {
    pub alg: f64,

    #[serde(rename = "type")]
    pub pub_key_cred_param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDelegatedRecoveryChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRecoveryChallengeResponseSupportedCredentialKinds {
    pub first_factor: Vec<CredentialKindElement>,

    pub second_factor: Vec<CredentialKindElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRecoveryChallengeResponseUser {
    pub display_name: String,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDelegatedRecoveryChallengeRequest {
    pub body: CreateDelegatedRecoveryChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRecoveryChallengeRequestBody {
    pub credential_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRegistrationChallengeBody {
    pub email: String,

    pub external_id: Option<String>,

    pub kind: UserInfoKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRegistrationChallengeResponse {
    pub attestation: Attestation,

    pub authenticator_selection: CreateDelegatedRegistrationChallengeResponseAuthenticatorSelection,

    pub challenge: String,

    pub exclude_credentials: Vec<CreateDelegatedRegistrationChallengeResponseExcludeCredential>,

    pub otp_url: String,

    pub pub_key_cred_params: Vec<CreateDelegatedRegistrationChallengeResponsePubKeyCredParam>,

    pub rp: Option<CreateDelegatedRegistrationChallengeResponseRp>,

    pub supported_credential_kinds:
        CreateDelegatedRegistrationChallengeResponseSupportedCredentialKinds,

    pub temporary_authentication_token: String,

    pub user: CreateDelegatedRegistrationChallengeResponseUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRegistrationChallengeResponseAuthenticatorSelection {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    pub require_resident_key: bool,

    pub resident_key: ResidentKey,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDelegatedRegistrationChallengeResponseExcludeCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub exclude_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDelegatedRegistrationChallengeResponsePubKeyCredParam {
    pub alg: f64,

    #[serde(rename = "type")]
    pub pub_key_cred_param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDelegatedRegistrationChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRegistrationChallengeResponseSupportedCredentialKinds {
    pub first_factor: Vec<CredentialKindElement>,

    pub second_factor: Vec<CredentialKindElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRegistrationChallengeResponseUser {
    pub display_name: String,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDelegatedRegistrationChallengeRequest {
    pub body: CreateDelegatedRegistrationChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegatedRegistrationChallengeRequestBody {
    pub email: String,

    pub external_id: Option<String>,

    pub kind: UserInfoKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLoginChallengeBody {
    pub login_code: Option<String>,

    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLoginChallengeResponse {
    pub allow_credentials: CreateLoginChallengeResponseAllowCredentials,

    pub attestation: Attestation,

    pub challenge: String,

    pub challenge_identifier: String,

    pub external_authentication_url: String,

    pub rp: Option<CreateLoginChallengeResponseRp>,

    pub supported_credential_kinds: Vec<CreateLoginChallengeResponseSupportedCredentialKind>,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLoginChallengeResponseAllowCredentials {
    pub key: Vec<PurpleKey>,

    pub password_protected_key: Option<Vec<PurplePasswordProtectedKey>>,

    pub webauthn: Vec<PurpleWebauthn>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleKey {
    pub id: String,

    #[serde(rename = "type")]
    pub key_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurplePasswordProtectedKey {
    pub encrypted_private_key: String,

    pub id: String,

    #[serde(rename = "type")]
    pub password_protected_key_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleWebauthn {
    pub id: String,

    #[serde(rename = "type")]
    pub webauthn_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateLoginChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLoginChallengeResponseSupportedCredentialKind {
    pub factor: Factor,

    pub kind: CredentialKindElement,

    pub requires_second_factor: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Factor {
    Either,

    First,

    Second,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateLoginChallengeRequest {
    pub body: CreateLoginChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLoginChallengeRequestBody {
    pub login_code: Option<String>,

    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersonalAccessTokenBody {
    pub days_valid: Option<f64>,

    pub external_id: Option<String>,

    pub name: String,

    pub permission_id: Option<String>,

    pub public_key: String,

    pub seconds_valid: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersonalAccessTokenResponse {
    pub access_token: String,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<CreatePersonalAccessTokenResponsePermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersonalAccessTokenResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePersonalAccessTokenRequest {
    pub body: CreatePersonalAccessTokenRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersonalAccessTokenRequestBody {
    pub days_valid: Option<f64>,

    pub external_id: Option<String>,

    pub name: String,

    pub permission_id: Option<String>,

    pub public_key: String,

    pub seconds_valid: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecoveryChallengeBody {
    pub credential_id: String,

    pub org_id: String,

    pub username: String,

    pub verification_code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecoveryChallengeResponse {
    pub allowed_recovery_credentials: Vec<CreateRecoveryChallengeResponseAllowedRecoveryCredential>,

    pub attestation: Attestation,

    pub authenticator_selection: CreateRecoveryChallengeResponseAuthenticatorSelection,

    pub challenge: String,

    pub exclude_credentials: Vec<CreateRecoveryChallengeResponseExcludeCredential>,

    pub otp_url: String,

    pub pub_key_cred_params: Vec<CreateRecoveryChallengeResponsePubKeyCredParam>,

    pub rp: Option<CreateRecoveryChallengeResponseRp>,

    pub supported_credential_kinds: CreateRecoveryChallengeResponseSupportedCredentialKinds,

    pub temporary_authentication_token: String,

    pub user: CreateRecoveryChallengeResponseUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecoveryChallengeResponseAllowedRecoveryCredential {
    pub encrypted_recovery_key: String,

    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecoveryChallengeResponseAuthenticatorSelection {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    pub require_resident_key: bool,

    pub resident_key: ResidentKey,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRecoveryChallengeResponseExcludeCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub exclude_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRecoveryChallengeResponsePubKeyCredParam {
    pub alg: f64,

    #[serde(rename = "type")]
    pub pub_key_cred_param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRecoveryChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecoveryChallengeResponseSupportedCredentialKinds {
    pub first_factor: Vec<CredentialKindElement>,

    pub second_factor: Vec<CredentialKindElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecoveryChallengeResponseUser {
    pub display_name: String,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRecoveryChallengeRequest {
    pub body: CreateRecoveryChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecoveryChallengeRequestBody {
    pub credential_id: String,

    pub org_id: String,

    pub username: String,

    pub verification_code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegistrationChallengeBody {
    pub org_id: String,

    pub registration_code: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegistrationChallengeResponse {
    pub attestation: Attestation,

    pub authenticator_selection: CreateRegistrationChallengeResponseAuthenticatorSelection,

    pub challenge: String,

    pub exclude_credentials: Vec<CreateRegistrationChallengeResponseExcludeCredential>,

    pub otp_url: String,

    pub pub_key_cred_params: Vec<CreateRegistrationChallengeResponsePubKeyCredParam>,

    pub rp: Option<CreateRegistrationChallengeResponseRp>,

    pub supported_credential_kinds: CreateRegistrationChallengeResponseSupportedCredentialKinds,

    pub temporary_authentication_token: String,

    pub user: CreateRegistrationChallengeResponseUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegistrationChallengeResponseAuthenticatorSelection {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    pub require_resident_key: bool,

    pub resident_key: ResidentKey,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRegistrationChallengeResponseExcludeCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub exclude_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRegistrationChallengeResponsePubKeyCredParam {
    pub alg: f64,

    #[serde(rename = "type")]
    pub pub_key_cred_param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRegistrationChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegistrationChallengeResponseSupportedCredentialKinds {
    pub first_factor: Vec<CredentialKindElement>,

    pub second_factor: Vec<CredentialKindElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegistrationChallengeResponseUser {
    pub display_name: String,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRegistrationChallengeRequest {
    pub body: CreateRegistrationChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegistrationChallengeRequestBody {
    pub org_id: String,

    pub registration_code: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountBody {
    pub days_valid: Option<f64>,

    pub external_id: Option<String>,

    pub name: String,

    pub permission_id: Option<String>,

    pub public_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountResponse {
    pub access_tokens: Vec<CreateServiceAccountResponseAccessToken>,

    pub user_info: CreateServiceAccountResponseUserInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<AmbitiousPermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountResponseUserInfo {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<CunningPermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateServiceAccountRequest {
    pub body: CreateServiceAccountRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountRequestBody {
    pub days_valid: Option<f64>,

    pub external_id: Option<String>,

    pub name: String,

    pub permission_id: Option<String>,

    pub public_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSocialRegistrationChallengeBody {
    pub id_token: String,

    pub social_login_provider_kind: SocialLoginProviderKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocialLoginProviderKind {
    Oidc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSocialRegistrationChallengeResponse {
    pub attestation: Attestation,

    pub authenticator_selection: CreateSocialRegistrationChallengeResponseAuthenticatorSelection,

    pub challenge: String,

    pub exclude_credentials: Vec<CreateSocialRegistrationChallengeResponseExcludeCredential>,

    pub otp_url: String,

    pub pub_key_cred_params: Vec<CreateSocialRegistrationChallengeResponsePubKeyCredParam>,

    pub rp: Option<CreateSocialRegistrationChallengeResponseRp>,

    pub supported_credential_kinds:
        CreateSocialRegistrationChallengeResponseSupportedCredentialKinds,

    pub temporary_authentication_token: String,

    pub user: CreateSocialRegistrationChallengeResponseUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSocialRegistrationChallengeResponseAuthenticatorSelection {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    pub require_resident_key: bool,

    pub resident_key: ResidentKey,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSocialRegistrationChallengeResponseExcludeCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub exclude_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSocialRegistrationChallengeResponsePubKeyCredParam {
    pub alg: f64,

    #[serde(rename = "type")]
    pub pub_key_cred_param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSocialRegistrationChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSocialRegistrationChallengeResponseSupportedCredentialKinds {
    pub first_factor: Vec<CredentialKindElement>,

    pub second_factor: Vec<CredentialKindElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSocialRegistrationChallengeResponseUser {
    pub display_name: String,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSocialRegistrationChallengeRequest {
    pub body: CreateSocialRegistrationChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSocialRegistrationChallengeRequestBody {
    pub id_token: String,

    pub social_login_provider_kind: SocialLoginProviderKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserBody {
    pub email: String,

    pub external_id: Option<String>,

    pub kind: CreateUserBodyKind,

    pub public_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateUserBodyKind {
    #[serde(rename = "CustomerEmployee")]
    CustomerEmployee,

    #[serde(rename = "DfnsStaff")]
    DfnsStaff,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<CreateUserResponsePermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub body: CreateUserRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequestBody {
    pub email: String,

    pub external_id: Option<String>,

    pub kind: CreateUserBodyKind,

    pub public_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionChallengeBody {
    pub user_action_http_method: String,

    pub user_action_http_path: String,

    pub user_action_payload: String,

    pub user_action_server_kind: Option<UserActionServerKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserActionServerKind {
    Api,

    Staff,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionChallengeResponse {
    pub allow_credentials: CreateUserActionChallengeResponseAllowCredentials,

    pub attestation: Attestation,

    pub challenge: String,

    pub challenge_identifier: String,

    pub external_authentication_url: String,

    pub rp: Option<CreateUserActionChallengeResponseRp>,

    pub supported_credential_kinds: Vec<CreateUserActionChallengeResponseSupportedCredentialKind>,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionChallengeResponseAllowCredentials {
    pub key: Vec<FluffyKey>,

    pub password_protected_key: Option<Vec<FluffyPasswordProtectedKey>>,

    pub webauthn: Vec<FluffyWebauthn>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyKey {
    pub id: String,

    #[serde(rename = "type")]
    pub key_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyPasswordProtectedKey {
    pub encrypted_private_key: String,

    pub id: String,

    #[serde(rename = "type")]
    pub password_protected_key_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyWebauthn {
    pub id: String,

    #[serde(rename = "type")]
    pub webauthn_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserActionChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionChallengeResponseSupportedCredentialKind {
    pub factor: Factor,

    pub kind: CredentialKindElement,

    pub requires_second_factor: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserActionChallengeRequest {
    pub body: CreateUserActionChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionChallengeRequestBody {
    pub user_action_http_method: String,

    pub user_action_http_path: String,

    pub user_action_payload: String,

    pub user_action_server_kind: Option<UserActionServerKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionSignatureBody {
    pub challenge_identifier: String,

    pub first_factor: CreateUserActionSignatureBodyFirstFactor,

    pub second_factor: Option<PurpleSecondFactor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionSignatureBodyFirstFactor {
    pub credential_assertion: Option<PurpleCredentialAssertion>,

    pub kind: FirstFactorKind,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleCredentialAssertion {
    pub algorithm: Option<String>,

    pub authenticator_data: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FirstFactorKind {
    Fido2,

    Key,

    Password,

    #[serde(rename = "PasswordProtectedKey")]
    PasswordProtectedKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleSecondFactor {
    pub credential_assertion: Option<FluffyCredentialAssertion>,

    pub kind: SecondFactorKind,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyCredentialAssertion {
    pub algorithm: Option<String>,

    pub authenticator_data: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecondFactorKind {
    Fido2,

    Key,

    #[serde(rename = "PasswordProtectedKey")]
    PasswordProtectedKey,

    Totp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionSignatureResponse {
    pub user_action: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserActionSignatureRequest {
    pub body: CreateUserActionSignatureRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserActionSignatureRequestBody {
    pub challenge_identifier: String,

    pub first_factor: PurpleFirstFactor,

    pub second_factor: Option<FluffySecondFactor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleFirstFactor {
    pub credential_assertion: Option<TentacledCredentialAssertion>,

    pub kind: FirstFactorKind,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledCredentialAssertion {
    pub algorithm: Option<String>,

    pub authenticator_data: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffySecondFactor {
    pub credential_assertion: Option<StickyCredentialAssertion>,

    pub kind: SecondFactorKind,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyCredentialAssertion {
    pub algorithm: Option<String>,

    pub authenticator_data: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateApplicationParams {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateApplicationResponse {
    pub access_tokens: Vec<DeactivateApplicationResponseAccessToken>,

    pub app_id: String,

    pub expected_origin: Option<String>,

    pub expected_rp_id: Option<String>,

    pub is_active: bool,

    pub kind: ActivateApplicationResponseKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<DeactivateApplicationResponsePermissionAssignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateApplicationResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<MagentaPermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateApplicationResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateApplicationRequest {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateCredentialBody {
    pub credential_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeactivateCredentialResponse {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeactivateCredentialRequest {
    pub body: DeactivateCredentialRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateCredentialRequestBody {
    pub credential_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivatePersonalAccessTokenParams {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivatePersonalAccessTokenResponse {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<DeactivatePersonalAccessTokenResponsePermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivatePersonalAccessTokenResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivatePersonalAccessTokenRequest {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateServiceAccountParams {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateServiceAccountResponse {
    pub access_tokens: Vec<DeactivateServiceAccountResponseAccessToken>,

    pub user_info: DeactivateServiceAccountResponseUserInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateServiceAccountResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<FriskyPermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateServiceAccountResponseUserInfo {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<MischievousPermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateServiceAccountRequest {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateUserParams {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateUserResponse {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<DeactivateUserResponsePermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateUserResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateUserRequest {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatedLoginBody {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatedLoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatedLoginRequest {
    pub body: DelegatedLoginRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatedLoginRequestBody {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApplicationParams {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApplicationResponse {
    pub access_tokens: Vec<GetApplicationResponseAccessToken>,

    pub app_id: String,

    pub expected_origin: Option<String>,

    pub expected_rp_id: Option<String>,

    pub is_active: bool,

    pub kind: ActivateApplicationResponseKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<GetApplicationResponsePermissionAssignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApplicationResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<BraggadociousPermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BraggadociousPermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApplicationResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApplicationRequest {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersonalAccessTokenParams {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersonalAccessTokenResponse {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<GetPersonalAccessTokenResponsePermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersonalAccessTokenResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersonalAccessTokenRequest {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServiceAccountParams {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServiceAccountResponse {
    pub access_tokens: Vec<GetServiceAccountResponseAccessToken>,

    pub user_info: GetServiceAccountResponseUserInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServiceAccountResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment1>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment1 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServiceAccountResponseUserInfo {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment2>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment2 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServiceAccountRequest {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserParams {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResponse {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<GetUserResponsePermissionAssignment>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserRequest {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListApplicationsResponse {
    pub items: Vec<ListApplicationsResponseItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListApplicationsResponseItem {
    pub access_tokens: Vec<PurpleAccessToken>,

    pub app_id: String,

    pub expected_origin: Option<String>,

    pub expected_rp_id: Option<String>,

    pub is_active: bool,

    pub kind: ActivateApplicationResponseKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment4>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment3>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment3 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment4 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListCredentialsResponse {
    pub items: Vec<ListCredentialsResponseItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCredentialsResponseItem {
    pub credential_id: String,

    pub credential_uuid: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: CredentialKindElement,

    pub name: String,

    pub origin: String,

    pub public_key: String,

    pub relying_party_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListPersonalAccessTokensResponse {
    pub items: Vec<ListPersonalAccessTokensResponseItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPersonalAccessTokensResponseItem {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment5>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment5 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListServiceAccountsResponse {
    pub items: Vec<ListServiceAccountsResponseItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListServiceAccountsResponseItem {
    pub access_tokens: Vec<FluffyAccessToken>,

    pub user_info: ItemUserInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment6>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment6 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemUserInfo {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment7>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment7 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersQuery {
    pub kind: Option<ListUsersQueryKind>,

    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListUsersQueryKind {
    #[serde(rename = "CustomerEmployee")]
    CustomerEmployee,

    #[serde(rename = "EndUser")]
    EndUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersResponse {
    pub items: Vec<ListUsersResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersResponseItem {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment8>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment8 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListUsersRequest {
    pub query: Option<Query>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub kind: Option<ListUsersQueryKind>,

    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginBody {
    pub challenge_identifier: String,

    pub first_factor: LoginBodyFirstFactor,

    pub second_factor: Option<TentacledSecondFactor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginBodyFirstFactor {
    pub credential_assertion: Option<IndigoCredentialAssertion>,

    pub kind: FirstFactorKind,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoCredentialAssertion {
    pub algorithm: Option<String>,

    pub authenticator_data: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledSecondFactor {
    pub credential_assertion: Option<IndecentCredentialAssertion>,

    pub kind: SecondFactorKind,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentCredentialAssertion {
    pub algorithm: Option<String>,

    pub authenticator_data: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub body: LoginRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequestBody {
    pub challenge_identifier: String,

    pub first_factor: FluffyFirstFactor,

    pub second_factor: Option<StickySecondFactor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyFirstFactor {
    pub credential_assertion: Option<HilariousCredentialAssertion>,

    pub kind: FirstFactorKind,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousCredentialAssertion {
    pub algorithm: Option<String>,

    pub authenticator_data: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickySecondFactor {
    pub credential_assertion: Option<AmbitiousCredentialAssertion>,

    pub kind: SecondFactorKind,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousCredentialAssertion {
    pub algorithm: Option<String>,

    pub authenticator_data: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub body: LogoutBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoutBody {
    pub all_sessions: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoverBody {
    pub new_credentials: RecoverBodyNewCredentials,

    pub recovery: RecoverBodyRecovery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoverBodyNewCredentials {
    pub first_factor_credential: PurpleFirstFactorCredential,

    pub recovery_credential: Option<PurpleRecoveryCredential>,

    pub second_factor_credential: Option<PurpleSecondFactorCredential>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleFirstFactorCredential {
    pub credential_info: PurpleCredentialInfo,

    pub credential_kind: FirstFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRecoveryCredential {
    pub credential_info: FluffyCredentialInfo,

    pub credential_kind: RecoveryCredentialKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyCredentialInfo {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecoveryCredentialKind {
    #[serde(rename = "RecoveryKey")]
    RecoveryKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleSecondFactorCredential {
    pub credential_info: TentacledCredentialInfo,

    pub credential_kind: SecondFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoverBodyRecovery {
    pub credential_assertion: CunningCredentialAssertion,

    pub kind: RecoveryCredentialKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningCredentialAssertion {
    pub algorithm: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoverResponse {
    pub credential: RecoverResponseCredential,

    pub user: RecoverResponseUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoverResponseCredential {
    pub kind: CredentialKindElement,

    pub name: String,

    pub uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoverResponseUser {
    pub id: String,

    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoverRequest {
    pub body: RecoverRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoverRequestBody {
    pub new_credentials: BodyNewCredentials,

    pub recovery: BodyRecovery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyNewCredentials {
    pub first_factor_credential: FluffyFirstFactorCredential,

    pub recovery_credential: Option<FluffyRecoveryCredential>,

    pub second_factor_credential: Option<FluffySecondFactorCredential>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyFirstFactorCredential {
    pub credential_info: StickyCredentialInfo,

    pub credential_kind: FirstFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRecoveryCredential {
    pub credential_info: IndigoCredentialInfo,

    pub credential_kind: RecoveryCredentialKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoCredentialInfo {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffySecondFactorCredential {
    pub credential_info: IndecentCredentialInfo,

    pub credential_kind: SecondFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyRecovery {
    pub credential_assertion: MagentaCredentialAssertion,

    pub kind: RecoveryCredentialKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaCredentialAssertion {
    pub algorithm: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecreateDelegatedRegistrationChallengeBody {
    pub email: String,

    pub external_id: Option<String>,

    pub kind: UserInfoKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecreateDelegatedRegistrationChallengeResponse {
    pub attestation: Attestation,

    pub authenticator_selection:
        RecreateDelegatedRegistrationChallengeResponseAuthenticatorSelection,

    pub challenge: String,

    pub exclude_credentials: Vec<RecreateDelegatedRegistrationChallengeResponseExcludeCredential>,

    pub otp_url: String,

    pub pub_key_cred_params: Vec<RecreateDelegatedRegistrationChallengeResponsePubKeyCredParam>,

    pub rp: Option<RecreateDelegatedRegistrationChallengeResponseRp>,

    pub supported_credential_kinds:
        RecreateDelegatedRegistrationChallengeResponseSupportedCredentialKinds,

    pub temporary_authentication_token: String,

    pub user: RecreateDelegatedRegistrationChallengeResponseUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecreateDelegatedRegistrationChallengeResponseAuthenticatorSelection {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    pub require_resident_key: bool,

    pub resident_key: ResidentKey,

    pub user_verification: ResidentKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecreateDelegatedRegistrationChallengeResponseExcludeCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub exclude_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecreateDelegatedRegistrationChallengeResponsePubKeyCredParam {
    pub alg: f64,

    #[serde(rename = "type")]
    pub pub_key_cred_param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecreateDelegatedRegistrationChallengeResponseRp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecreateDelegatedRegistrationChallengeResponseSupportedCredentialKinds {
    pub first_factor: Vec<CredentialKindElement>,

    pub second_factor: Vec<CredentialKindElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecreateDelegatedRegistrationChallengeResponseUser {
    pub display_name: String,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecreateDelegatedRegistrationChallengeRequest {
    pub body: RecreateDelegatedRegistrationChallengeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecreateDelegatedRegistrationChallengeRequestBody {
    pub email: String,

    pub external_id: Option<String>,

    pub kind: UserInfoKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBody {
    pub first_factor_credential: RegisterBodyFirstFactorCredential,

    pub recovery_credential: Option<RegisterBodyRecoveryCredential>,

    pub second_factor_credential: Option<TentacledSecondFactorCredential>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBodyFirstFactorCredential {
    pub credential_info: HilariousCredentialInfo,

    pub credential_kind: FirstFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBodyRecoveryCredential {
    pub credential_info: AmbitiousCredentialInfo,

    pub credential_kind: RecoveryCredentialKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousCredentialInfo {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledSecondFactorCredential {
    pub credential_info: CunningCredentialInfo,

    pub credential_kind: SecondFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub credential: RegisterResponseCredential,

    pub user: RegisterResponseUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterResponseCredential {
    pub kind: CredentialKindElement,

    pub name: String,

    pub uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponseUser {
    pub id: String,

    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub body: RegisterRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequestBody {
    pub first_factor_credential: TentacledFirstFactorCredential,

    pub recovery_credential: Option<TentacledRecoveryCredential>,

    pub second_factor_credential: Option<StickySecondFactorCredential>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledFirstFactorCredential {
    pub credential_info: MagentaCredentialInfo,

    pub credential_kind: FirstFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledRecoveryCredential {
    pub credential_info: FriskyCredentialInfo,

    pub credential_kind: RecoveryCredentialKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyCredentialInfo {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickySecondFactorCredential {
    pub credential_info: MischievousCredentialInfo,

    pub credential_kind: SecondFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterEndUserBody {
    pub first_factor_credential: RegisterEndUserBodyFirstFactorCredential,

    pub recovery_credential: Option<RegisterEndUserBodyRecoveryCredential>,

    pub second_factor_credential: Option<IndigoSecondFactorCredential>,

    pub wallets: Vec<RegisterEndUserBodyWallet>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterEndUserBodyFirstFactorCredential {
    pub credential_info: BraggadociousCredentialInfo,

    pub credential_kind: FirstFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BraggadociousCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterEndUserBodyRecoveryCredential {
    pub credential_info: CredentialInfo1,

    pub credential_kind: RecoveryCredentialKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialInfo1 {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoSecondFactorCredential {
    pub credential_info: CredentialInfo2,

    pub credential_kind: SecondFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialInfo2 {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterEndUserBodyWallet {
    pub name: Option<String>,

    pub network: Network,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Network {
    Algorand,

    #[serde(rename = "AlgorandTestnet")]
    AlgorandTestnet,

    Aptos,

    #[serde(rename = "AptosTestnet")]
    AptosTestnet,

    #[serde(rename = "ArbitrumGoerli")]
    ArbitrumGoerli,

    #[serde(rename = "ArbitrumOne")]
    ArbitrumOne,

    #[serde(rename = "ArbitrumSepolia")]
    ArbitrumSepolia,

    #[serde(rename = "AvalancheC")]
    AvalancheC,

    #[serde(rename = "AvalancheCFuji")]
    AvalancheCFuji,

    Base,

    #[serde(rename = "BaseGoerli")]
    BaseGoerli,

    #[serde(rename = "BaseSepolia")]
    BaseSepolia,

    Berachain,

    #[serde(rename = "BerachainBArtio")]
    BerachainBArtio,

    Bitcoin,

    #[serde(rename = "BitcoinSignet")]
    BitcoinSignet,

    #[serde(rename = "BitcoinTestnet3")]
    BitcoinTestnet3,

    Bsc,

    #[serde(rename = "BscTestnet")]
    BscTestnet,

    Cardano,

    #[serde(rename = "CardanoPreprod")]
    CardanoPreprod,

    Celo,

    #[serde(rename = "CeloAlfajores")]
    CeloAlfajores,

    Dogecoin,

    #[serde(rename = "DogecoinTestnet")]
    DogecoinTestnet,

    Ethereum,

    #[serde(rename = "EthereumGoerli")]
    EthereumGoerli,

    #[serde(rename = "EthereumHolesky")]
    EthereumHolesky,

    #[serde(rename = "EthereumSepolia")]
    EthereumSepolia,

    #[serde(rename = "FantomOpera")]
    FantomOpera,

    #[serde(rename = "FantomTestnet")]
    FantomTestnet,

    #[serde(rename = "InternetComputer")]
    InternetComputer,

    Ion,

    #[serde(rename = "IonTestnet")]
    IonTestnet,

    Iota,

    #[serde(rename = "IotaTestnet")]
    IotaTestnet,

    Kaspa,

    #[serde(rename = "KaspaTestnet11")]
    KaspaTestnet11,

    #[serde(rename = "KeyECDSA")]
    KeyEcdsa,

    #[serde(rename = "KeyECDSAStark")]
    KeyEcdsaStark,

    #[serde(rename = "KeyEdDSA")]
    KeyEdDsa,

    Kusama,

    Litecoin,

    #[serde(rename = "LitecoinTestnet")]
    LitecoinTestnet,

    Optimism,

    #[serde(rename = "OptimismGoerli")]
    OptimismGoerli,

    #[serde(rename = "OptimismSepolia")]
    OptimismSepolia,

    Origyn,

    Polkadot,

    Polygon,

    #[serde(rename = "PolygonAmoy")]
    PolygonAmoy,

    #[serde(rename = "PolygonMumbai")]
    PolygonMumbai,

    Race,

    #[serde(rename = "RaceSepolia")]
    RaceSepolia,

    #[serde(rename = "SeiAtlantic2")]
    SeiAtlantic2,

    #[serde(rename = "SeiPacific1")]
    SeiPacific1,

    Solana,

    #[serde(rename = "SolanaDevnet")]
    SolanaDevnet,

    Stellar,

    #[serde(rename = "StellarTestnet")]
    StellarTestnet,

    Tezos,

    #[serde(rename = "TezosGhostnet")]
    TezosGhostnet,

    Ton,

    #[serde(rename = "TonTestnet")]
    TonTestnet,

    Tron,

    #[serde(rename = "TronNile")]
    TronNile,

    Westend,

    #[serde(rename = "XrpLedger")]
    XrpLedger,

    #[serde(rename = "XrpLedgerTestnet")]
    XrpLedgerTestnet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterEndUserResponse {
    pub authentication: Authentication,

    pub credential: RegisterEndUserResponseCredential,

    pub user: RegisterEndUserResponseUser,

    pub wallets: Vec<RegisterEndUserResponseWallet>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authentication {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterEndUserResponseCredential {
    pub kind: CredentialKindElement,

    pub name: String,

    pub uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterEndUserResponseUser {
    pub id: String,

    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterEndUserResponseWallet {
    pub address: Option<String>,

    pub custodial: bool,

    pub date_created: String,

    pub date_exported: Option<String>,

    pub exported: Option<bool>,

    pub external_id: Option<String>,

    pub id: String,

    pub imported: Option<bool>,

    pub name: Option<String>,

    pub network: Network,

    pub signing_key: SigningKey,

    pub status: Status,

    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningKey {
    pub curve: Curve,

    pub public_key: String,

    pub scheme: Scheme,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Curve {
    Ed25519,

    Secp256K1,

    Stark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Scheme {
    #[serde(rename = "ECDSA")]
    Ecdsa,

    #[serde(rename = "EdDSA")]
    EdDsa,

    Schnorr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Active,

    Archived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterEndUserRequest {
    pub body: RegisterEndUserRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterEndUserRequestBody {
    pub first_factor_credential: StickyFirstFactorCredential,

    pub recovery_credential: Option<StickyRecoveryCredential>,

    pub second_factor_credential: Option<IndecentSecondFactorCredential>,

    pub wallets: Vec<BodyWallet>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyFirstFactorCredential {
    pub credential_info: CredentialInfo3,

    pub credential_kind: FirstFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialInfo3 {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyRecoveryCredential {
    pub credential_info: CredentialInfo4,

    pub credential_kind: RecoveryCredentialKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialInfo4 {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentSecondFactorCredential {
    pub credential_info: CredentialInfo5,

    pub credential_kind: SecondFactorKind,

    pub credential_name: Option<String>,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialInfo5 {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyWallet {
    pub name: Option<String>,

    pub network: Network,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendRegistrationCodeBody {
    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResendRegistrationCodeResponse {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResendRegistrationCodeRequest {
    pub body: ResendRegistrationCodeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendRegistrationCodeRequestBody {
    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLoginCodeBody {
    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendLoginCodeResponse {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendLoginCodeRequest {
    pub body: SendLoginCodeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLoginCodeRequestBody {
    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendRecoveryCodeBody {
    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendRecoveryCodeResponse {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendRecoveryCodeRequest {
    pub body: SendRecoveryCodeRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendRecoveryCodeRequestBody {
    pub org_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialLoginBody {
    pub id_token: String,

    pub social_login_provider_kind: SocialLoginProviderKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialLoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialLoginRequest {
    pub body: SocialLoginRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialLoginRequestBody {
    pub id_token: String,

    pub social_login_provider_kind: SocialLoginProviderKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplicationBody {
    pub external_id: Option<String>,

    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplicationParams {
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplicationResponse {
    pub access_tokens: Vec<UpdateApplicationResponseAccessToken>,

    pub app_id: String,

    pub expected_origin: Option<String>,

    pub expected_rp_id: Option<String>,

    pub is_active: bool,

    pub kind: ActivateApplicationResponseKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<UpdateApplicationResponsePermissionAssignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplicationResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment9>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment9 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplicationResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplicationRequest {
    pub app_id: String,

    pub body: UpdateApplicationRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplicationRequestBody {
    pub external_id: Option<String>,

    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePersonalAccessTokenBody {
    pub external_id: Option<String>,

    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePersonalAccessTokenParams {
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePersonalAccessTokenResponse {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<UpdatePersonalAccessTokenResponsePermissionAssignment>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePersonalAccessTokenResponsePermissionAssignment {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePersonalAccessTokenRequest {
    pub body: UpdatePersonalAccessTokenRequestBody,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePersonalAccessTokenRequestBody {
    pub external_id: Option<String>,

    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceAccountBody {
    pub external_id: Option<String>,

    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceAccountParams {
    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceAccountResponse {
    pub access_tokens: Vec<UpdateServiceAccountResponseAccessToken>,

    pub user_info: UpdateServiceAccountResponseUserInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceAccountResponseAccessToken {
    pub access_token: Option<String>,

    pub cred_id: String,

    pub date_created: String,

    pub is_active: bool,

    pub kind: AccessTokenKind,

    pub linked_app_id: String,

    pub linked_user_id: String,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment10>,

    pub public_key: String,

    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment10 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceAccountResponseUserInfo {
    pub credential_uuid: String,

    pub is_active: bool,

    pub is_registered: bool,

    pub is_service_account: bool,

    pub kind: UserInfoKind,

    pub name: String,

    pub org_id: String,

    pub permission_assignments: Vec<PermissionAssignment11>,

    pub permissions: Option<Vec<String>>,

    pub user_id: String,

    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignment11 {
    pub assignment_id: String,

    pub operations: Option<Vec<String>>,

    pub permission_id: String,

    pub permission_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceAccountRequest {
    pub body: UpdateServiceAccountRequestBody,

    pub service_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceAccountRequestBody {
    pub external_id: Option<String>,

    pub name: Option<String>,
}
