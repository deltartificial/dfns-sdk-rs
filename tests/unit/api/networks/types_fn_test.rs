/// @dfns-sdk-rs/tests/unit/api/networks/types_fn_test.rs
#[path = "../../../../src/api/networks/types.rs"]
mod parent;
use parent::GetFeesQueryNetwork;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fees_query_network_display() {
        let test_cases = vec![
            (GetFeesQueryNetwork::ArbitrumGoerli, "ArbitrumGoerli"),
            (GetFeesQueryNetwork::ArbitrumOne, "ArbitrumOne"),
            (GetFeesQueryNetwork::ArbitrumSepolia, "ArbitrumSepolia"),
            (GetFeesQueryNetwork::AvalancheC, "AvalancheC"),
            (GetFeesQueryNetwork::AvalancheCFuji, "AvalancheCFuji"),
            (GetFeesQueryNetwork::Base, "Base"),
            (GetFeesQueryNetwork::BaseGoerli, "BaseGoerli"),
            (GetFeesQueryNetwork::BaseSepolia, "BaseSepolia"),
            (GetFeesQueryNetwork::Berachain, "Berachain"),
            (GetFeesQueryNetwork::BerachainBArtio, "BerachainBArtio"),
            (GetFeesQueryNetwork::Bitcoin, "Bitcoin"),
            (GetFeesQueryNetwork::BitcoinSignet, "BitcoinSignet"),
            (GetFeesQueryNetwork::BitcoinTestnet3, "BitcoinTestnet3"),
            (GetFeesQueryNetwork::Bsc, "Bsc"),
            (GetFeesQueryNetwork::BscTestnet, "BscTestnet"),
            (GetFeesQueryNetwork::Celo, "Celo"),
            (GetFeesQueryNetwork::CeloAlfajores, "CeloAlfajores"),
            (GetFeesQueryNetwork::Ethereum, "Ethereum"),
            (GetFeesQueryNetwork::EthereumGoerli, "EthereumGoerli"),
            (GetFeesQueryNetwork::EthereumHolesky, "EthereumHolesky"),
            (GetFeesQueryNetwork::EthereumSepolia, "EthereumSepolia"),
            (GetFeesQueryNetwork::FantomOpera, "FantomOpera"),
            (GetFeesQueryNetwork::FantomTestnet, "FantomTestnet"),
            (GetFeesQueryNetwork::Optimism, "Optimism"),
            (GetFeesQueryNetwork::OptimismGoerli, "OptimismGoerli"),
            (GetFeesQueryNetwork::OptimismSepolia, "OptimismSepolia"),
            (GetFeesQueryNetwork::Polygon, "Polygon"),
            (GetFeesQueryNetwork::PolygonAmoy, "PolygonAmoy"),
            (GetFeesQueryNetwork::PolygonMumbai, "PolygonMumbai"),
            (GetFeesQueryNetwork::Race, "Race"),
            (GetFeesQueryNetwork::RaceSepolia, "RaceSepolia"),
            (GetFeesQueryNetwork::None, "*"),
        ];

        for (network, expected) in test_cases {
            assert_eq!(format!("{}", network), expected);
        }

        assert_eq!(format!("{}", GetFeesQueryNetwork::default()), "*");
    }
}
