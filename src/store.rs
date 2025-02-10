// @dfns-sdk-rs/src/store.rs

use crate::api::auth::types::{
    CreateCredentialChallengeResponse, CreateRegistrationChallengeResponse,
};
use crate::error::DfnsError;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthenticatorAttachment {
    #[serde(rename = "cross-platform")]
    CrossPlatform,

    Platform,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidentKeyRequirement {
    Discouraged,

    Preferred,

    Required,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttestationConveyancePreference {
    Direct,

    Enterprise,

    Indirect,

    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAttestation {
    pub credential_info: KeyAttestationCredentialInfo,

    pub credential_kind: KeyAttestationCredentialKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAttestationCredentialInfo {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyAttestationCredentialKind {
    Key,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2Attestation {
    pub credential_info: Fido2AttestationCredentialInfo,

    pub credential_kind: Fido2AttestationCredentialKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2AttestationCredentialInfo {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Fido2AttestationCredentialKind {
    Fido2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordAttestation {
    pub credential_info: PasswordAttestationCredentialInfo,

    pub credential_kind: PasswordAttestationCredentialKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordAttestationCredentialInfo {
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PasswordAttestationCredentialKind {
    Password,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotpAttestation {
    pub credential_info: TotpAttestationCredentialInfo,

    pub credential_kind: TotpAttestationCredentialKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotpAttestationCredentialInfo {
    pub otp_code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TotpAttestationCredentialKind {
    Totp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstFactorAttestation {
    pub credential_info: FirstFactorAttestationCredentialInfo,

    pub credential_kind: FirstFactorAttestationCredentialKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstFactorAttestationCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FirstFactorAttestationCredentialKind {
    Fido2,

    Key,

    Password,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondFactorAttestation {
    pub credential_info: SecondFactorAttestationCredentialInfo,

    pub credential_kind: SecondFactorAttestationCredentialKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondFactorAttestationCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecondFactorAttestationCredentialKind {
    Fido2,

    Key,

    Totp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryKeyAttestation {
    pub credential_info: RecoveryKeyAttestationCredentialInfo,

    pub credential_kind: RecoveryKeyAttestationCredentialKind,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryKeyAttestationCredentialInfo {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecoveryKeyAttestationCredentialKind {
    #[serde(rename = "RecoveryKey")]
    RecoveryKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryFactorAttestation {
    pub credential_info: RecoveryFactorAttestationCredentialInfo,

    pub credential_kind: RecoveryKeyAttestationCredentialKind,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryFactorAttestationCredentialInfo {
    pub attestation_data: String,

    pub client_data: String,

    pub cred_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialAttestation {
    pub credential_info: CredentialAttestationCredentialInfo,

    pub credential_kind: CredentialAttestationCredentialKind,

    pub encrypted_private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialAttestationCredentialInfo {
    pub attestation_data: Option<String>,

    pub client_data: Option<String>,

    pub cred_id: Option<String>,

    pub password: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialAttestationCredentialKind {
    Fido2,

    Key,

    Password,

    #[serde(rename = "RecoveryKey")]
    RecoveryKey,

    Totp,
}

pub type UserRegistrationChallenge = CreateRegistrationChallengeResponse;

#[derive(Debug, Clone)]
pub enum Challenge {
    Registration(CreateRegistrationChallengeResponse),
    Credential(CreateCredentialChallengeResponse),
}

pub trait CredentialAttestationTrait: Send + Sync + 'static {
    fn get_kind(&self) -> &str;
}

pub trait CredentialStore<T: CredentialAttestationTrait> {
    fn create<'a>(
        &'a self,
        challenge: Challenge,
    ) -> Pin<Box<dyn Future<Output = Result<T, DfnsError>> + Send + 'a>>;
}

impl CredentialAttestationTrait for FirstFactorAttestation {
    fn get_kind(&self) -> &str {
        "FirstFactor"
    }
}
