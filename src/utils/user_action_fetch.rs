// @dfns-sdk-rs/src/utils/user_action_fetch.rs

use crate::client::base_auth_api::{
    BaseAuthApi, CreateUserActionChallengeRequest, SignUserActionChallengeRequest,
};
use crate::error::DfnsError;
use crate::models::generic::{DfnsApiClientOptions, DfnsBaseApiOptions};
use crate::utils::fetch::{DfnsFetch, Fetch, FetchOptions, HttpMethod};
use reqwest::Response;
use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub struct UserActionFetch {
    inner: DfnsFetch,
}

impl UserActionFetch {
    pub fn new() -> Self {
        Self {
            inner: DfnsFetch::new(),
        }
    }
}

impl Fetch for UserActionFetch {
    async fn execute(
        &self,
        resource: &str,
        mut options: FetchOptions<DfnsBaseApiOptions>,
    ) -> Result<Response, DfnsError> {
        if options.method != HttpMethod::GET {
            return Err(DfnsError::new(
                400,
                "A 'signer' needs to be passed to Dfns client.",
                Some(serde_json::json!({
                    "detail": "Most non-readonly endpoints require 'User Action Signing' flow. During that flow, the credential 'signer' that you passed will handle signing the user action challenge, using your credential."
                })),
            ));
        }

        self.inner.execute(resource, options).await
    }
}

pub async fn user_action_fetch<T>(
    resource: &str,
    options: FetchOptions<DfnsApiClientOptions>,
) -> Result<T, DfnsError>
where
    T: serde::de::DeserializeOwned,
{
    let fetch = UserActionFetch::new();

    if options.method != HttpMethod::GET {
        let api_options = options.api_options;
        let signer = api_options.signer.ok_or_else(|| DfnsError::new(
            400,
            "A 'signer' needs to be passed to Dfns client.",
            Some(serde_json::json!({
                "detail": "Most non-readonly endpoints require 'User Action Signing' flow. During that flow, the credential 'signer' that you passed will handle signing the user action challenge, using your credential."
            }))
        ))?;

        let url = Url::parse(resource)?;
        let challenge = BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_payload: options
                    .body
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                user_action_http_method: options.method.clone(),
                user_action_http_path: url.path().to_string(),
                user_action_server_kind: "Api".to_string(),
            },
            api_options.base.clone(),
        )
        .await?;

        let challenge_id = challenge.challenge_identifier.clone();
        let assertion = signer.sign(challenge).await?;
        let user_action_response = BaseAuthApi::sign_user_action_challenge(
            SignUserActionChallengeRequest {
                challenge_identifier: challenge_id,
                first_factor: assertion,
                second_factor: None,
            },
            api_options.base.clone(),
        )
        .await?;

        let mut base_options = FetchOptions {
            method: options.method,
            headers: options.headers,
            body: options.body,
            api_options: api_options.base,
        };

        let mut headers = base_options.headers.unwrap_or_default();
        headers.insert(
            "x-dfns-useraction".to_string(),
            user_action_response.user_action,
        );
        base_options.headers = Some(headers);

        let response = fetch.execute(resource, base_options).await?;
        response.json::<T>().await.map_err(Into::into)
    } else {
        let base_options = FetchOptions {
            method: options.method,
            headers: options.headers,
            body: options.body,
            api_options: options.api_options.base,
        };
        let response = fetch.execute(resource, base_options).await?;
        response.json::<T>().await.map_err(Into::into)
    }
}
