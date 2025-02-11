// @dfns-sdk-rs/src/api/auth/client.rs

use super::types::*;
use crate::{
    error::DfnsError,
    models::generic::DfnsApiClientOptions,
    utils::{fetch::simple_fetch, url::build_path_and_query, user_action_fetch::user_action_fetch},
};
use std::collections::HashMap;

pub struct AuthClient {
    api_options: DfnsApiClientOptions,
}

impl AuthClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn activate_application(
        &self,
        request: ActivateApplicationRequest,
    ) -> Result<ActivateApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/apps/:app_id/activate",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("app_id".to_string(), request.app_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn activate_credential(
        &self,
        request: ActivateCredentialRequest,
    ) -> Result<ActivateCredentialResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/activate",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
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

    pub async fn activate_personal_access_token(
        &self,
        request: ActivatePersonalAccessTokenRequest,
    ) -> Result<ActivatePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pats/:token_id/activate",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("token_id".to_string(), request.token_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn activate_service_account(
        &self,
        request: ActivateServiceAccountRequest,
    ) -> Result<ActivateServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:service_account_id/activate",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("service_account_id".to_string(), request.service_account_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn activate_user(
        &self,
        request: ActivateUserRequest,
    ) -> Result<ActivateUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:user_id/activate",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("user_id".to_string(), request.user_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn archive_application(
        &self,
        request: ArchiveApplicationRequest,
    ) -> Result<ArchiveApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/apps/:app_id/archive",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("app_id".to_string(), request.app_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn archive_personal_access_token(
        &self,
        request: ArchivePersonalAccessTokenRequest,
    ) -> Result<ArchivePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pats/:token_id/archive",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("token_id".to_string(), request.token_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn archive_service_account(
        &self,
        request: ArchiveServiceAccountRequest,
    ) -> Result<ArchiveServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:service_account_id/archive",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("service_account_id".to_string(), request.service_account_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn archive_user(
        &self,
        request: ArchiveUserRequest,
    ) -> Result<ArchiveUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:user_id/archive",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("user_id".to_string(), request.user_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn create_application(
        &self,
        request: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/apps",
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

    pub async fn create_credential(
        &self,
        request: CreateCredentialRequest,
    ) -> Result<CreateCredentialResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials",
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

    pub async fn create_credential_challenge(
        &self,
        request: CreateCredentialChallengeRequest,
    ) -> Result<CreateCredentialChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/init",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_credential_challenge_with_code(
        &self,
        request: CreateCredentialChallengeWithCodeRequest,
    ) -> Result<CreateCredentialChallengeWithCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/code/init",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_credential_code(
        &self,
        request: CreateCredentialCodeRequest,
    ) -> Result<CreateCredentialCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/code",
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

    pub async fn create_credential_with_code(
        &self,
        request: CreateCredentialWithCodeRequest,
    ) -> Result<CreateCredentialWithCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/code/verify",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_delegated_recovery_challenge(
        &self,
        request: CreateDelegatedRecoveryChallengeRequest,
    ) -> Result<CreateDelegatedRecoveryChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/recover/user/delegated",
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

    pub async fn create_delegated_registration_challenge(
        &self,
        request: CreateDelegatedRegistrationChallengeRequest,
    ) -> Result<CreateDelegatedRegistrationChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/delegated",
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

    pub async fn create_login_challenge(
        &self,
        request: CreateLoginChallengeRequest,
    ) -> Result<CreateLoginChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/login/init",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_personal_access_token(
        &self,
        request: CreatePersonalAccessTokenRequest,
    ) -> Result<CreatePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pats",
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

    pub async fn create_recovery_challenge(
        &self,
        request: CreateRecoveryChallengeRequest,
    ) -> Result<CreateRecoveryChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/recover/user/init",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_registration_challenge(
        &self,
        request: CreateRegistrationChallengeRequest,
    ) -> Result<CreateRegistrationChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/init",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_service_account(
        &self,
        request: CreateServiceAccountRequest,
    ) -> Result<CreateServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts",
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

    pub async fn create_social_registration_challenge(
        &self,
        request: CreateSocialRegistrationChallengeRequest,
    ) -> Result<CreateSocialRegistrationChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/social",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<CreateUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users",
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

    pub async fn create_user_action_challenge(
        &self,
        request: CreateUserActionChallengeRequest,
    ) -> Result<CreateUserActionChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/action/init",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_user_action_signature(
        &self,
        request: CreateUserActionSignatureRequest,
    ) -> Result<CreateUserActionSignatureResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/action",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_application(
        &self,
        request: DeactivateApplicationRequest,
    ) -> Result<DeactivateApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/apps/:app_id/deactivate",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("app_id".to_string(), request.app_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_credential(
        &self,
        request: DeactivateCredentialRequest,
    ) -> Result<DeactivateCredentialResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/deactivate",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
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

    pub async fn deactivate_personal_access_token(
        &self,
        request: DeactivatePersonalAccessTokenRequest,
    ) -> Result<DeactivatePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pats/:token_id/deactivate",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("token_id".to_string(), request.token_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_service_account(
        &self,
        request: DeactivateServiceAccountRequest,
    ) -> Result<DeactivateServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:service_account_id/deactivate",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("service_account_id".to_string(), request.service_account_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_user(
        &self,
        request: DeactivateUserRequest,
    ) -> Result<DeactivateUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:user_id/deactivate",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("user_id".to_string(), request.user_id);
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
                body: Some(serde_json::json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn delegated_login(
        &self,
        request: DelegatedLoginRequest,
    ) -> Result<DelegatedLoginResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/login/delegated",
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

    pub async fn get_application(
        &self,
        request: GetApplicationRequest,
    ) -> Result<GetApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/apps/:app_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("app_id".to_string(), request.app_id);
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

    pub async fn get_personal_access_token(
        &self,
        request: GetPersonalAccessTokenRequest,
    ) -> Result<GetPersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pats/:token_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("token_id".to_string(), request.token_id);
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

    pub async fn get_service_account(
        &self,
        request: GetServiceAccountRequest,
    ) -> Result<GetServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:service_account_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("service_account_id".to_string(), request.service_account_id);
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

    pub async fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:user_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("user_id".to_string(), request.user_id);
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

    pub async fn list_applications(&self) -> Result<ListApplicationsResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/apps",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
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

    pub async fn list_credentials(&self) -> Result<ListCredentialsResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
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

    pub async fn list_personal_access_tokens(
        &self,
    ) -> Result<ListPersonalAccessTokensResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pats",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
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

    pub async fn list_service_accounts(&self) -> Result<ListServiceAccountsResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
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

    pub async fn list_users(
        &self,
        request: Option<ListUsersRequest>,
    ) -> Result<ListUsersResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(kind) = q.kind {
                            map.insert("kind".to_string(), format!("{:?}", kind));
                        }
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(pagination_token) = q.pagination_token {
                            map.insert("pagination_token".to_string(), pagination_token);
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

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/login",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn logout(
        &self,
        request: Option<LogoutRequest>,
    ) -> Result<LogoutResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/logout",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::PUT,
                headers: None,
                body: request.map(|r| serde_json::to_value(&r.body).unwrap()),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn recover(&self, request: RecoverRequest) -> Result<RecoverResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/recover/user",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn recreate_delegated_registration_challenge(
        &self,
        request: RecreateDelegatedRegistrationChallengeRequest,
    ) -> Result<RecreateDelegatedRegistrationChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/delegated/restart",
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

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn register_end_user(
        &self,
        request: RegisterEndUserRequest,
    ) -> Result<RegisterEndUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/enduser",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn resend_registration_code(
        &self,
        request: ResendRegistrationCodeRequest,
    ) -> Result<ResendRegistrationCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/code",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
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

    pub async fn send_login_code(
        &self,
        request: SendLoginCodeRequest,
    ) -> Result<SendLoginCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/login/code",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn send_recovery_code(
        &self,
        request: SendRecoveryCodeRequest,
    ) -> Result<SendRecoveryCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/recover/user/code",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn social_login(
        &self,
        request: SocialLoginRequest,
    ) -> Result<SocialLoginResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/login/social",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn update_application(
        &self,
        request: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/apps/:app_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("app_id".to_string(), request.app_id.clone());
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

    pub async fn update_personal_access_token(
        &self,
        request: UpdatePersonalAccessTokenRequest,
    ) -> Result<UpdatePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pats/:token_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("token_id".to_string(), request.token_id.clone());
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

    pub async fn update_service_account(
        &self,
        request: UpdateServiceAccountRequest,
    ) -> Result<UpdateServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:service_account_id",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "service_account_id".to_string(),
                        request.service_account_id.clone(),
                    );
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
