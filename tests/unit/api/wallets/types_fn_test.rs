/// @dfns-sdk-rs/tests/unit/api/wallets/types_fn_test.rs
#[path = "../../../../src/api/wallets/types.rs"]
mod parent;
use parent::{Direction, GetWalletHistoryQueryKind, NetWorth};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_worth_display() {
        assert_eq!(format!("{}", NetWorth::True), "true");
        assert_eq!(format!("{}", NetWorth::None), "*");

        assert_eq!(format!("{}", NetWorth::default()), "*");
    }

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::In), "in");
        assert_eq!(format!("{}", Direction::Out), "out");
        assert_eq!(format!("{}", Direction::None), "*");

        assert_eq!(format!("{}", Direction::default()), "*");
    }

    #[test]
    fn test_wallet_history_query_kind_display() {
        let test_cases = vec![
            (GetWalletHistoryQueryKind::Aip21Transfer, "Aip21Transfer"),
            (GetWalletHistoryQueryKind::AsaTransfer, "AsaTransfer"),
            (GetWalletHistoryQueryKind::Erc20Transfer, "Erc20Transfer"),
            (GetWalletHistoryQueryKind::Erc721Transfer, "Erc721Transfer"),
            (GetWalletHistoryQueryKind::NativeTransfer, "NativeTransfer"),
            (GetWalletHistoryQueryKind::Sep41Transfer, "Sep41Transfer"),
            (
                GetWalletHistoryQueryKind::Spl2022Transfer,
                "Spl2022Transfer",
            ),
            (GetWalletHistoryQueryKind::SplTransfer, "SplTransfer"),
            (GetWalletHistoryQueryKind::Tep74Transfer, "Tep74Transfer"),
            (GetWalletHistoryQueryKind::Trc10Transfer, "Trc10Transfer"),
            (GetWalletHistoryQueryKind::Trc20Transfer, "Trc20Transfer"),
            (GetWalletHistoryQueryKind::Trc721Transfer, "Trc721Transfer"),
            (GetWalletHistoryQueryKind::UtxoTransfer, "UtxoTransfer"),
            (GetWalletHistoryQueryKind::None, "*"),
        ];

        for (kind, expected) in test_cases {
            assert_eq!(format!("{}", kind), expected);
        }

        assert_eq!(format!("{}", GetWalletHistoryQueryKind::default()), "*");
    }
}
