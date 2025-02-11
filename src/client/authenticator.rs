// @dfns-sdk-rs/src/client/authenticator.rs

use crate::{
    client::base_auth_api::{
        BaseAuthApi, CreateUserLoginChallengeRequest, CreateUserRegistrationChallengeRequest,
        CreateUserRegistrationRequest, SignUserActionChallengeRequest,
    },
    error::DfnsError,
    models::generic::DfnsBaseApiOptions,
    signer::CredentialSigner,
    store::{Challenge, CredentialStore, FirstFactorAttestation},
};
use serde::{Deserialize, Serialize};

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
pub struct LoginRequest {
    pub username: String,
    pub org_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegisterRequest {
    pub org_id: String,
    pub username: String,
    pub registration_code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegisterResponse {
    pub credential: CredentialInfo,
    pub user: UserInfo,
}

#[derive(Debug, Clone)]
pub struct DfnsAuthenticatorOptions<S> {
    pub app_id: String,
    pub base_url: Option<String>,
    pub app_secret: Option<String>,
    pub signer: S,
}

pub struct DfnsAuthenticator<S> {
    api_options: DfnsAuthenticatorOptions<S>,
}

impl<S> DfnsAuthenticator<S>
where
    S: CredentialSigner + CredentialStore<FirstFactorAttestation> + Send + Sync,
{
    pub fn new(api_options: DfnsAuthenticatorOptions<S>) -> Self {
        Self { api_options }
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, DfnsError> {
        let base_options = DfnsBaseApiOptions {
            app_id: self.api_options.app_id.clone(),
            base_url: self.api_options.base_url.clone(),
            app_secret: self.api_options.app_secret.clone(),
            auth_token: None,
        };

        let challenge = BaseAuthApi::create_user_login_challenge(
            CreateUserLoginChallengeRequest {
                username: request.username,
                org_id: request.org_id,
            },
            base_options.clone(),
        )
        .await?;

        let challenge_id = challenge.challenge_identifier.clone();
        let assertion = self.api_options.signer.sign(challenge).await?;

        let response = BaseAuthApi::create_user_login(
            SignUserActionChallengeRequest {
                challenge_identifier: challenge_id,
                first_factor: assertion,
                second_factor: None,
            },
            base_options,
        )
        .await?;

        Ok(LoginResponse {
            token: response.token,
        })
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, DfnsError> {
        let base_options = DfnsBaseApiOptions {
            app_id: self.api_options.app_id.clone(),
            base_url: self.api_options.base_url.clone(),
            app_secret: self.api_options.app_secret.clone(),
            auth_token: None,
        };

        let registration_challenge = BaseAuthApi::create_user_registration_challenge(
            CreateUserRegistrationChallengeRequest {
                org_id: request.org_id,
                username: request.username,
                registration_code: request.registration_code,
            },
            base_options.clone(),
        )
        .await?;

        let challenge = Challenge::Registration(registration_challenge.clone());
        let attestation = self.api_options.signer.create(challenge).await?;

        let options_with_token = DfnsBaseApiOptions {
            auth_token: Some(registration_challenge.temporary_authentication_token),
            ..base_options
        };

        let response = BaseAuthApi::create_user_registration(
            CreateUserRegistrationRequest {
                first_factor_credential: attestation,
                second_factor_credential: None,
                recovery_credential: None,
            },
            options_with_token,
        )
        .await?;

        Ok(RegisterResponse {
            credential: CredentialInfo {
                uuid: response.credential.uuid,
                kind: response.credential.kind,
                name: response.credential.name,
            },
            user: UserInfo {
                id: response.user.id,
                username: response.user.username,
                org_id: response.user.org_id,
            },
        })
    }
}