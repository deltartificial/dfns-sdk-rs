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
        Priority, Protocol, Scheme, TransferAssetBody, TransferAssetBodyKind, Message, PurpleSignature,
        TagWalletBody, TagWalletResponse, UntagWalletBody, UpdateWalletBody, UpdateWalletResponse,
        GetTransactionResponse, GetTransferResponse, GetWalletResponse, GetSignatureResponse,
        DelegateWalletResponse, DelegateWalletResponseStatus, GetWalletHistoryQuery,
        ImportWalletBodyEncryptedKeyShare, Nft, ListWalletsResponseItem, ItemSigningKey,
        TransferAssetResponse, TentacledRequestBody, TentacledRequester,
        BroadcastTransactionResponseRequestBody, BroadcastTransactionResponseRequester,
        TransferAssetResponseMetadata, StickyAsset, ListWalletsQuery, UpdateWalletResponseSigningKey,
        ListTransfersResponse, ListTransfersResponseItem,
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

    #[test]
    fn test_generate_signature_response() {
        let response = GenerateSignatureResponse {
            approval_id: Some("approval-123".to_string()),
            date_confirmed: Some("2024-01-01T00:01:00Z".to_string()),
            date_policy_resolved: Some("2024-01-01T00:00:30Z".to_string()),
            date_requested: "2024-01-01T00:00:00Z".to_string(),
            date_signed: Some("2024-01-01T00:00:45Z".to_string()),
            external_id: Some("ext-123".to_string()),
            fee: Some("0.001".to_string()),
            id: "sig-123".to_string(),
            network: CreateWalletBodyNetwork::Ethereum,
            reason: None,
            request_body: dfns_sdk_rs::api::wallets::types::GenerateSignatureResponseRequestBody {
                external_id: Some("ext-123".to_string()),
                kind: GenerateSignatureBodyKind::Eip712,
                sign_doc: None,
                hash: None,
                taproot_merkle_root: None,
                message: Some(Message::String("Hello".to_string())),
                transaction: None,
                domain: None,
                types: None,
                psbt: None,
                format: Some(Format::Full),
            },
            requester: dfns_sdk_rs::api::wallets::types::GenerateSignatureResponseRequester {
                app_id: Some("app-123".to_string()),
                token_id: Some("token-123".to_string()),
                user_id: "user-123".to_string(),
            },
            signature: Some(PurpleSignature {
                encoded: Some("0xsignature...".to_string()),
                r: "r-value".to_string(),
                recid: Some(0.0),
                s: "s-value".to_string(),
            }),
            signatures: None,
            signed_data: Some("signed-data".to_string()),
            status: GenerateSignatureResponseStatus::Signed,
            tx_hash: None,
            wallet_id: "wallet-123".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GenerateSignatureResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_tag_operations() {
        // Test TagWalletBody
        let tag_body = TagWalletBody {
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };
        let serialized = serde_json::to_string(&tag_body).unwrap();
        let deserialized: TagWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, tag_body);

        // Test UntagWalletBody
        let untag_body = UntagWalletBody {
            tags: vec!["tag1".to_string()],
        };
        let serialized = serde_json::to_string(&untag_body).unwrap();
        let deserialized: UntagWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, untag_body);

        // Test TagWalletResponse
        let mut tag_response = HashMap::new();
        tag_response.insert("tag1".to_string(), Some(json!("value1")));
        let serialized = serde_json::to_string(&tag_response).unwrap();
        let deserialized: TagWalletResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, tag_response);
    }

    #[test]
    fn test_update_wallet() {
        let body = UpdateWalletBody {
            external_id: Some("new-ext-123".to_string()),
            name: Some("Updated Wallet Name".to_string()),
        };
        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: UpdateWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);

        let response = UpdateWalletResponse {
            address: Some("0x123...".to_string()),
            custodial: true,
            date_created: "2024-01-01T00:00:00Z".to_string(),
            date_exported: None,
            exported: Some(false),
            external_id: Some("new-ext-123".to_string()),
            id: "wallet-123".to_string(),
            imported: Some(false),
            name: Some("Updated Wallet Name".to_string()),
            network: CreateWalletBodyNetwork::Ethereum,
            signing_key: dfns_sdk_rs::api::wallets::types::UpdateWalletResponseSigningKey {
                curve: Curve::Secp256K1,
                public_key: "pubkey123".to_string(),
                scheme: Scheme::Ecdsa,
            },
            status: CreateWalletResponseStatus::Active,
            tags: vec!["tag1".to_string()],
        };
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: UpdateWalletResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_get_responses() {
        // Test GetTransactionResponse
        let tx_response = GetTransactionResponse {
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
            request_body: dfns_sdk_rs::api::wallets::types::GetTransactionResponseRequestBody {
                external_id: Some("ext-123".to_string()),
                kind: BroadcastTransactionBodyKind::Evm,
                transaction: Some("0xraw...".to_string()),
                data: Some("0xdata...".to_string()),
                gas_limit: Some("21000".to_string()),
                nonce: Some(Nonce::String("1".to_string())),
                to: Some("0x123...".to_string()),
                value: Some("1.0".to_string()),
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                gas_price: Some("50000000000".to_string()),
                psbt: None,
            },
            requester: dfns_sdk_rs::api::wallets::types::GetTransactionResponseRequester {
                app_id: Some("app-123".to_string()),
                token_id: Some("token-123".to_string()),
                user_id: "user-123".to_string(),
            },
            status: BroadcastTransactionResponseStatus::Confirmed,
            tx_hash: Some("0xhash...".to_string()),
            wallet_id: "wallet-123".to_string(),
        };
        let serialized = serde_json::to_string(&tx_response).unwrap();
        let deserialized: GetTransactionResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, tx_response);

        // Test GetWalletResponse
        let wallet_response = GetWalletResponse {
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
            signing_key: dfns_sdk_rs::api::wallets::types::GetWalletResponseSigningKey {
                curve: Curve::Secp256K1,
                public_key: "pubkey123".to_string(),
                scheme: Scheme::Ecdsa,
            },
            status: CreateWalletResponseStatus::Active,
            tags: vec!["tag1".to_string()],
        };
        let serialized = serde_json::to_string(&wallet_response).unwrap();
        let deserialized: GetWalletResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, wallet_response);
    }

    #[test]
    fn test_delegate_wallet_response() {
        let response = DelegateWalletResponse {
            status: DelegateWalletResponseStatus::Delegated,
            wallet_id: "wallet-123".to_string(),
        };
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: DelegateWalletResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_wallet_history_query() {
        let query = GetWalletHistoryQuery {
            contract: Some("0x123...".to_string()),
            direction: Some(Direction::In),
            kind: Some(GetWalletHistoryQueryKind::NativeTransfer),
            limit: Some("10".to_string()),
            pagination_token: Some("token123".to_string()),
        };
        let serialized = serde_json::to_string(&query).unwrap();
        let deserialized: GetWalletHistoryQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, query);

        // Test Direction Display implementation
        assert_eq!(Direction::In.to_string(), "in");
        assert_eq!(Direction::Out.to_string(), "out");

        // Test GetWalletHistoryQueryKind Display implementation
        assert_eq!(
            GetWalletHistoryQueryKind::NativeTransfer.to_string(),
            "NativeTransfer"
        );
        assert_eq!(
            GetWalletHistoryQueryKind::Erc20Transfer.to_string(),
            "Erc20Transfer"
        );
    }

    #[test]
    fn test_signature_response_statuses() {
        let statuses = vec![
            (GenerateSignatureResponseStatus::Confirmed, "Confirmed"),
            (GenerateSignatureResponseStatus::Executing, "Executing"),
            (GenerateSignatureResponseStatus::Failed, "Failed"),
            (GenerateSignatureResponseStatus::Pending, "Pending"),
            (GenerateSignatureResponseStatus::Rejected, "Rejected"),
            (GenerateSignatureResponseStatus::Signed, "Signed"),
        ];

        for (status, expected) in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: GenerateSignatureResponseStatus =
                serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, status);
        }
    }

    #[test]
    fn test_transaction_response_statuses() {
        let statuses = vec![
            (BroadcastTransactionResponseStatus::Broadcasted, "Broadcasted"),
            (BroadcastTransactionResponseStatus::Confirmed, "Confirmed"),
            (BroadcastTransactionResponseStatus::Executing, "Executing"),
            (BroadcastTransactionResponseStatus::Failed, "Failed"),
            (BroadcastTransactionResponseStatus::Pending, "Pending"),
            (BroadcastTransactionResponseStatus::Rejected, "Rejected"),
        ];

        for (status, expected) in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: BroadcastTransactionResponseStatus =
                serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, status);
        }
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
            min_signers: 2.0,
            name: Some("Imported Wallet".to_string()),
            network: CreateWalletBodyNetwork::Ethereum,
            protocol: Protocol::Frost,
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: ImportWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_nft_kinds() {
        let kinds = vec![
            (NftKind::Erc721, "Erc721"),
            (NftKind::Asa, "Asa"),
            (NftKind::Trc721, "Trc721"),
        ];

        for (kind, expected) in kinds {
            let serialized = serde_json::to_string(&kind).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: NftKind = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, kind);
        }
    }

    #[test]
    fn test_get_wallet_nfts_response() {
        let response = GetWalletNftsResponse {
            network: BroadcastTransactionResponseNetwork::Ethereum,
            nfts: vec![Nft {
                asset_id: Some("asset-123".to_string()),
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
    fn test_list_wallets_response() {
        let response = ListWalletsResponse {
            items: vec![ListWalletsResponseItem {
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
                signing_key: ItemSigningKey {
                    curve: Curve::Secp256K1,
                    public_key: "pubkey123".to_string(),
                    scheme: Scheme::Ecdsa,
                },
                status: CreateWalletResponseStatus::Active,
                tags: vec!["tag1".to_string()],
            }],
            next_page_token: Some("next-page".to_string()),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ListWalletsResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_transfer_asset_response() {
        let response = TransferAssetResponse {
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
            request_body: TentacledRequestBody {
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
            },
            requester: TentacledRequester {
                app_id: Some("app-123".to_string()),
                token_id: Some("token-123".to_string()),
                user_id: "user-123".to_string(),
            },
            status: BroadcastTransactionResponseStatus::Confirmed,
            tx_hash: Some("0xhash...".to_string()),
            wallet_id: "wallet-123".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: TransferAssetResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_list_transactions_response() {
        let response = ListTransactionsResponse {
            items: vec![ListTransactionsResponseItem {
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
                request_body: FluffyRequestBody {
                    external_id: Some("ext-123".to_string()),
                    kind: BroadcastTransactionBodyKind::Evm,
                    transaction: Some("0xraw...".to_string()),
                    data: Some("0xdata...".to_string()),
                    gas_limit: Some("21000".to_string()),
                    nonce: Some(Nonce::String("1".to_string())),
                    to: Some("0x123...".to_string()),
                    value: Some("1.0".to_string()),
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    gas_price: Some("50000000000".to_string()),
                    psbt: None,
                },
                requester: FluffyRequester {
                    app_id: Some("app-123".to_string()),
                    token_id: Some("token-123".to_string()),
                    user_id: "user-123".to_string(),
                },
                status: BroadcastTransactionResponseStatus::Confirmed,
                tx_hash: Some("0xhash...".to_string()),
                wallet_id: "wallet-123".to_string(),
            }],
            next_page_token: Some("next-page".to_string()),
            wallet_id: "wallet-123".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ListTransactionsResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
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
            request_body: BroadcastTransactionResponseRequestBody {
                external_id: Some("ext-123".to_string()),
                kind: BroadcastTransactionBodyKind::Evm,
                transaction: Some("0xraw...".to_string()),
                data: Some("0xdata...".to_string()),
                gas_limit: Some("21000".to_string()),
                nonce: Some(Nonce::String("1".to_string())),
                to: Some("0x123...".to_string()),
                value: Some("1.0".to_string()),
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                gas_price: Some("50000000000".to_string()),
                psbt: None,
            },
            requester: BroadcastTransactionResponseRequester {
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
    fn test_broadcast_transaction_body_kind() {
        let kinds = vec![
            (BroadcastTransactionBodyKind::Eip1559, "Eip1559"),
            (BroadcastTransactionBodyKind::Evm, "Evm"),
            (BroadcastTransactionBodyKind::EvmLegacy, "EvmLegacy"),
            (BroadcastTransactionBodyKind::Psbt, "Psbt"),
            (BroadcastTransactionBodyKind::Transaction, "Transaction"),
        ];

        for (kind, expected) in kinds {
            let serialized = serde_json::to_string(&kind).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: BroadcastTransactionBodyKind = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, kind);
        }
    }

    #[test]
    fn test_nonce_enum() {
        let double_nonce = Nonce::Double(1.0);
        let string_nonce = Nonce::String("1".to_string());

        let serialized_double = serde_json::to_string(&double_nonce).unwrap();
        let serialized_string = serde_json::to_string(&string_nonce).unwrap();

        assert_eq!(serialized_double, "1.0");
        assert_eq!(serialized_string, "\"1\"");

        let deserialized_double: Nonce = serde_json::from_str(&serialized_double).unwrap();
        let deserialized_string: Nonce = serde_json::from_str(&serialized_string).unwrap();

        assert_eq!(deserialized_double, double_nonce);
        assert_eq!(deserialized_string, string_nonce);
    }

    #[test]
    fn test_create_wallet_body() {
        let body = CreateWalletBody {
            delay_delegation: Some(true),
            delegate_to: Some("delegate-123".to_string()),
            external_id: Some("ext-123".to_string()),
            name: Some("Test Wallet".to_string()),
            network: CreateWalletBodyNetwork::Ethereum,
            signing_key: Some(CreateWalletBodySigningKey {
                curve: Curve::Secp256K1,
                public_key: "pubkey123".to_string(),
                scheme: Scheme::Ecdsa,
            }),
            tags: Some(vec!["tag1".to_string()]),
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_create_wallet_body_network() {
        let networks = vec![
            (CreateWalletBodyNetwork::Algorand, "Algorand"),
            (CreateWalletBodyNetwork::Aptos, "Aptos"),
            (CreateWalletBodyNetwork::Ethereum, "Ethereum"),
            (CreateWalletBodyNetwork::Solana, "Solana"),
        ];

        for (network, expected) in networks {
            let serialized = serde_json::to_string(&network).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: CreateWalletBodyNetwork = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, network);
        }
    }

    #[test]
    fn test_transfer_asset_response_metadata() {
        let metadata = TransferAssetResponseMetadata {
            asset: StickyAsset {
                decimals: Some(18.0),
                quotes: Some(HashMap::from([("USD".to_string(), 2000.0)])),
                symbol: Some("ETH".to_string()),
                verified: Some(true),
            },
        };

        let serialized = serde_json::to_string(&metadata).unwrap();
        let deserialized: TransferAssetResponseMetadata = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, metadata);
    }

    #[test]
    fn test_list_wallets_query() {
        let query = ListWalletsQuery {
            limit: Some("10".to_string()),
            owner_id: Some("owner-123".to_string()),
            owner_username: Some("user123".to_string()),
            pagination_token: Some("token123".to_string()),
        };

        let serialized = serde_json::to_string(&query).unwrap();
        let deserialized: ListWalletsQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, query);
    }

    #[test]
    fn test_update_wallet_response() {
        let response = UpdateWalletResponse {
            address: Some("0x123...".to_string()),
            custodial: true,
            date_created: "2024-01-01T00:00:00Z".to_string(),
            date_exported: None,
            exported: Some(false),
            external_id: Some("new-ext-123".to_string()),
            id: "wallet-123".to_string(),
            imported: Some(false),
            name: Some("Updated Wallet Name".to_string()),
            network: CreateWalletBodyNetwork::Ethereum,
            signing_key: UpdateWalletResponseSigningKey {
                curve: Curve::Secp256K1,
                public_key: "pubkey123".to_string(),
                scheme: Scheme::Ecdsa,
            },
            status: CreateWalletResponseStatus::Active,
            tags: vec!["tag1".to_string()],
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: UpdateWalletResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_tag_wallet_body() {
        let body = TagWalletBody {
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: TagWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_untag_wallet_body() {
        let body = UntagWalletBody {
            tags: vec!["tag1".to_string()],
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: UntagWalletBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }
}
