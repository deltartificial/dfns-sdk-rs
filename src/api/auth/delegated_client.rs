use super::types::*;
use crate::{
    client::{
        base_auth_api::{
            BaseAuthApi, CreateUserActionChallengeRequest, SignUserActionChallengeRequest,
        },
        delegated_api_client::DfnsDelegatedApiClientOptions,
    },
    error::DfnsError,
    signer::UserActionChallenge,
    utils::{
        fetch::{simple_fetch, FetchOptions, HttpMethod},
        url::{build_path_and_query, PathAndQueryParams},
    },
};
use serde_json::json;
use std::collections::HashMap;

pub struct DelegatedAuthClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedAuthClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn activate_application_init(
        &self,
        request: ActivateApplicationRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId/activate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn activate_application_complete(
        &self,
        request: ActivateApplicationRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ActivateApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId/activate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn activate_credential_init(
        &self,
        request: ActivateCredentialRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/activate",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn activate_credential_complete(
        &self,
        request: ActivateCredentialRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ActivateCredentialResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/activate",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn activate_personal_access_token_init(
        &self,
        request: ActivatePersonalAccessTokenRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat/:tokenId/activate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn activate_personal_access_token_complete(
        &self,
        request: ActivatePersonalAccessTokenRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ActivatePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat/:tokenId/activate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn activate_service_account_init(
        &self,
        request: ActivateServiceAccountRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:serviceAccountId/activate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn activate_service_account_complete(
        &self,
        request: ActivateServiceAccountRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ActivateServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:serviceAccountId/activate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn activate_user_init(
        &self,
        request: ActivateUserRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:userId/activate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("userId".to_string(), request.user_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn activate_user_complete(
        &self,
        request: ActivateUserRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ActivateUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:userId/activate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("userId".to_string(), request.user_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn archive_application_init(
        &self,
        request: ArchiveApplicationRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn archive_application_complete(
        &self,
        request: ArchiveApplicationRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ArchiveApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn archive_personal_access_token_init(
        &self,
        request: ArchivePersonalAccessTokenRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat/:tokenId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn archive_personal_access_token_complete(
        &self,
        request: ArchivePersonalAccessTokenRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ArchivePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat/:tokenId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn archive_service_account_init(
        &self,
        request: ArchiveServiceAccountRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:serviceAccountId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn archive_service_account_complete(
        &self,
        request: ArchiveServiceAccountRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ArchiveServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:serviceAccountId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn archive_user_init(
        &self,
        request: ArchiveUserRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:userId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("userId".to_string(), request.user_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn archive_user_complete(
        &self,
        request: ArchiveUserRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ArchiveUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:userId/archive",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("userId".to_string(), request.user_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_application_init(
        &self,
        request: CreateApplicationRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_application_complete(
        &self,
        request: CreateApplicationRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_credential_init(
        &self,
        request: CreateCredentialRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_credential_complete(
        &self,
        request: CreateCredentialRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateCredentialResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_credential_challenge(
        &self,
        request: CreateCredentialChallengeRequest,
    ) -> Result<CreateCredentialChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
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
            "/auth/credentials/challenge-with-code",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_credential_code_init(
        &self,
        request: CreateCredentialCodeRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/code",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_credential_code_complete(
        &self,
        request: CreateCredentialCodeRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateCredentialCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/code",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_credential_with_code(
        &self,
        request: CreateCredentialWithCodeRequest,
    ) -> Result<CreateCredentialWithCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/with-code",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_delegated_recovery_challenge_init(
        &self,
        request: CreateDelegatedRecoveryChallengeRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/recovery/delegated/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_delegated_recovery_challenge_complete(
        &self,
        request: CreateDelegatedRecoveryChallengeRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateDelegatedRecoveryChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/recovery/delegated/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_delegated_registration_challenge_init(
        &self,
        request: CreateDelegatedRegistrationChallengeRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/delegated/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_delegated_registration_challenge_complete(
        &self,
        request: CreateDelegatedRegistrationChallengeRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateDelegatedRegistrationChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/delegated/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_login_challenge(
        &self,
        request: CreateLoginChallengeRequest,
    ) -> Result<CreateLoginChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/login/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_personal_access_token_init(
        &self,
        request: CreatePersonalAccessTokenRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_personal_access_token_complete(
        &self,
        request: CreatePersonalAccessTokenRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreatePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_recovery_challenge(
        &self,
        request: CreateRecoveryChallengeRequest,
    ) -> Result<CreateRecoveryChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/recovery/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
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
            "/auth/registration/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_service_account_init(
        &self,
        request: CreateServiceAccountRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_service_account_complete(
        &self,
        request: CreateServiceAccountRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_social_registration_challenge(
        &self,
        request: CreateSocialRegistrationChallengeRequest,
    ) -> Result<CreateSocialRegistrationChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/social/challenge",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_user_init(
        &self,
        request: CreateUserRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/users",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_user_complete(
        &self,
        request: CreateUserRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_application_init(
        &self,
        request: DeactivateApplicationRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId/deactivate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn deactivate_application_complete(
        &self,
        request: DeactivateApplicationRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DeactivateApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId/deactivate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_credential_init(
        &self,
        request: DeactivateCredentialRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/deactivate",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn deactivate_credential_complete(
        &self,
        request: DeactivateCredentialRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DeactivateCredentialResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/credentials/deactivate",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_personal_access_token_init(
        &self,
        request: DeactivatePersonalAccessTokenRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat/:tokenId/deactivate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn deactivate_personal_access_token_complete(
        &self,
        request: DeactivatePersonalAccessTokenRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DeactivatePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat/:tokenId/deactivate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_service_account_init(
        &self,
        request: DeactivateServiceAccountRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:serviceAccountId/deactivate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn deactivate_service_account_complete(
        &self,
        request: DeactivateServiceAccountRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DeactivateServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:serviceAccountId/deactivate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn deactivate_user_init(
        &self,
        request: DeactivateUserRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:userId/deactivate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("userId".to_string(), request.user_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: "{}".to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn deactivate_user_complete(
        &self,
        request: DeactivateUserRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DeactivateUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:userId/deactivate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("userId".to_string(), request.user_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn delegated_login_init(
        &self,
        request: DelegatedLoginRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/login/delegated",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn delegated_login_complete(
        &self,
        request: DelegatedLoginRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DelegatedLoginResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/login/delegated",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_application(
        &self,
        request: GetApplicationRequest,
    ) -> Result<GetApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
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
            "/auth/pat/:tokenId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
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
            "/auth/service-accounts/:serviceAccountId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/users/:userId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("userId".to_string(), request.user_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn list_applications(&self) -> Result<ListApplicationsResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
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
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
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
            "/auth/pat",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
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
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
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
            &PathAndQueryParams {
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
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
                        map
                    })
                    .unwrap_or_default(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
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
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
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
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: None,
                body: request.map(|r| json!(r.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn recover(&self, request: RecoverRequest) -> Result<RecoverResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/recover",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn recreate_delegated_registration_challenge_init(
        &self,
        request: RecreateDelegatedRegistrationChallengeRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/delegated/challenge/recreate",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn recreate_delegated_registration_challenge_complete(
        &self,
        request: RecreateDelegatedRegistrationChallengeRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<RecreateDelegatedRegistrationChallengeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/delegated/challenge/recreate",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
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
            "/auth/registration/end-user",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn resend_registration_code_init(
        &self,
        request: ResendRegistrationCodeRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/code/resend",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn resend_registration_code_complete(
        &self,
        request: ResendRegistrationCodeRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ResendRegistrationCodeResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/registration/code/resend",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
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
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
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
            "/auth/recovery/code",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
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
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn update_application_init(
        &self,
        request: UpdateApplicationRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn update_application_complete(
        &self,
        request: UpdateApplicationRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<UpdateApplicationResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/applications/:appId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("appId".to_string(), request.app_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn update_personal_access_token_init(
        &self,
        request: UpdatePersonalAccessTokenRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat/:tokenId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn update_personal_access_token_complete(
        &self,
        request: UpdatePersonalAccessTokenRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<UpdatePersonalAccessTokenResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/pat/:tokenId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("tokenId".to_string(), request.token_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn update_service_account_init(
        &self,
        request: UpdateServiceAccountRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:serviceAccountId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn update_service_account_complete(
        &self,
        request: UpdateServiceAccountRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<UpdateServiceAccountResponse, DfnsError> {
        let path = build_path_and_query(
            "/auth/service-accounts/:serviceAccountId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert(
                        "serviceAccountId".to_string(),
                        request.service_account_id.clone(),
                    );
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
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
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!({
                    "user_action_payload": request.user_action_payload,
                    "user_action_http_method": request.user_action_http_method,
                    "user_action_http_path": request.user_action_http_path,
                    "user_action_server_kind": request.user_action_server_kind,
                })),
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
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }
}
