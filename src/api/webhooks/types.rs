// @dfns-sdk-rs/src/api/webhooks/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookBody {
    pub description: Option<String>,

    pub events: Vec<Event>,

    /// Webhook status
    pub status: Option<Status>,

    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "*")]
    Empty,

    #[serde(rename = "policy.approval.pending")]
    PolicyApprovalPending,

    #[serde(rename = "policy.approval.resolved")]
    PolicyApprovalResolved,

    #[serde(rename = "policy.triggered")]
    PolicyTriggered,

    #[serde(rename = "wallet.blockchainevent.detected")]
    WalletBlockchaineventDetected,

    #[serde(rename = "wallet.created")]
    WalletCreated,

    #[serde(rename = "wallet.delegated")]
    WalletDelegated,

    #[serde(rename = "wallet.exported")]
    WalletExported,

    #[serde(rename = "wallet.signature.failed")]
    WalletSignatureFailed,

    #[serde(rename = "wallet.signature.rejected")]
    WalletSignatureRejected,

    #[serde(rename = "wallet.signature.requested")]
    WalletSignatureRequested,

    #[serde(rename = "wallet.signature.signed")]
    WalletSignatureSigned,

    #[serde(rename = "wallet.tags.modified")]
    WalletTagsModified,

    #[serde(rename = "wallet.transaction.broadcasted")]
    WalletTransactionBroadcasted,

    #[serde(rename = "wallet.transaction.confirmed")]
    WalletTransactionConfirmed,

    #[serde(rename = "wallet.transaction.failed")]
    WalletTransactionFailed,

    #[serde(rename = "wallet.transaction.rejected")]
    WalletTransactionRejected,

    #[serde(rename = "wallet.transaction.requested")]
    WalletTransactionRequested,

    #[serde(rename = "wallet.transfer.broadcasted")]
    WalletTransferBroadcasted,

    #[serde(rename = "wallet.transfer.confirmed")]
    WalletTransferConfirmed,

    #[serde(rename = "wallet.transfer.failed")]
    WalletTransferFailed,

    #[serde(rename = "wallet.transfer.rejected")]
    WalletTransferRejected,

    #[serde(rename = "wallet.transfer.requested")]
    WalletTransferRequested,
}

/// Webhook status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Disabled,

    Enabled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookResponse {
    /// Date when webhook was created
    pub date_created: String,

    /// Date when webhook was last updated
    pub date_updated: String,

    /// Short description this webhook's purpose
    pub description: Option<String>,

    /// All events this webhook is subscribed to.
    pub events: Vec<Event>,

    /// Webhook ID
    pub id: String,

    /// The secret associated with this webhook, with which webhook requests will be signed.
    pub secret: String,

    /// Webhook status
    pub status: Status,

    /// Webhook url
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub body: CreateWebhookRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookRequestBody {
    pub description: Option<String>,

    pub events: Vec<Event>,

    /// Webhook status
    pub status: Option<Status>,

    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWebhookParams {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWebhookResponse {
    pub deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWebhookRequest {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookParams {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookResponse {
    /// Date when webhook was created
    pub date_created: String,

    /// Date when webhook was last updated
    pub date_updated: String,

    /// Short description this webhook's purpose
    pub description: Option<String>,

    /// All events this webhook is subscribed to.
    pub events: Vec<Event>,

    /// Webhook ID
    pub id: String,

    /// Webhook status
    pub status: Status,

    /// Webhook url
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookRequest {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookEventParams {
    pub webhook_event_id: String,

    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookEventResponse {
    pub data: HashMap<String, Option<serde_json::Value>>,

    /// ISO date string when event was raised
    pub date: String,

    /// Error message if any error happened during the webhook request.
    pub error: Option<String>,

    /// WebhookEvent ID
    pub id: String,

    /// Webhook event
    pub kind: Kind,

    /// Status code of the webhook request
    pub status: String,

    /// Unix timestamp when the event was forwarded to the webhook url by our servers.
    pub timestamp_sent: f64,
}

/// Webhook event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "policy.approval.pending")]
    PolicyApprovalPending,

    #[serde(rename = "policy.approval.resolved")]
    PolicyApprovalResolved,

    #[serde(rename = "policy.triggered")]
    PolicyTriggered,

    #[serde(rename = "wallet.blockchainevent.detected")]
    WalletBlockchaineventDetected,

    #[serde(rename = "wallet.created")]
    WalletCreated,

    #[serde(rename = "wallet.delegated")]
    WalletDelegated,

    #[serde(rename = "wallet.exported")]
    WalletExported,

    #[serde(rename = "wallet.signature.failed")]
    WalletSignatureFailed,

    #[serde(rename = "wallet.signature.rejected")]
    WalletSignatureRejected,

    #[serde(rename = "wallet.signature.requested")]
    WalletSignatureRequested,

    #[serde(rename = "wallet.signature.signed")]
    WalletSignatureSigned,

    #[serde(rename = "wallet.tags.modified")]
    WalletTagsModified,

    #[serde(rename = "wallet.transaction.broadcasted")]
    WalletTransactionBroadcasted,

    #[serde(rename = "wallet.transaction.confirmed")]
    WalletTransactionConfirmed,

    #[serde(rename = "wallet.transaction.failed")]
    WalletTransactionFailed,

    #[serde(rename = "wallet.transaction.rejected")]
    WalletTransactionRejected,

    #[serde(rename = "wallet.transaction.requested")]
    WalletTransactionRequested,

    #[serde(rename = "wallet.transfer.broadcasted")]
    WalletTransferBroadcasted,

    #[serde(rename = "wallet.transfer.confirmed")]
    WalletTransferConfirmed,

    #[serde(rename = "wallet.transfer.failed")]
    WalletTransferFailed,

    #[serde(rename = "wallet.transfer.rejected")]
    WalletTransferRejected,

    #[serde(rename = "wallet.transfer.requested")]
    WalletTransferRequested,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookEventRequest {
    pub webhook_event_id: String,

    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsParams {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsQuery {
    pub delivery_failed: Option<DeliveryFailed>,

    pub kind: Option<Kind>,

    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryFailed {
    False,

    True,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsResponse {
    pub items: Vec<ListWebhookEventsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsResponseItem {
    pub data: HashMap<String, Option<serde_json::Value>>,

    /// ISO date string when event was raised
    pub date: String,

    /// Error message if any error happened during the webhook request.
    pub error: Option<String>,

    /// WebhookEvent ID
    pub id: String,

    /// Webhook event
    pub kind: Kind,

    /// Status code of the webhook request
    pub status: String,

    /// Unix timestamp when the event was forwarded to the webhook url by our servers.
    pub timestamp_sent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsRequest {
    pub query: Option<ListWebhookEventsRequestQuery>,

    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsRequestQuery {
    pub delivery_failed: Option<DeliveryFailed>,

    pub kind: Option<Kind>,

    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhooksQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhooksResponse {
    pub items: Vec<ListWebhooksResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhooksResponseItem {
    /// Date when webhook was created
    pub date_created: String,

    /// Date when webhook was last updated
    pub date_updated: String,

    /// Short description this webhook's purpose
    pub description: Option<String>,

    /// All events this webhook is subscribed to.
    pub events: Vec<Event>,

    /// Webhook ID
    pub id: String,

    /// Webhook status
    pub status: Status,

    /// Webhook url
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListWebhooksRequest {
    pub query: Option<ListWebhooksRequestQuery>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhooksRequestQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingWebhookParams {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PingWebhookResponse {
    pub error: Option<String>,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingWebhookRequest {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhookBody {
    pub description: Option<String>,

    pub events: Option<Vec<Event>>,

    /// Webhook status
    pub status: Option<Status>,

    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookParams {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookResponse {
    /// Date when webhook was created
    pub date_created: String,

    /// Date when webhook was last updated
    pub date_updated: String,

    /// Short description this webhook's purpose
    pub description: Option<String>,

    /// All events this webhook is subscribed to.
    pub events: Vec<Event>,

    /// Webhook ID
    pub id: String,

    /// Webhook status
    pub status: Status,

    /// Webhook url
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookRequest {
    pub body: UpdateWebhookRequestBody,

    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhookRequestBody {
    pub description: Option<String>,

    pub events: Option<Vec<Event>>,

    /// Webhook status
    pub status: Option<Status>,

    pub url: Option<String>,
}
