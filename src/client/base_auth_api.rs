// @dfns-sdk-rs/src/client/base_auth_api.rs

use crate::error::DfnsError;
use crate::signer::{
    FirstFactorAssertion, RecoveryKeyAssertion, SecondFactorAssertion, UserActionChallenge,
};
use crate::models::generic::DfnsBaseApiOptions;
use crate::store::{FirstFactorAttestation, RecoveryFactorAttestation, SecondFactorAttestation};
use crate::utils::fetch::{simple_fetch, FetchOptions, HttpMethod};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateUserActionChallengeRequest {
    pub user_action_payload: String,
    pub user_action_http_method: HttpMethod,
    pub user_action_http_path: String,
    pub user_action_server_kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SignUserActionChallengeRequest {
    pub challenge_identifier: String,
    pub first_factor: FirstFactorAssertion,
    pub second_factor: Option<SecondFactorAssertion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserActionResponse {
    pub user_action: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateUserLoginChallengeRequest {
    pub username: String,
    pub org_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserLoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateUserRegistrationChallengeRequest {
    pub org_id: String,
    pub username: String,
    pub registration_code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateUserRegistrationRequest {
    pub first_factor_credential: FirstFactorAttestation,
    pub second_factor_credential: Option<SecondFactorAttestation>,
    pub recovery_credential: Option<RecoveryFactorAttestation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserRegistrationResponse {
    pub credential: CredentialInfo,
    pub user: UserInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CredentialInfo {
    pub uuid: String,
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub org_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateUserRecoveryRequest {
    pub recovery: RecoveryKeyAssertion,
    pub new_credentials: NewCredentials,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NewCredentials {
    pub first_factor_credential: FirstFactorAttestation,
    pub second_factor_credential: Option<SecondFactorAttestation>,
    pub recovery_credential: Option<RecoveryFactorAttestation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BaseAuthApi;

impl BaseAuthApi {
    pub async fn create_user_action_challenge(
        request: CreateUserActionChallengeRequest,
        options: DfnsBaseApiOptions,
    ) -> Result<UserActionChallenge, DfnsError> {
        let fetch_options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(serde_json::to_value(&request)?),
            api_options: options,
        };

        simple_fetch("/auth/action/init", fetch_options).await
    }

    pub async fn sign_user_action_challenge(
        request: SignUserActionChallengeRequest,
        options: DfnsBaseApiOptions,
    ) -> Result<UserActionResponse, DfnsError> {
        let fetch_options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(serde_json::to_value(&request)?),
            api_options: options,
        };

        simple_fetch("/auth/action", fetch_options).await
    }

    pub async fn create_user_login_challenge(
        request: CreateUserLoginChallengeRequest,
        options: DfnsBaseApiOptions,
    ) -> Result<UserActionChallenge, DfnsError> {
        let fetch_options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(serde_json::to_value(&request)?),
            api_options: options,
        };

        simple_fetch("/auth/login/init", fetch_options).await
    }

    pub async fn create_user_login(
        request: SignUserActionChallengeRequest,
        options: DfnsBaseApiOptions,
    ) -> Result<UserLoginResponse, DfnsError> {
        let fetch_options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(serde_json::to_value(&request)?),
            api_options: options,
        };

        simple_fetch("/auth/login", fetch_options).await
    }

    pub async fn user_logout(options: DfnsBaseApiOptions) -> Result<(), DfnsError> {
        if options.auth_token.is_none() {
            return Err(DfnsError::new(400, "authToken is required".to_string(), None));
        }

        let fetch_options = FetchOptions {
            method: HttpMethod::PUT,
            headers: None,
            body: None,
            api_options: options,
        };

        simple_fetch("/auth/logout", fetch_options).await
    }

    pub async fn create_user_registration_challenge(
        request: CreateUserRegistrationChallengeRequest,
        options: DfnsBaseApiOptions,
    ) -> Result<UserActionChallenge, DfnsError> {
        let fetch_options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(serde_json::to_value(&request)?),
            api_options: options,
        };

        simple_fetch("/auth/registration/init", fetch_options).await
    }

    pub async fn create_user_registration(
        request: CreateUserRegistrationRequest,
        options: DfnsBaseApiOptions,
    ) -> Result<UserRegistrationResponse, DfnsError> {
        let fetch_options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(serde_json::to_value(&request)?),
            api_options: options,
        };

        simple_fetch("/auth/registration", fetch_options).await
    }

    pub async fn create_user_recovery(
        request: CreateUserRecoveryRequest,
        options: DfnsBaseApiOptions,
    ) -> Result<UserRegistrationResponse, DfnsError> {
        let fetch_options = FetchOptions {
            method: HttpMethod::POST,
            headers: None,
            body: Some(serde_json::to_value(&request)?),
            api_options: options,
        };

        simple_fetch("/auth/recover/user", fetch_options).await
    }
}