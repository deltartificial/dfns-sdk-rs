// @dfns-sdk-rs/src/api/webhooks/client.rs

use super::types::*;
use crate::{
    models::generic::DfnsApiClientOptions,
    utils::{
        fetch::simple_fetch,
        url::{build_path_and_query, PathAndQueryParams},
        user_action_fetch::user_action_fetch,
    },
};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct WebhooksClient {
    api_options: DfnsApiClientOptions,
}

impl WebhooksClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn create_webhook(
        &self,
        request: CreateWebhookRequest,
    ) -> Result<CreateWebhookResponse, crate::error::DfnsError> {
        let path = build_path_and_query(
            "/webhooks",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn delete_webhook(
        &self,
        request: DeleteWebhookRequest,
    ) -> Result<DeleteWebhookResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("webhookId".to_string(), request.webhook_id);

        let path = build_path_and_query(
            "/webhooks/:webhookId",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::DELETE,
                headers: None,
                body: Some(json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn get_webhook(
        &self,
        request: GetWebhookRequest,
    ) -> Result<GetWebhookResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("webhookId".to_string(), request.webhook_id);

        let path = build_path_and_query(
            "/webhooks/:webhookId",
            &PathAndQueryParams {
                path: path_params,
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

    pub async fn get_webhook_event(
        &self,
        request: GetWebhookEventRequest,
    ) -> Result<GetWebhookEventResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("webhookId".to_string(), request.webhook_id);
        path_params.insert("webhookEventId".to_string(), request.webhook_event_id);

        let path = build_path_and_query(
            "/webhooks/:webhookId/events/:webhookEventId",
            &PathAndQueryParams {
                path: path_params,
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

    pub async fn list_webhook_events(
        &self,
        request: ListWebhookEventsRequest,
    ) -> Result<ListWebhookEventsResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("webhookId".to_string(), request.webhook_id);

        let mut query_params = HashMap::new();
        if let Some(query) = request.query {
            if let Some(delivery_failed) = query.delivery_failed {
                query_params.insert("deliveryFailed".to_string(), delivery_failed.to_string());
            }
            if let Some(kind) = query.kind {
                query_params.insert("kind".to_string(), kind.to_string());
            }
            if let Some(limit) = query.limit {
                query_params.insert("limit".to_string(), limit.to_string());
            }
            if let Some(token) = query.pagination_token {
                query_params.insert("paginationToken".to_string(), token);
            }
        }

        let path = build_path_and_query(
            "/webhooks/:webhookId/events",
            &PathAndQueryParams {
                path: path_params,
                query: query_params,
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

    pub async fn list_webhooks(
        &self,
        request: Option<ListWebhooksRequest>,
    ) -> Result<ListWebhooksResponse, crate::error::DfnsError> {
        let mut query_params = HashMap::new();
        if let Some(req) = request {
            if let Some(query) = req.query {
                if let Some(limit) = query.limit {
                    query_params.insert("limit".to_string(), limit.to_string());
                }
                if let Some(token) = query.pagination_token {
                    query_params.insert("paginationToken".to_string(), token);
                }
            }
        }

        let path = build_path_and_query(
            "/webhooks",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: query_params,
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

    pub async fn ping_webhook(
        &self,
        request: PingWebhookRequest,
    ) -> Result<PingWebhookResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("webhookId".to_string(), request.webhook_id);

        let path = build_path_and_query(
            "/webhooks/:webhookId/ping",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!({})),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn update_webhook(
        &self,
        request: UpdateWebhookRequest,
    ) -> Result<UpdateWebhookResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("webhookId".to_string(), request.webhook_id);

        let path = build_path_and_query(
            "/webhooks/:webhookId",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::PUT,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }
}
