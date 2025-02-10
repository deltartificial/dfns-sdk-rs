// @dfns-sdk-rs/src/api/networks/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Network {
    Bitcoin,
    BitcoinSignet,
    BitcoinTestnet3,
    ArbitrumOne,
    ArbitrumGoerli,
    ArbitrumSepolia,
    AvalancheC,
    AvalancheCFuji,
    Base,
    BaseGoerli,
    BaseSepolia,
    Bsc,
    BscTestnet,
    Berachain,
    BerachainBArtio,
    Celo,
    CeloAlfajores,
    Ethereum,
    EthereumGoerli,
    EthereumSepolia,
    EthereumHolesky,
    FantomOpera,
    FantomTestnet,
    Optimism,
    OptimismGoerli,
    OptimismSepolia,
    Polygon,
    PolygonAmoy,
    PolygonMumbai,
    Race,
    RaceSepolia,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeRate {
    pub fee_rate: String,
    pub block_horizon: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Eip1559Fee {
    pub max_priority_fee_per_gas: String,
    pub max_fee_per_gas: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum GetFeesResponse {
    Bitcoin {
        network: Network,
        block_number: u64,
        slow: FeeRate,
        standard: FeeRate,
        fast: FeeRate,
    },
    Eip1559 {
        network: Network,
        block_number: u64,
        slow: Eip1559Fee,
        standard: Eip1559Fee,
        fast: Eip1559Fee,
        estimated_base_fee: u64,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFeesRequest {
    pub network: Network,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ReadContractRequest {
    Evm {
        network: Network,
        contract: String,
        data: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ReadContractResponse {
    Evm {
        data: String,
    },
}