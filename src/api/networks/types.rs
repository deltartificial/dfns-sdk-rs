// @dfns-sdk-rs/src/api/networks/types.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFeesQuery {
    pub network: GetFeesQueryNetwork,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetFeesQueryNetwork {
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

    Celo,

    #[serde(rename = "CeloAlfajores")]
    CeloAlfajores,

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

    Optimism,

    #[serde(rename = "OptimismGoerli")]
    OptimismGoerli,

    #[serde(rename = "OptimismSepolia")]
    OptimismSepolia,

    Polygon,

    #[serde(rename = "PolygonAmoy")]
    PolygonAmoy,

    #[serde(rename = "PolygonMumbai")]
    PolygonMumbai,

    Race,

    #[serde(rename = "RaceSepolia")]
    RaceSepolia,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeesResponse {
    pub block_number: f64,

    pub fast: Fast,

    pub kind: GetFeesResponseKind,

    pub network: GetFeesResponseNetwork,

    pub slow: Slow,

    pub standard: Standard,

    pub estimated_base_fee: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fast {
    pub block_horizon: Option<f64>,

    pub fee_rate: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetFeesResponseKind {
    Bitcoin,

    Eip1559,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetFeesResponseNetwork {
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

    Litecoin,

    #[serde(rename = "LitecoinTestnet")]
    LitecoinTestnet,

    Optimism,

    #[serde(rename = "OptimismGoerli")]
    OptimismGoerli,

    #[serde(rename = "OptimismSepolia")]
    OptimismSepolia,

    Polygon,

    #[serde(rename = "PolygonAmoy")]
    PolygonAmoy,

    #[serde(rename = "PolygonMumbai")]
    PolygonMumbai,

    Race,

    #[serde(rename = "RaceSepolia")]
    RaceSepolia,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slow {
    pub block_horizon: Option<f64>,

    pub fee_rate: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standard {
    pub block_horizon: Option<f64>,

    pub fee_rate: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFeesRequest {
    pub query: Option<Query>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub network: GetFeesQueryNetwork,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadContractBody {
    pub contract: String,

    pub data: String,

    pub kind: ReadContractBodyKind,

    pub network: ReadContractBodyNetwork,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReadContractBodyKind {
    Evm,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReadContractBodyNetwork {
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

    Bsc,

    #[serde(rename = "BscTestnet")]
    BscTestnet,

    Celo,

    #[serde(rename = "CeloAlfajores")]
    CeloAlfajores,

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

    Optimism,

    #[serde(rename = "OptimismGoerli")]
    OptimismGoerli,

    #[serde(rename = "OptimismSepolia")]
    OptimismSepolia,

    Polygon,

    #[serde(rename = "PolygonAmoy")]
    PolygonAmoy,

    #[serde(rename = "PolygonMumbai")]
    PolygonMumbai,

    Race,

    #[serde(rename = "RaceSepolia")]
    RaceSepolia,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadContractResponse {
    pub data: String,

    pub kind: ReadContractBodyKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadContractRequest {
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub contract: String,

    pub data: String,

    pub kind: ReadContractBodyKind,

    pub network: ReadContractBodyNetwork,
}
