// @dfns-sdk-rs/src/api/wallets/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority {
    Slow,
    Standard,
    Fast,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Executing,
    Broadcasted,
    Confirmed,
    Failed,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetMetadata {
    pub symbol: Option<String>,
    pub decimals: Option<u32>,
    pub verified: Option<bool>,
    pub quotes: Option<HashMap<String, f64>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub asset: AssetMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum TransferRequestBody {
    Native {
        to: String,
        amount: String,
        memo: Option<String>,
        priority: Option<Priority>,
        create_destination_account: Option<bool>,
        external_id: Option<String>,
    },
    Asa {
        asset_id: String,
        to: String,
        amount: String,
        external_id: Option<String>,
    },
    Aip21 {
        metadata: String,
        to: String,
        amount: String,
        external_id: Option<String>,
    },
    Erc20 {
        contract: String,
        to: String,
        amount: String,
        priority: Option<Priority>,
        external_id: Option<String>,
    },
    Erc721 {
        contract: String,
        to: String,
        token_id: String,
        priority: Option<Priority>,
        external_id: Option<String>,
    },
    Sep41 {
        issuer: String,
        asset_code: String,
        to: String,
        amount: String,
        memo: Option<String>,
        external_id: Option<String>,
    },
    #[serde(rename = "Spl")]
    Spl {
        to: String,
        amount: String,
        mint: String,
        create_destination_account: Option<bool>,
        external_id: Option<String>,
    },
    #[serde(rename = "Spl2022")]
    Spl2022 {
        to: String,
        amount: String,
        mint: String,
        create_destination_account: Option<bool>,
        external_id: Option<String>,
    },
    Tep74 {
        to: String,
        master: String,
        amount: String,
        memo: Option<String>,
        external_id: Option<String>,
    },
    Trc10 {
        token_id: String,
        to: String,
        amount: String,
        external_id: Option<String>,
    },
    Trc20 {
        contract: String,
        to: String,
        amount: String,
        external_id: Option<String>,
    },
    Trc721 {
        contract: String,
        to: String,
        token_id: String,
        external_id: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum TransactionRequestBody {
    Transaction {
        transaction: String,
        external_id: Option<String>,
    },
    Evm {
        to: Option<String>,
        value: Option<String>,
        data: Option<String>,
        nonce: Option<String>,
        gas_limit: Option<String>,
        external_id: Option<String>,
    },
    Eip1559 {
        to: Option<String>,
        value: Option<String>,
        data: Option<String>,
        nonce: Option<String>,
        gas_limit: Option<String>,
        max_fee_per_gas: Option<String>,
        max_priority_fee_per_gas: Option<String>,
        external_id: Option<String>,
    },
    EvmLegacy {
        to: Option<String>,
        value: Option<String>,
        data: Option<String>,
        nonce: Option<String>,
        gas_limit: Option<String>,
        gas_price: Option<String>,
        external_id: Option<String>,
    },
    Psbt {
        psbt: String,
        external_id: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAssetRequest {
    pub wallet_id: String,
    #[serde(flatten)]
    pub body: TransferRequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAssetResponse {
    pub id: String,
    pub wallet_id: String,
    pub request_body: TransferRequestBody,
    pub metadata: Metadata,
    pub status: TransactionStatus,
    pub reason: Option<String>,
    pub tx_hash: Option<String>,
    pub fee: Option<String>,
    pub date_requested: String,
    pub date_policy_resolved: Option<String>,
    pub date_broadcasted: Option<String>,
    pub date_confirmed: Option<String>,
    pub approval_id: Option<String>,
    pub external_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTransfersRequest {
    pub wallet_id: String,
    pub limit: Option<String>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTransfersResponse {
    pub wallet_id: String,
    pub items: Vec<TransferAssetResponse>,
    pub next_page_token: Option<String>,
}

// Additional types would follow similar patterns...