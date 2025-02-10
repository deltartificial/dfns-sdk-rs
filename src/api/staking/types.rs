// @dfns-sdk-rs/src/api/staking/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Provider {
    Figment,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Protocol {
    Babylon,
    Ethereum,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum StakeStatus {
    Creating,
    Active,
    Withdrawing,
    Withdrawn,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum StakeActionKind {
    Stake,
    Unbond,
    UnbondWithdrawal,
    StakeWithdrawal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Requester {
    pub user_id: String,
    pub token_id: Option<String>,
    pub app_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum StakeRequestBody {
    Native {
        amount: String,
        wallet_id: String,
        provider: Provider,
        protocol: Protocol,
        duration: Option<u64>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StakeActionRequestBody {
    Native(StakeRequestBody),
    Withdrawal {
        protocol: Protocol,
        kind: StakeActionKind,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stake {
    pub id: String,
    pub provider: Provider,
    pub provider_stake_id: String,
    pub wallet_id: String,
    pub protocol: Protocol,
    pub status: StakeStatus,
    pub requester: Requester,
    pub request_body: StakeRequestBody,
    pub date_created: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StakeAction {
    pub id: String,
    pub stake_id: String,
    pub transaction_id: Option<String>,
    pub kind: StakeActionKind,
    pub requester: Requester,
    pub request_body: StakeActionRequestBody,
    pub date_created: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStakeRequest {
    #[serde(flatten)]
    pub body: StakeRequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStakeResponse {
    pub stake: Stake,
    pub stake_action: StakeAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStakeActionRequest {
    pub stake_id: String,
    pub protocol: Protocol,
    pub kind: StakeActionKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStakeActionResponse {
    pub stake: Stake,
    pub stake_action: StakeAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStakeRewardsRequest {
    pub stake_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StakeRewards {
    pub symbol: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStakeRewardsResponse(Option<StakeRewards>);

#[derive(Debug, Serialize, Deserialize)]
pub struct ListStakeActionsRequest {
    pub limit: Option<u32>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListStakeActionsResponse {
    pub items: Vec<StakeAction>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListStakesRequest {
    pub limit: Option<u32>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListStakesResponse {
    pub items: Vec<Stake>,
    pub next_page_token: Option<String>,
}