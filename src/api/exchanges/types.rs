// @dfns-sdk-rs/src/api/exchanges/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority {
    Slow,
    Standard,
    Fast,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ExchangeKind {
    Binance,
    Kraken,
    CoinbaseApp,
    CoinbasePrime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransferKind {
    Withdrawal,
    Deposit,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum TransferRequestBody {
    Native {
        amount: String,
        priority: Option<Priority>,
        create_destination_account: Option<bool>,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
    Erc20 {
        contract: String,
        amount: String,
        priority: Option<Priority>,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
    Trc10 {
        token_id: String,
        amount: String,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
    Trc20 {
        contract: String,
        amount: String,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
    Asa {
        asset_id: String,
        amount: String,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
    Sep41 {
        issuer: String,
        asset_code: String,
        amount: String,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
    #[serde(rename = "Spl")]
    Spl {
        amount: String,
        mint: String,
        create_destination_account: Option<bool>,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
    #[serde(rename = "Spl2022")]
    Spl2022 {
        amount: String,
        mint: String,
        create_destination_account: Option<bool>,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
    Tep74 {
        master: String,
        amount: String,
        external_id: Option<String>,
        wallet_id: String,
        otp: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requester {
    pub user_id: String,
    pub token_id: Option<String>,
    pub app_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDepositRequest {
    pub exchange_id: String,
    pub account_id: String,
    #[serde(flatten)]
    pub body: TransferRequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDepositResponse {
    pub id: String,
    pub exchange_id: String,
    pub account_id: String,
    pub transfer_id: Option<String>,
    pub exchange_reference: Option<String>,
    pub kind: TransferKind,
    pub wallet_id: String,
    pub requester: Requester,
    pub request_body: TransferRequestBody,
    pub date_created: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeConfiguration {
    pub public_api_key: String,
    pub private_api_key: String,
    pub password: Option<String>,
    pub otp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExchangeRequest {
    pub name: Option<String>,
    pub kind: ExchangeKind,
    pub read_configuration: ExchangeConfiguration,
    pub write_configuration: ExchangeConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExchangeResponse {
    pub id: String,
    pub name: Option<String>,
    pub kind: ExchangeKind,
    pub date_created: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAccountAssetsRequest {
    pub exchange_id: String,
    pub account_id: String,
    pub limit: Option<u32>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub symbol: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAccountAssetsResponse {
    pub items: Vec<Asset>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: Option<String>,
    pub exchange_id: String,
    pub exchange_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAccountsResponse {
    pub items: Vec<Account>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteExchangeResponse {
    pub deleted: bool,
}