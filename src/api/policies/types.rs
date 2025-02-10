// @dfns-sdk-rs/src/api/policies/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ActivityKind {
    #[serde(rename = "Permissions:Assign")]
    PermissionsAssign,
    #[serde(rename = "Permissions:Modify")]
    PermissionsModify,
    #[serde(rename = "Policies:Modify")]
    PoliciesModify,
    #[serde(rename = "Wallets:Sign")]
    WalletsSign,
    #[serde(rename = "Wallets:IncomingTransaction")]
    WalletsIncomingTransaction,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AlertLevel {
    LOW,
    MEDIUM,
    HIGH,
    SEVERE,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum PolicyRule {
    AlwaysTrigger {
        configuration: Option<serde_json::Value>,
    },
    TransactionRecipientWhitelist {
        configuration: WhitelistConfig,
    },
    TransactionAmountLimit {
        configuration: AmountLimitConfig,
    },
    TransactionAmountVelocity {
        configuration: AmountVelocityConfig,
    },
    TransactionCountVelocity {
        configuration: CountVelocityConfig,
    },
    ChainalysisTransactionPrescreening {
        configuration: ChainalysisConfig,
    },
    ChainalysisTransactionScreening {
        configuration: ChainalysisScreeningConfig,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhitelistConfig {
    pub addresses: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmountLimitConfig {
    pub limit: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmountVelocityConfig {
    pub limit: f64,
    pub currency: String,
    pub timeframe: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountVelocityConfig {
    pub limit: u64,
    pub timeframe: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainalysisAlerts {
    pub alert_level: AlertLevel,
    pub category_ids: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectExposure {
    pub category_ids: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FallbackBehaviours {
    pub skip_unscreenable_transaction: bool,
    pub skip_unsupported_network: bool,
    pub skip_unsupported_asset: bool,
    pub skip_chainalysis_failure: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainalysisConfig {
    pub alerts: ChainalysisAlerts,
    pub exposures: ChainalysisExposures,
    pub addresses: ChainalysisAddresses,
    pub fallback_behaviours: FallbackBehaviours,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainalysisExposures {
    pub direct: DirectExposure,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainalysisAddresses {
    pub category_ids: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainalysisScreeningConfig {
    pub alerts: ChainalysisAlerts,
    pub exposures: ChainalysisExposures,
    pub fallback_behaviours: FallbackBehaviours,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalGroup {
    pub name: Option<String>,
    pub quorum: u32,
    pub approvers: Approvers,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Approvers {
    pub user_id: Option<UserIdFilter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserIdFilter {
    pub r#in: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum PolicyAction {
    RequestApproval {
        approval_groups: Vec<ApprovalGroup>,
        auto_reject_timeout: Option<u64>,
    },
    Block,
    NoAction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletFilters {
    pub wallet_id: Option<WalletIdFilter>,
    pub wallet_tags: Option<WalletTags>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletIdFilter {
    pub r#in: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletTags {
    pub has_any: Option<Vec<String>>,
    pub has_all: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub activity_kind: ActivityKind,
    pub rule: PolicyRule,
    pub action: PolicyAction,
    pub filters: Option<WalletFilters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub status: String,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub activity_kind: ActivityKind,
    pub rule: PolicyRule,
    pub action: PolicyAction,
    pub filters: Option<WalletFilters>,
}