// @dfns-sdk-rs/src/signer.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialTransport {
    Ble,

    Internal,

    Nfc,

    Usb,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowCredential {
    pub id: String,

    #[serde(rename = "type")]
    pub allow_credential_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Type {
    #[serde(rename = "public-key")]
    PublicKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedCredential {
    pub factor: CredentialFactor,

    pub kind: CredentialKind,

    pub requires_second_factor: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialFactor {
    Either,

    First,

    Second,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialKind {
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
#[serde(rename_all = "camelCase")]
pub struct UserActionChallenge {
    pub allow_credentials: AllowCredentials,

    pub challenge: String,

    pub challenge_identifier: String,

    pub external_authentication_url: String,

    pub rp: Option<Rp>,

    pub supported_credential_kinds: Vec<SupportedCredentialKind>,

    pub user_verification: UserVerificationRequirement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowCredentials {
    pub key: Vec<Key>,

    pub webauthn: Vec<Webauthn>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    pub id: String,

    #[serde(rename = "type")]
    pub key_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Webauthn {
    pub id: String,

    #[serde(rename = "type")]
    pub webauthn_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rp {
    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedCredentialKind {
    pub factor: CredentialFactor,

    pub kind: CredentialKind,

    pub requires_second_factor: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserVerificationRequirement {
    Discouraged,

    Preferred,

    Required,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAssertion {
    pub credential_assertion: KeyAssertionCredentialAssertion,

    pub kind: KeyAssertionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAssertionCredentialAssertion {
    pub algorithm: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyAssertionKind {
    Key,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2Assertion {
    pub credential_assertion: Fido2AssertionCredentialAssertion,

    pub kind: Fido2AssertionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2AssertionCredentialAssertion {
    pub authenticator_data: String,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Fido2AssertionKind {
    Fido2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordAssertion {
    pub kind: PasswordAssertionKind,

    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PasswordAssertionKind {
    Password,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotpAssertion {
    pub kind: TotpAssertionKind,

    pub otp_code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TotpAssertionKind {
    Totp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryKeyAssertion {
    pub credential_assertion: RecoveryKeyAssertionCredentialAssertion,

    pub kind: RecoveryKeyAssertionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryKeyAssertionCredentialAssertion {
    pub algorithm: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecoveryKeyAssertionKind {
    #[serde(rename = "RecoveryKey")]
    RecoveryKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstFactorAssertion {
    pub credential_assertion: Option<FirstFactorAssertionCredentialAssertion>,

    pub kind: FirstFactorAssertionKind,

    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstFactorAssertionCredentialAssertion {
    pub algorithm: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub authenticator_data: Option<String>,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FirstFactorAssertionKind {
    Fido2,

    Key,

    Password,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondFactorAssertion {
    pub credential_assertion: Option<SecondFactorAssertionCredentialAssertion>,

    pub kind: SecondFactorAssertionKind,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondFactorAssertionCredentialAssertion {
    pub algorithm: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub authenticator_data: Option<String>,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecondFactorAssertionKind {
    Fido2,

    Key,

    Totp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialAssertion {
    pub credential_assertion: Option<CredentialAssertionCredentialAssertion>,

    pub kind: CredentialAssertionKind,

    pub password: Option<String>,

    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialAssertionCredentialAssertion {
    pub algorithm: Option<String>,

    pub client_data: String,

    pub cred_id: String,

    pub signature: String,

    pub authenticator_data: Option<String>,

    pub user_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialAssertionKind {
    Fido2,

    Key,

    Password,

    Totp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialSignerT {}
