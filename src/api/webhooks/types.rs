// @dfns-sdk-rs/src/api/webhooks/types.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WebhookStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WebhookEventKind {
    #[serde(rename = "policy.triggered")]
    PolicyTriggered,
    #[serde(rename = "policy.approval.pending")]
    PolicyApprovalPending,
    #[serde(rename = "policy.approval.resolved")]
    PolicyApprovalResolved,
    #[serde(rename = "wallet.blockchainevent.detected")]
    WalletBlockchainEventDetected,
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
    #[serde(rename = "wallet.tags.modified")]
    WalletTagsModified,
    #[serde(rename = "*")]
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub status: Option<WebhookStatus>,
    pub description: Option<String>,
    pub events: Vec<WebhookEventKind>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookResponse {
    pub id: String,
    pub url: String,
    pub events: Vec<WebhookEventKind>,
    pub status: WebhookStatus,
    pub description: Option<String>,
    pub date_created: String,
    pub date_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWebhookResponse {
    #[serde(flatten)]
    pub webhook: WebhookResponse,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub date: String,
    pub kind: WebhookEventKind,
    pub data: Value,
    pub status: String,
    pub error: Option<String>,
    pub timestamp_sent: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListWebhookEventsRequest {
    pub webhook_id: String,
    pub kind: Option<WebhookEventKind>,
    pub delivery_failed: Option<bool>,
    pub limit: Option<u32>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListWebhookEventsResponse {
    pub items: Vec<WebhookEvent>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListWebhooksRequest {
    pub limit: Option<u32>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListWebhooksResponse {
    pub items: Vec<WebhookResponse>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingWebhookRequest {
    pub webhook_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingWebhookResponse {
    pub status: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWebhookRequest {
    pub webhook_id: String,
    pub url: Option<String>,
    pub description: Option<Option<String>>,
    pub events: Option<Vec<WebhookEventKind>>,
    pub status: Option<Option<WebhookStatus>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteWebhookRequest {
    pub webhook_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteWebhookResponse {
    pub deleted: bool,
}