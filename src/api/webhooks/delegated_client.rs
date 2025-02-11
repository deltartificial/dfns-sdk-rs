// @dfns-sdk-rs/src/api/webhooks/delegated_client.rs

use super::types::*;
use crate::{
    client::{
        base_auth_api::{
            BaseAuthApi, CreateUserActionChallengeRequest, SignUserActionChallengeRequest,
        },
        delegated_api_client::DfnsDelegatedApiClientOptions,
    },
    error::DfnsError,
    utils::{
        fetch::{simple_fetch, FetchOptions, HttpMethod},
        url::build_path_and_query,
    },
};
use std::collections::HashMap;

pub struct DelegatedWebhooksClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedWebhooksClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn create_webhook_init(
        &self,
        request: CreateWebhookRequest,
    ) -> Result<crate::signer::UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/webhooks",
            &crate::utils::url::PathAndQueryParams {
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

    pub async fn create_webhook_complete(
        &self,
        request: CreateWebhookRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateWebhookResponse, DfnsError> {
        let path = build_path_and_query(
            "/webhooks",
            &crate::utils::url::PathAndQueryParams {
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
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn delete_webhook_init(
        &self,
        request: DeleteWebhookRequest,
    ) -> Result<crate::signer::UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::DELETE,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&serde_json::json!({}))?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn delete_webhook_complete(
        &self,
        request: DeleteWebhookRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DeleteWebhookResponse, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
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
                method: HttpMethod::DELETE,
                headers: Some(headers),
                body: Some(serde_json::json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_webhook(
        &self,
        request: GetWebhookRequest,
    ) -> Result<GetWebhookResponse, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
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

    pub async fn get_webhook_event(
        &self,
        request: GetWebhookEventRequest,
    ) -> Result<GetWebhookEventResponse, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId/events/:webhookEventId",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
                    map.insert("webhookEventId".to_string(), request.webhook_event_id);
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

    pub async fn list_webhook_events(
        &self,
        request: ListWebhookEventsRequest,
    ) -> Result<ListWebhookEventsResponse, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId/events",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(delivery_failed) = q.delivery_failed {
                            map.insert("deliveryFailed".to_string(), delivery_failed.to_string());
                        }
                        if let Some(kind) = q.kind {
                            map.insert("kind".to_string(), kind.to_string());
                        }
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("pageToken".to_string(), token);
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

    pub async fn list_webhooks(
        &self,
        request: Option<ListWebhooksRequest>,
    ) -> Result<ListWebhooksResponse, DfnsError> {
        let path = build_path_and_query(
            "/webhooks",
            &crate::utils::url::PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit.to_string());
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("pageToken".to_string(), token);
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

    pub async fn ping_webhook_init(
        &self,
        request: PingWebhookRequest,
    ) -> Result<crate::signer::UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId/ping",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&serde_json::json!({}))?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn ping_webhook_complete(
        &self,
        request: PingWebhookRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<PingWebhookResponse, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId/ping",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
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
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(serde_json::json!({})),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn update_webhook_init(
        &self,
        request: UpdateWebhookRequest,
    ) -> Result<crate::signer::UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
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

    pub async fn update_webhook_complete(
        &self,
        request: UpdateWebhookRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<UpdateWebhookResponse, DfnsError> {
        let path = build_path_and_query(
            "/webhooks/:webhookId",
            &crate::utils::url::PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("webhookId".to_string(), request.webhook_id);
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
                body: Some(serde_json::to_value(&request.body)?),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }
}
