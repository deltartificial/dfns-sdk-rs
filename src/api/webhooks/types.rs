// @dfns-sdk-rs/src/api/webhooks/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookBody {
    pub description: Option<String>,

    pub events: Vec<Event>,

    pub status: Option<Status>,

    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    #[default]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Disabled,

    Enabled,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookResponse {
    pub date_created: String,

    pub date_updated: String,

    pub description: Option<String>,

    pub events: Vec<Event>,

    pub id: String,

    pub secret: String,

    pub status: Status,

    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub body: CreateWebhookRequestBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookRequestBody {
    pub description: Option<String>,

    pub events: Vec<Event>,

    pub status: Option<Status>,

    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWebhookParams {
    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWebhookResponse {
    pub deleted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWebhookRequest {
    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookParams {
    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookResponse {
    pub date_created: String,

    pub date_updated: String,

    pub description: Option<String>,

    pub events: Vec<Event>,

    pub id: String,

    pub status: Status,

    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookRequest {
    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookEventParams {
    pub webhook_event_id: String,

    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookEventResponse {
    pub data: HashMap<String, Option<serde_json::Value>>,

    pub date: String,

    pub error: Option<String>,

    pub id: String,

    pub kind: Kind,

    pub status: String,

    pub timestamp_sent: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookEventRequest {
    pub webhook_event_id: String,

    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsParams {
    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsQuery {
    pub delivery_failed: Option<DeliveryFailed>,

    pub kind: Option<Kind>,

    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryFailed {
    #[default]
    False,

    True,
}

impl std::fmt::Display for DeliveryFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsResponse {
    pub items: Vec<ListWebhookEventsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsResponseItem {
    pub data: HashMap<String, Option<serde_json::Value>>,

    pub date: String,

    pub error: Option<String>,

    pub id: String,

    pub kind: Kind,

    pub status: String,

    pub timestamp_sent: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsRequest {
    pub query: Option<ListWebhookEventsRequestQuery>,

    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhookEventsRequestQuery {
    pub delivery_failed: Option<DeliveryFailed>,

    pub kind: Option<Kind>,

    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhooksQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhooksResponse {
    pub items: Vec<ListWebhooksResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhooksResponseItem {
    pub date_created: String,

    pub date_updated: String,

    pub description: Option<String>,

    pub events: Vec<Event>,

    pub id: String,

    pub status: Status,

    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListWebhooksRequest {
    pub query: Option<ListWebhooksRequestQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWebhooksRequestQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingWebhookParams {
    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PingWebhookResponse {
    pub error: Option<String>,

    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingWebhookRequest {
    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhookBody {
    pub description: Option<String>,

    pub events: Option<Vec<Event>>,

    pub status: Option<Status>,

    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookParams {
    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookResponse {
    pub date_created: String,

    pub date_updated: String,

    pub description: Option<String>,

    pub events: Vec<Event>,

    pub id: String,

    pub status: Status,

    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookRequest {
    pub body: UpdateWebhookRequestBody,

    pub webhook_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhookRequestBody {
    pub description: Option<String>,

    pub events: Option<Vec<Event>>,

    pub status: Option<Status>,

    pub url: Option<String>,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PolicyApprovalPending => write!(f, "policy.approval.pending"),
            Self::PolicyApprovalResolved => write!(f, "policy.approval.resolved"),
            Self::PolicyTriggered => write!(f, "policy.triggered"),
            Self::WalletBlockchaineventDetected => write!(f, "wallet.blockchainevent.detected"),
            Self::WalletCreated => write!(f, "wallet.created"),
            Self::WalletDelegated => write!(f, "wallet.delegated"),
            Self::WalletExported => write!(f, "wallet.exported"),
            Self::WalletSignatureFailed => write!(f, "wallet.signature.failed"),
            Self::WalletSignatureRejected => write!(f, "wallet.signature.rejected"),
            Self::WalletSignatureRequested => write!(f, "wallet.signature.requested"),
            Self::WalletSignatureSigned => write!(f, "wallet.signature.signed"),
            Self::WalletTagsModified => write!(f, "wallet.tags.modified"),
            Self::WalletTransactionBroadcasted => write!(f, "wallet.transaction.broadcasted"),
            Self::WalletTransactionConfirmed => write!(f, "wallet.transaction.confirmed"),
            Self::WalletTransactionFailed => write!(f, "wallet.transaction.failed"),
            Self::WalletTransactionRejected => write!(f, "wallet.transaction.rejected"),
            Self::WalletTransactionRequested => write!(f, "wallet.transaction.requested"),
            Self::WalletTransferBroadcasted => write!(f, "wallet.transfer.broadcasted"),
            Self::WalletTransferConfirmed => write!(f, "wallet.transfer.confirmed"),
            Self::WalletTransferFailed => write!(f, "wallet.transfer.failed"),
            Self::WalletTransferRejected => write!(f, "wallet.transfer.rejected"),
            Self::WalletTransferRequested => write!(f, "wallet.transfer.requested"),
            Self::None => write!(f, "*"),
        }
    }
}
