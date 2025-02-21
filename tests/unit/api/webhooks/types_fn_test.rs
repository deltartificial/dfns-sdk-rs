/// @dfns-sdk-rs/tests/unit/api/webhooks/types_fn_test.rs
#[path = "../../../../src/api/webhooks/types.rs"]
mod parent;
use parent::{DeliveryFailed, Kind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_failed_display() {
        assert_eq!(format!("{}", DeliveryFailed::False), "false");
        assert_eq!(format!("{}", DeliveryFailed::True), "true");

        assert_eq!(format!("{}", DeliveryFailed::default()), "false");
    }

    #[test]
    fn test_kind_display() {
        let test_cases = vec![
            (Kind::PolicyApprovalPending, "policy.approval.pending"),
            (Kind::PolicyApprovalResolved, "policy.approval.resolved"),
            (Kind::PolicyTriggered, "policy.triggered"),
            (
                Kind::WalletBlockchaineventDetected,
                "wallet.blockchainevent.detected",
            ),
            (Kind::WalletCreated, "wallet.created"),
            (Kind::WalletDelegated, "wallet.delegated"),
            (Kind::WalletExported, "wallet.exported"),
            (Kind::WalletSignatureFailed, "wallet.signature.failed"),
            (Kind::WalletSignatureRejected, "wallet.signature.rejected"),
            (Kind::WalletSignatureRequested, "wallet.signature.requested"),
            (Kind::WalletSignatureSigned, "wallet.signature.signed"),
            (Kind::WalletTagsModified, "wallet.tags.modified"),
            (
                Kind::WalletTransactionBroadcasted,
                "wallet.transaction.broadcasted",
            ),
            (
                Kind::WalletTransactionConfirmed,
                "wallet.transaction.confirmed",
            ),
            (Kind::WalletTransactionFailed, "wallet.transaction.failed"),
            (
                Kind::WalletTransactionRejected,
                "wallet.transaction.rejected",
            ),
            (
                Kind::WalletTransactionRequested,
                "wallet.transaction.requested",
            ),
            (
                Kind::WalletTransferBroadcasted,
                "wallet.transfer.broadcasted",
            ),
            (Kind::WalletTransferConfirmed, "wallet.transfer.confirmed"),
            (Kind::WalletTransferFailed, "wallet.transfer.failed"),
            (Kind::WalletTransferRejected, "wallet.transfer.rejected"),
            (Kind::WalletTransferRequested, "wallet.transfer.requested"),
            (Kind::None, "*"),
        ];

        for (kind, expected) in test_cases {
            assert_eq!(format!("{}", kind), expected);
        }

        assert_eq!(format!("{}", Kind::default()), "*");
    }
}
