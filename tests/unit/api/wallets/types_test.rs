/// @dfns-sdk-rs/tests/unit/api/wallets/types_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::api::wallets::types::{
        AssetKind, BroadcastTransactionBody, BroadcastTransactionBodyKind, BroadcastTransactionResponse,
        BroadcastTransactionResponseNetwork, BroadcastTransactionResponseStatus, CreateWalletBody,
        CreateWalletBodyNetwork, CreateWalletResponse, CreateWalletResponseStatus, Curve, Direction,
        Format, GenerateSignatureBody, GenerateSignatureBodyKind, GenerateSignatureResponse,
        GenerateSignatureResponseStatus, GetWalletAssetsResponse, GetWalletHistoryQueryKind,
        GetWalletHistoryResponseItem, GetWalletNftsResponse, ImportWalletBody, ListSignaturesResponse,
        ListTransactionsResponse, ListTransfersResponse, ListWalletsResponse, Nonce, NftKind,
        Priority, Protocol, Scheme, TransferAssetBody, TransferAssetBodyKind,
    };
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_network_serialization() {
        let networks = vec![
            (CreateWalletBodyNetwork::Ethereum, "Ethereum"),
            (CreateWalletBodyNetwork::Bitcoin, "Bitcoin"),
            (CreateWalletBodyNetwork::EthereumGoerli, "EthereumGoerli"),
            (CreateWalletBodyNetwork::Polygon, "Polygon"),
        ];

        for (network, expected) in networks {
            let serialized = serde_json::to_string(&network).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: CreateWalletBodyNetwork = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, network);
        }
    }

    #[test]
    fn test_curve_and_scheme() {
        let curves = vec![
            (Curve::Ed25519, "ed25519"),
            (Curve::Secp256K1, "secp256k1"),
            (Curve::Stark, "stark"),
        ];

        for (curve, expected) in curves {
            let serialized = serde_json::to_string(&curve).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: Curve = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, curve);
        }

        let schemes = vec![
            (Scheme::Ecdsa, "ECDSA"),
            (Scheme::EdDsa, "EdDSA"),
            (Scheme::Schnorr, "Schnorr"),
        ];

        for (scheme, expected) in schemes {
            let serialized = serde_json::to_string(&scheme).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: Scheme = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, scheme);
        }
    }

    #[test]
    fn test_create_wallet_body() {
        let body = CreateWalletBody {
            delay_delegation: Some(false),
            delegate_to: Some("user-123".to_string()),
            external_id: Some("ext-123".to_string()),
            name: Some("Test Wallet".to_string()),
            network: CreateWalletBodyNetwork::Ethereum,
            signing_key: Some(dfns_sdk_rs::api::wallets::types::CreateWalletBodySigningKey {
                curve: Some(Curve::Secp256K1),
                scheme: Some(Scheme::Ecdsa),
            }),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_create_wallet_response() {
        let response = CreateWalletResponse {
            address: Some("0x123...".to_string()),
            custodial: true,
            date_created: "2024-01-01T00:00:00Z".to_string(),
            date_exported: None,
            exported: Some(false),
            external_id: Some("ext-123".to_string()),
            id: "wallet-123".to_string(),
            imported: Some(false),
            name: Some("Test Wallet".to_string()),
            network: CreateWalletBodyNetwork::Ethereum,
            signing_key: dfns_sdk_rs::api::wallets::types::CreateWalletResponseSigningKey {
                curve: Curve::Secp256K1,
                public_key: "pubkey123".to_string(),
                scheme: Scheme::Ecdsa,
            },
            status: CreateWalletResponseStatus::Active,
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: CreateWalletResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_nonce_variants() {
        let nonce_double = Nonce::Double(123.45);
        let nonce_string = Nonce::String("123".to_string());

        let serialized_double = serde_json::to_string(&nonce_double).unwrap();
        let serialized_string = serde_json::to_string(&nonce_string).unwrap();

        assert_eq!(serialized_double, "123.45");
        assert_eq!(serialized_string, "\"123\"");

        let deserialized_double: Nonce = serde_json::from_str(&serialized_double).unwrap();
        let deserialized_string: Nonce = serde_json::from_str(&serialized_string).unwrap();

        assert_eq!(deserialized_double, nonce_double);
        assert_eq!(deserialized_string, nonce_string);
    }

    #[test]
    fn test_transaction_kinds() {
        let kinds = vec![
            (BroadcastTransactionBodyKind::Eip1559, "Eip1559"),
            (BroadcastTransactionBodyKind::Evm, "Evm"),
            (BroadcastTransactionBodyKind::EvmLegacy, "EvmLegacy"),
            (BroadcastTransactionBodyKind::Psbt, "Psbt"),
        ];

        for (kind, expected) in kinds {
            let serialized = serde_json::to_string(&kind).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: BroadcastTransactionBodyKind = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, kind);
        }
    }

    #[test]
    fn test_transfer_asset_body() {
        let body = TransferAssetBody {
            amount: Some("1.5".to_string()),
            create_destination_account: Some(false),
            external_id: Some("ext-123".to_string()),
            kind: TransferAssetBodyKind::Native,
            memo: Some("Test transfer".to_string()),
            priority: Some(Priority::Fast),
            to: "0x456...".to_string(),
            asset_id: Some("asset-123".to_string()),
            metadata: Some("metadata".to_string()),
            contract: Some("0x789...".to_string()),
            token_id: Some("token-123".to_string()),
            asset_code: Some("ETH".to_string()),
            issuer: Some("issuer-123".to_string()),
            mint: Some("mint-123".to_string()),
            master: Some("master-123".to_string()),
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: TransferAssetBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_wallet_history_response() {
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), Some(json!("value1")));

        let item = GetWalletHistoryResponseItem {
            balance_id: Some("balance-123".to_string()),
            block_number: 12345.0,
            decimals: Some(18.0),
            direction: Direction::In,
            fee: Some("0.001".to_string()),
            from: Some("0x123...".to_string()),
            index: Some("0".to_string()),
            kind: GetWalletHistoryQueryKind::NativeTransfer,
            liquidity_pool: None,
            memo: Some("Test transfer".to_string()),
            metadata: dfns_sdk_rs::api::wallets::types::PurpleMetadata {
                asset: dfns_sdk_rs::api::wallets::types::FluffyAsset {
                    decimals: Some(18.0),
                    quotes: Some(HashMap::new()),
                    symbol: Some("ETH".to_string()),
                    verified: Some(true),
                },
                fee: Some(dfns_sdk_rs::api::wallets::types::Fee {
                    decimals: Some(18.0),
                    quotes: Some(HashMap::new()),
                    symbol: Some("ETH".to_string()),
                    verified: Some(true),
                }),
            },
            network: CreateWalletBodyNetwork::Ethereum,
            symbol: Some("ETH".to_string()),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            to: Some("0x456...".to_string()),
            tx_hash: "0xabc...".to_string(),
            value: Some("1.5".to_string()),
            verified: Some(true),
            wallet_id: "wallet-123".to_string(),
            metadata_address: None,
            asset_id: None,
            clawback: None,
            opt_in: None,
            opt_out: None,
            contract: None,
            token_id: None,
            asset_code: None,
            issuer: None,
            mint: None,
            master: None,
            froms: None,
            tos: None,
        };

        let serialized = serde_json::to_string(&item).unwrap();
        let deserialized: GetWalletHistoryResponseItem = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, item);
    }

    #[test]
    fn test_asset_kinds() {
        let kinds = vec![
            (AssetKind::Native, "Native"),
            (AssetKind::Erc20, "Erc20"),
            (AssetKind::Spl, "Spl"),
            (AssetKind::Trc20, "Trc20"),
        ];

        for (kind, expected) in kinds {
            let serialized = serde_json::to_string(&kind).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: AssetKind = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, kind);
        }
    }

    #[test]
    fn test_signature_formats() {
        let formats = vec![
            (Format::Full, "Full"),
            (Format::Simple, "Simple"),
        ];

        for (format, expected) in formats {
            let serialized = serde_json::to_string(&format).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: Format = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, format);
        }
    }

    #[test]
    fn test_protocols() {
        let protocols = vec![
            (Protocol::Cggmp21, "CGGMP21"),
            (Protocol::Frost, "FROST"),
            (Protocol::FrostBitcoin, "FROST_BITCOIN"),
            (Protocol::Ku23, "KU23"),
        ];

        for (protocol, expected) in protocols {
            let serialized = serde_json::to_string(&protocol).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: Protocol = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, protocol);
        }
    }

    #[test]
    fn test_broadcast_transaction_body() {
        let body = BroadcastTransactionBody {
            external_id: Some("ext-123".to_string()),
            kind: BroadcastTransactionBodyKind::Evm,
            transaction: Some("0xraw...".to_string()),
            data: Some("0xdata...".to_string()),
            gas_limit: Some("21000".to_string()),
            nonce: Some(Nonce::String("1".to_string())),
            to: Some("0x123...".to_string()),
            value: Some("1000000000000000000".to_string()),
            max_fee_per_gas: Some("50000000000".to_string()),
            max_priority_fee_per_gas: Some("1500000000".to_string()),
            gas_price: Some("50000000000".to_string()),
            psbt: None,
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: BroadcastTransactionBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_broadcast_transaction_response() {
        let response = BroadcastTransactionResponse {
            approval_id: Some("approval-123".to_string()),
            date_broadcasted: Some("2024-01-01T00:00:00Z".to_string()),
            date_confirmed: Some("2024-01-01T00:01:00Z".to_string()),
            date_policy_resolved: Some("2024-01-01T00:00:30Z".to_string()),
            date_requested: "2024-01-01T00:00:00Z".to_string(),
            external_id: Some("ext-123".to_string()),
            fee: Some("0.001".to_string()),
            id: "tx-123".to_string(),
            network: BroadcastTransactionResponseNetwork::Ethereum,
            reason: None,
            request_body: dfns_sdk_rs::api::wallets::types::BroadcastTransactionResponseRequestBody {
                external_id: Some("ext-123".to_string()),
                kind: BroadcastTransactionBodyKind::Evm,
                transaction: Some("0xraw...".to_string()),
                data: Some("0xdata...".to_string()),
                gas_limit: Some("21000".to_string()),
                nonce: Some(Nonce::String("1".to_string())),
                to: Some("0x123...".to_string()),
                value: Some("1000000000000000000".to_string()),
                max_fee_per_gas: Some("50000000000".to_string()),
                max_priority_fee_per_gas: Some("1500000000".to_string()),
                gas_price: Some("50000000000".to_string()),
                psbt: None,
            },
            requester: dfns_sdk_rs::api::wallets::types::BroadcastTransactionResponseRequester {
                app_id: Some("app-123".to_string()),
                token_id: Some("token-123".to_string()),
                user_id: "user-123".to_string(),
            },
            status: BroadcastTransactionResponseStatus::Confirmed,
            tx_hash: Some("0xhash...".to_string()),
            wallet_id: "wallet-123".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: BroadcastTransactionResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_generate_signature_body() {
        let mut types = HashMap::new();
        types.insert(
            "Person".to_string(),
            vec![dfns_sdk_rs::api::wallets::types::GenerateSignatureBodyType {
                name: "name".to_string(),
                type_type: "string".to_string(),
            }],
        );

        let body = GenerateSignatureBody {
            external_id: Some("ext-123".to_string()),
            kind: GenerateSignatureBodyKind::Eip712,
            sign_doc: None,
            hash: None,
            taproot_merkle_root: None,
            message: Some(dfns_sdk_rs::api::wallets::types::Message::String("Hello".to_string())),
            transaction: None,
            domain: Some(dfns_sdk_rs::api::wallets::types::GenerateSignatureBodyDomain {
                chain_id: Some(Nonce::Double(1.0)),
                name: Some("Test Domain".to_string()),
                salt: Some("0x123".to_string()),
                verifying_contract: Some("0x456".to_string()),
                version: Some("1".to_string()),
            }),
            types: Some(types),
            psbt: None,
            format: Some(Format::Full),
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: GenerateSignatureBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_get_wallet_assets_response() {
        let mut quotes = HashMap::new();
        quotes.insert("USD".to_string(), 1800.0);

        let response = GetWalletAssetsResponse {
            assets: vec![dfns_sdk_rs::api::wallets::types::AssetElement {
                balance: "1.5".to_string(),
                decimals: 18.0,
                kind: AssetKind::Native,
                quotes: Some(quotes.clone()),
                symbol: Some("ETH".to_string()),
                verified: Some(true),
                metadata: None,
                asset_id: None,
                contract: None,
                asset_code: None,
                issuer: None,
                token_id: None,
                mint: None,
                master: None,
            }],
            network: BroadcastTransactionResponseNetwork::Ethereum,
            net_worth: Some(quotes),
            wallet_id: "wallet-123".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetWalletAssetsResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_get_wallet_nfts_response() {
        let response = GetWalletNftsResponse {
            network: BroadcastTransactionResponseNetwork::Ethereum,
            nfts: vec![dfns_sdk_rs::api::wallets::types::Nft {
                asset_id: None,
                kind: NftKind::Erc721,
                symbol: Some("BAYC".to_string()),
                token_uri: Some("ipfs://...".to_string()),
                contract: Some("0x123...".to_string()),
                token_id: Some("1234".to_string()),
            }],
            wallet_id: "wallet-123".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetWalletNftsResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_import_wallet_body() {
        let body = ImportWalletBody {
            curve: Curve::Secp256K1,
            encrypted_key_shares: vec![
                dfns_sdk_rs::api::wallets::types::ImportWalletBodyEncryptedKeyShare {
                    encrypted_key_share: "encrypted-data".to_string(),
                    signer_id: "signer-123".to_string(),
                },
            ],
            external_id: Some("ext-123".to_string()),
            min_signers: 1.0,
            name: Some("Imported Wallet".to_string()),
            network: CreateWalletBodyNetwork::Ethereum,
            protocol: Protocol::Frost,
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: ImportWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_list_responses() {
        // Test ListSignaturesResponse
        let signatures_response = ListSignaturesResponse {
            items: vec![],
            next_page_token: None,
            wallet_id: "wallet-123".to_string(),
        };
        let serialized = serde_json::to_string(&signatures_response).unwrap();
        let deserialized: ListSignaturesResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, signatures_response);

        // Test ListTransactionsResponse
        let transactions_response = ListTransactionsResponse {
            items: vec![],
            next_page_token: None,
            wallet_id: "wallet-123".to_string(),
        };
        let serialized = serde_json::to_string(&transactions_response).unwrap();
        let deserialized: ListTransactionsResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, transactions_response);

        // Test ListTransfersResponse
        let transfers_response = ListTransfersResponse {
            items: vec![],
            next_page_token: None,
            wallet_id: "wallet-123".to_string(),
        };
        let serialized = serde_json::to_string(&transfers_response).unwrap();
        let deserialized: ListTransfersResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, transfers_response);

        // Test ListWalletsResponse
        let wallets_response = ListWalletsResponse {
            items: vec![],
            next_page_token: None,
        };
        let serialized = serde_json::to_string(&wallets_response).unwrap();
        let deserialized: ListWalletsResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, wallets_response);
    }
}
