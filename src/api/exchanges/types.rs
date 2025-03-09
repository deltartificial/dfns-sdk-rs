// @dfns-sdk-rs/src/api/exchanges/types.rs

use serde::{Deserialize, Serialize};

pub type ListAssetWithdrawalNetworksResponse = Vec<ListAssetWithdrawalNetworksResponseElement>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepositParams {
    pub account_id: String,

    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepositResponse {
    pub account_id: String,

    pub date_created: String,

    pub exchange_id: String,

    pub exchange_reference: Option<String>,

    pub id: String,

    pub kind: CreateDepositResponseKind,

    pub request_body: CreateDepositResponseRequestBody,

    pub requester: CreateDepositResponseRequester,

    pub transfer_id: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateDepositResponseKind {
    Deposit,

    Withdrawal,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepositResponseRequestBody {
    pub amount: String,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: CreateDepositBodyKind,

    pub otp: Option<String>,

    pub priority: Option<Priority>,

    pub wallet_id: String,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateDepositBodyKind {
    Asa,

    Erc20,

    Native,

    Sep41,

    Spl,

    Spl2022,

    Tep74,

    Trc10,

    Trc20,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Fast,

    Slow,

    Standard,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepositResponseRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepositRequest {
    pub account_id: String,

    pub body: CreateDepositBody,

    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepositBody {
    pub amount: String,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: CreateDepositBodyKind,

    pub otp: Option<String>,

    pub priority: Option<Priority>,

    pub wallet_id: String,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExchangeBody {
    pub kind: CreateExchangeBodyKind,

    pub name: Option<String>,

    pub read_configuration: CreateExchangeBodyReadConfiguration,

    pub write_configuration: CreateExchangeBodyWriteConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateExchangeBodyKind {
    Binance,

    #[serde(rename = "CoinbaseApp")]
    CoinbaseApp,

    #[serde(rename = "CoinbasePrime")]
    CoinbasePrime,

    Kraken,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExchangeBodyReadConfiguration {
    pub otp: Option<String>,

    pub password: Option<String>,

    pub private_api_key: String,

    pub public_api_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExchangeBodyWriteConfiguration {
    pub otp: Option<String>,

    pub password: Option<String>,

    pub private_api_key: String,

    pub public_api_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExchangeResponse {
    pub date_created: String,

    pub id: String,

    pub kind: CreateExchangeBodyKind,

    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateExchangeRequest {
    pub body: Body,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub kind: CreateExchangeBodyKind,

    pub name: Option<String>,

    pub read_configuration: BodyReadConfiguration,

    pub write_configuration: BodyWriteConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyReadConfiguration {
    pub otp: Option<String>,

    pub password: Option<String>,

    pub private_api_key: String,

    pub public_api_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyWriteConfiguration {
    pub otp: Option<String>,

    pub password: Option<String>,

    pub private_api_key: String,

    pub public_api_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWithdrawalParams {
    pub account_id: String,

    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWithdrawalResponse {
    pub account_id: String,

    pub date_created: String,

    pub exchange_id: String,

    pub exchange_reference: Option<String>,

    pub id: String,

    pub kind: CreateDepositResponseKind,

    pub request_body: CreateWithdrawalResponseRequestBody,

    pub requester: CreateWithdrawalResponseRequester,

    pub transfer_id: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWithdrawalResponseRequestBody {
    pub amount: String,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: CreateDepositBodyKind,

    pub otp: Option<String>,

    pub priority: Option<Priority>,

    pub wallet_id: String,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWithdrawalResponseRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWithdrawalRequest {
    pub account_id: String,

    pub body: CreateWithdrawalBody,

    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWithdrawalBody {
    pub amount: String,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: CreateDepositBodyKind,

    pub otp: Option<String>,

    pub priority: Option<Priority>,

    pub wallet_id: String,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteExchangeParams {
    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteExchangeResponse {
    pub deleted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteExchangeRequest {
    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExchangeParams {
    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExchangeResponse {
    pub date_created: String,

    pub id: String,

    pub kind: CreateExchangeBodyKind,

    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExchangeRequest {
    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountAssetsParams {
    pub account_id: String,

    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountAssetsQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountAssetsResponse {
    pub items: Vec<ListAccountAssetsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListAccountAssetsResponseItem {
    pub balance: String,

    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountAssetsRequest {
    pub account_id: String,

    pub exchange_id: String,

    pub query: Option<ListAccountAssetsRequestQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountAssetsRequestQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsParams {
    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsResponse {
    pub items: Vec<ListAccountsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsResponseItem {
    pub exchange_id: String,

    pub exchange_name: Option<String>,

    pub id: String,

    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsRequest {
    pub exchange_id: String,

    pub query: Option<ListAccountsRequestQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsRequestQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetWithdrawalNetworksParams {
    pub account_id: String,

    pub asset: String,

    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetWithdrawalNetworksResponseElement {
    pub decimals: f64,

    pub kind: ListAssetWithdrawalNetworksResponseKind,

    pub network: Network,

    pub metadata: Option<String>,

    pub asset_id: Option<String>,

    pub contract: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub token_id: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListAssetWithdrawalNetworksResponseKind {
    Aip21,

    Asa,

    Erc20,

    Native,

    Sep41,

    Spl,

    Spl2022,

    Tep74,

    Trc10,

    Trc20,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Network {
    Algorand,

    #[serde(rename = "AlgorandTestnet")]
    AlgorandTestnet,

    Aptos,

    #[serde(rename = "AptosTestnet")]
    AptosTestnet,

    #[serde(rename = "ArbitrumGoerli")]
    ArbitrumGoerli,

    #[serde(rename = "ArbitrumOne")]
    ArbitrumOne,

    #[serde(rename = "ArbitrumSepolia")]
    ArbitrumSepolia,

    #[serde(rename = "AvalancheC")]
    AvalancheC,

    #[serde(rename = "AvalancheCFuji")]
    AvalancheCFuji,

    Base,

    #[serde(rename = "BaseGoerli")]
    BaseGoerli,

    #[serde(rename = "BaseSepolia")]
    BaseSepolia,

    Berachain,

    #[serde(rename = "BerachainBArtio")]
    BerachainBArtio,

    Bitcoin,

    #[serde(rename = "BitcoinSignet")]
    BitcoinSignet,

    #[serde(rename = "BitcoinTestnet3")]
    BitcoinTestnet3,

    Bsc,

    #[serde(rename = "BscTestnet")]
    BscTestnet,

    Canton,

    #[serde(rename = "CantonDevnet")]
    CantonDevnet,

    #[serde(rename = "CantonTestnet")]
    CantonTestnet,

    Cardano,

    #[serde(rename = "CardanoPreprod")]
    CardanoPreprod,

    Celo,

    #[serde(rename = "CeloAlfajores")]
    CeloAlfajores,

    Dogecoin,

    #[serde(rename = "DogecoinTestnet")]
    DogecoinTestnet,

    Ethereum,

    #[serde(rename = "EthereumGoerli")]
    EthereumGoerli,

    #[serde(rename = "EthereumHolesky")]
    EthereumHolesky,

    #[serde(rename = "EthereumSepolia")]
    EthereumSepolia,

    #[serde(rename = "FantomOpera")]
    FantomOpera,

    #[serde(rename = "FantomTestnet")]
    FantomTestnet,

    #[serde(rename = "InternetComputer")]
    InternetComputer,

    Ion,

    #[serde(rename = "IonTestnet")]
    IonTestnet,

    Iota,

    #[serde(rename = "IotaTestnet")]
    IotaTestnet,

    Kaspa,

    #[serde(rename = "KaspaTestnet11")]
    KaspaTestnet11,

    Kusama,

    Litecoin,

    #[serde(rename = "LitecoinTestnet")]
    LitecoinTestnet,

    Optimism,

    #[serde(rename = "OptimismGoerli")]
    OptimismGoerli,

    #[serde(rename = "OptimismSepolia")]
    OptimismSepolia,

    Origyn,

    Polkadot,

    Polygon,

    #[serde(rename = "PolygonAmoy")]
    PolygonAmoy,

    #[serde(rename = "PolygonMumbai")]
    PolygonMumbai,

    Polymesh,

    #[serde(rename = "PolymeshTestnet")]
    PolymeshTestnet,

    Race,

    #[serde(rename = "RaceSepolia")]
    RaceSepolia,

    #[serde(rename = "SeiAtlantic2")]
    SeiAtlantic2,

    #[serde(rename = "SeiPacific1")]
    SeiPacific1,

    Solana,

    #[serde(rename = "SolanaDevnet")]
    SolanaDevnet,

    Stellar,

    #[serde(rename = "StellarTestnet")]
    StellarTestnet,

    Tezos,

    #[serde(rename = "TezosGhostnet")]
    TezosGhostnet,

    Ton,

    #[serde(rename = "TonTestnet")]
    TonTestnet,

    Tron,

    #[serde(rename = "TronNile")]
    TronNile,

    Westend,

    #[serde(rename = "XrpLedger")]
    XrpLedger,

    #[serde(rename = "XrpLedgerTestnet")]
    XrpLedgerTestnet,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetWithdrawalNetworksRequest {
    pub account_id: String,

    pub asset: String,

    pub exchange_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListExchangesQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListExchangesResponse {
    pub items: Vec<ListExchangesResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListExchangesResponseItem {
    pub date_created: String,

    pub id: String,

    pub kind: CreateExchangeBodyKind,

    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListExchangesRequest {
    pub query: Option<ListExchangesRequestQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListExchangesRequestQuery {
    pub limit: Option<f64>,

    pub pagination_token: Option<String>,
}
