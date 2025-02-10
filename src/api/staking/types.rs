// @dfns-sdk-rs/src/api/staking/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeBody {
    pub amount: String,

    pub duration: Option<f64>,

    pub kind: CreateStakeBodyKind,

    pub protocol: Protocol,

    pub provider: Provider,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateStakeBodyKind {
    Native,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Protocol {
    Babylon,

    Ethereum,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Provider {
    Figment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeResponse {
    pub stake: CreateStakeResponseStake,

    pub stake_action: CreateStakeResponseStakeAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeResponseStake {
    pub date_created: String,

    pub id: String,

    pub protocol: Protocol,

    pub provider: Provider,

    pub provider_stake_id: String,

    pub request_body: TentacledRequestBody,

    pub requester: PurpleRequester,

    pub status: Status,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledRequestBody {
    pub amount: String,

    pub duration: Option<f64>,

    pub kind: CreateStakeBodyKind,

    pub protocol: Protocol,

    pub provider: Provider,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Active,

    Creating,

    Withdrawing,

    Withdrawn,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeResponseStakeAction {
    pub date_created: String,

    pub id: String,

    pub kind: StakeActionKind,

    pub request_body: PurpleRequestBody,

    pub requester: FluffyRequester,

    pub stake_id: String,

    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StakeActionKind {
    Stake,

    #[serde(rename = "StakeWithdrawal")]
    StakeWithdrawal,

    Unbond,

    #[serde(rename = "UnbondWithdrawal")]
    UnbondWithdrawal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRequestBody {
    pub amount: Option<String>,

    pub duration: Option<f64>,

    pub kind: PurpleKind,

    pub protocol: Protocol,

    pub provider: Option<Provider>,

    pub wallet_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PurpleKind {
    Native,

    #[serde(rename = "StakeWithdrawal")]
    StakeWithdrawal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateStakeRequest {
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub amount: String,

    pub duration: Option<f64>,

    pub kind: CreateStakeBodyKind,

    pub protocol: Protocol,

    pub provider: Provider,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeActionParams {
    pub stake_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeActionResponse {
    pub stake: CreateStakeActionResponseStake,

    pub stake_action: CreateStakeActionResponseStakeAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeActionResponseStake {
    pub date_created: String,

    pub id: String,

    pub protocol: Protocol,

    pub provider: Provider,

    pub provider_stake_id: String,

    pub request_body: StickyRequestBody,

    pub requester: TentacledRequester,

    pub status: Status,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyRequestBody {
    pub amount: String,

    pub duration: Option<f64>,

    pub kind: CreateStakeBodyKind,

    pub protocol: Protocol,

    pub provider: Provider,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeActionResponseStakeAction {
    pub date_created: String,

    pub id: String,

    pub kind: StakeActionKind,

    pub request_body: FluffyRequestBody,

    pub requester: StickyRequester,

    pub stake_id: String,

    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRequestBody {
    pub amount: Option<String>,

    pub duration: Option<f64>,

    pub kind: PurpleKind,

    pub protocol: Protocol,

    pub provider: Option<Provider>,

    pub wallet_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeActionRequest {
    pub body: CreateStakeActionBody,

    pub stake_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateStakeActionBody {
    pub kind: CreateStakeActionBodyKind,

    pub protocol: Protocol,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateStakeActionBodyKind {
    #[serde(rename = "StakeWithdrawal")]
    StakeWithdrawal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStakeRewardsParams {
    pub stake_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStakeRewardsResponse {
    pub balance: String,

    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStakeRewardsRequest {
    pub stake_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStakeActionsQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStakeActionsResponse {
    pub items: Vec<ListStakeActionsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStakeActionsResponseItem {
    pub date_created: String,

    pub id: String,

    pub kind: StakeActionKind,

    pub request_body: ItemRequestBody,

    pub requester: IndigoRequester,

    pub stake_id: String,

    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemRequestBody {
    pub amount: Option<String>,

    pub duration: Option<f64>,

    pub kind: PurpleKind,

    pub protocol: Protocol,

    pub provider: Option<Provider>,

    pub wallet_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListStakeActionsRequest {
    pub query: Option<ListStakeActionsRequestQuery>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStakeActionsRequestQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStakesQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStakesResponse {
    pub items: Vec<ListStakesResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStakesResponseItem {
    pub date_created: String,

    pub id: String,

    pub protocol: Protocol,

    pub provider: Provider,

    pub provider_stake_id: String,

    pub request_body: ItemRequestBodyClass,

    pub requester: IndecentRequester,

    pub status: Status,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemRequestBodyClass {
    pub amount: String,

    pub duration: Option<f64>,

    pub kind: CreateStakeBodyKind,

    pub protocol: Protocol,

    pub provider: Provider,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListStakesRequest {
    pub query: Option<ListStakesRequestQuery>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStakesRequestQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}
