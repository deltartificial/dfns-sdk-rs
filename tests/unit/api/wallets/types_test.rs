
/// @dfns-sdk-rs/tests/unit/api/wallets/types_test.rs

#[path = "../../../src/api/wallets/types.rs"]
mod parent;
use parent::{
    AssetElement, AssetKind, BodyEncryptedKeyShare, BodySigningKey, BodySupportedScheme,
    BroadcastTransactionBody, BroadcastTransactionBodyKind, BroadcastTransactionParams,
    BroadcastTransactionRequest, BroadcastTransactionResponse, BroadcastTransactionResponseNetwork,
    BroadcastTransactionResponseRequestBody, BroadcastTransactionResponseRequester,
    BroadcastTransactionResponseStatus, CreateWalletBody, CreateWalletBodyNetwork,
    CreateWalletBodySigningKey, CreateWalletRequest, CreateWalletRequestBody, CreateWalletResponse,
    CreateWalletResponseSigningKey, CreateWalletResponseStatus, Curve, DelegateWalletBody,
    DelegateWalletParams, DelegateWalletRequest, DelegateWalletRequestBody, DelegateWalletResponse,
    DelegateWalletResponseStatus, Direction, ExportWalletBody, ExportWalletBodySupportedScheme,
    ExportWalletParams, ExportWalletRequest, ExportWalletRequestBody, ExportWalletResponse,
    ExportWalletResponseEncryptedKeyShare, Fee, FluffyAsset, FluffyDomain, FluffyMetadata,
    FluffyRequestBody, FluffyRequester, FluffySignature, FluffyType, Format, GenerateSignatureBody,
    GenerateSignatureBodyDomain, GenerateSignatureBodyKind, GenerateSignatureBodyType,
    GenerateSignatureParams, GenerateSignatureRequest, GenerateSignatureResponse,
    GenerateSignatureResponseRequestBody, GenerateSignatureResponseRequester,
    GenerateSignatureResponseStatus, GetSignatureParams, GetSignatureRequest, GetSignatureResponse,
    GetSignatureResponseRequestBody, GetSignatureResponseRequester, GetTransactionParams,
    GetTransactionRequest, GetTransactionResponse, GetTransactionResponseRequestBody,
    GetTransactionResponseRequester, GetTransferParams, GetTransferRequest, GetTransferResponse,
    GetTransferResponseMetadata, GetTransferResponseRequestBody, GetTransferResponseRequester,
    GetWalletAssetsParams, GetWalletAssetsQuery, GetWalletAssetsRequest,
    GetWalletAssetsRequestQuery, GetWalletAssetsResponse, GetWalletHistoryParams,
    GetWalletHistoryQuery, GetWalletHistoryQueryKind, GetWalletHistoryRequest,
    GetWalletHistoryRequestQuery, GetWalletHistoryResponse, GetWalletHistoryResponseItem,
    GetWalletNftsParams, GetWalletNftsRequest, GetWalletNftsResponse, GetWalletParams,
    GetWalletRequest, GetWalletResponse, GetWalletResponseSigningKey, ImportWalletBody,
    ImportWalletBodyEncryptedKeyShare, ImportWalletRequest, ImportWalletRequestBody,
    ImportWalletResponse, ImportWalletResponseSigningKey, IndecentSignature, IndigoSignature,
    ItemSigningKey, ListSignaturesParams, ListSignaturesQuery, ListSignaturesRequest,
    ListSignaturesRequestQuery, ListSignaturesResponse, ListSignaturesResponseItem,
    ListTransactionsParams, ListTransactionsQuery, ListTransactionsRequest,
    ListTransactionsRequestQuery, ListTransactionsResponse, ListTransactionsResponseItem,
    ListTransfersParams, ListTransfersQuery, ListTransfersRequest, ListTransfersRequestQuery,
    ListTransfersResponse, ListTransfersResponseItem, ListWalletsQuery, ListWalletsRequest,
    ListWalletsRequestQuery, ListWalletsResponse, ListWalletsResponseItem, Message, NetWorth, Nft,
    NftKind, Nonce, Priority, Protocol, PurpleAsset, PurpleDomain, PurpleMetadata,
    PurpleRequestBody, PurpleRequester, PurpleSignature, PurpleType, Scheme, StickyAsset,
    StickySignature, TagWalletBody, TagWalletParams, TagWalletRequest, TagWalletRequestBody,
    TentacledAsset, TentacledDomain, TentacledRequestBody, TentacledRequester, TentacledSignature,
    TentacledType, TransferAssetBody, TransferAssetBodyKind, TransferAssetParams,
    TransferAssetRequest, TransferAssetResponse, TransferAssetResponseMetadata,
    TransferAssetResponseRequestBody, TransferAssetResponseRequester, UntagWalletBody,
    UntagWalletParams, UntagWalletRequest, UntagWalletRequestBody, UpdateWalletBody,
    UpdateWalletParams, UpdateWalletRequest, UpdateWalletRequestBody, UpdateWalletResponse,
    UpdateWalletResponseSigningKey,
};
use serde_json;
use std::mem;
#[cfg(test)]
mod test_broadcasttransactionparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionParams::default();
        let b = BroadcastTransactionParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionParams>();
        let align = std::mem::align_of::<BroadcastTransactionParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BroadcastTransactionParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionParams>>();
        let type_size = std::mem::size_of::<BroadcastTransactionParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = BroadcastTransactionParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BroadcastTransactionParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_broadcasttransactionresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionResponse::default();
        let b = BroadcastTransactionResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionResponse =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionResponse>();
        let align = std::mem::align_of::<BroadcastTransactionResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BroadcastTransactionResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionResponse>>();
        let type_size = std::mem::size_of::<BroadcastTransactionResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionResponse),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = BroadcastTransactionResponse::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_broadcasted() {
        let instance = BroadcastTransactionResponse::default();
        let _: Option<String> = instance.date_broadcasted;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = BroadcastTransactionResponse::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = BroadcastTransactionResponse::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = BroadcastTransactionResponse::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_external_id() {
        let instance = BroadcastTransactionResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = BroadcastTransactionResponse::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = BroadcastTransactionResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_network() {
        let instance = BroadcastTransactionResponse::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = BroadcastTransactionResponse::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = BroadcastTransactionResponse::default();
        let _: BroadcastTransactionResponseRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = BroadcastTransactionResponse::default();
        let _: BroadcastTransactionResponseRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = BroadcastTransactionResponse::default();
        let _: BroadcastTransactionResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = BroadcastTransactionResponse::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = BroadcastTransactionResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BroadcastTransactionResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_broadcasttransactionresponsenetwork {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionResponseNetwork::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionResponseNetwork::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionResponseNetwork::default();
        let b = BroadcastTransactionResponseNetwork::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionResponseNetwork::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionResponseNetwork =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionResponseNetwork::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionResponseNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionResponseNetwork::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionResponseNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionResponseNetwork::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionResponseNetwork>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionResponseNetwork>();
        let align = std::mem::align_of::<BroadcastTransactionResponseNetwork>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(BroadcastTransactionResponseNetwork)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionResponseNetwork::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionResponseNetwork>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionResponseNetwork>>();
        let type_size = std::mem::size_of::<BroadcastTransactionResponseNetwork>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionResponseNetwork),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionResponseNetwork),
            type_size
        );
    }
}
#[cfg(test)]
mod test_broadcasttransactionresponserequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionResponseRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionResponseRequestBody::default();
        let b = BroadcastTransactionResponseRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionResponseRequestBody =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionResponseRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionResponseRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionResponseRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionResponseRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionResponseRequestBody>();
        let align = std::mem::align_of::<BroadcastTransactionResponseRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(BroadcastTransactionResponseRequestBody)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionResponseRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionResponseRequestBody>>();
        let type_size = std::mem::size_of::<BroadcastTransactionResponseRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionResponseRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionResponseRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: BroadcastTransactionBodyKind = instance.kind;
    }
    #[test]
    fn test_field_transaction() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.transaction;
    }
    #[test]
    fn test_field_data() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.data;
    }
    #[test]
    fn test_field_gas_limit() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.gas_limit;
    }
    #[test]
    fn test_field_nonce() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<Nonce> = instance.nonce;
    }
    #[test]
    fn test_field_to() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.to;
    }
    #[test]
    fn test_field_value() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.value;
    }
    #[test]
    fn test_field_max_fee_per_gas() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.max_fee_per_gas;
    }
    #[test]
    fn test_field_max_priority_fee_per_gas() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.max_priority_fee_per_gas;
    }
    #[test]
    fn test_field_gas_price() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.gas_price;
    }
    #[test]
    fn test_field_psbt() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let _: Option<String> = instance.psbt;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BroadcastTransactionResponseRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_broadcasttransactionbodykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionBodyKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionBodyKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionBodyKind::default();
        let b = BroadcastTransactionBodyKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionBodyKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionBodyKind =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionBodyKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionBodyKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionBodyKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionBodyKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionBodyKind>();
        let align = std::mem::align_of::<BroadcastTransactionBodyKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BroadcastTransactionBodyKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionBodyKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionBodyKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionBodyKind>>();
        let type_size = std::mem::size_of::<BroadcastTransactionBodyKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionBodyKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionBodyKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_nonce {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Nonce::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Nonce::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Nonce::default();
        let b = Nonce::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Nonce::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Nonce = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Nonce::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Nonce = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Nonce::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Nonce =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Nonce::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Nonce>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Nonce>();
        let align = std::mem::align_of::<Nonce>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Nonce));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Nonce::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Nonce>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Nonce>>();
        let type_size = std::mem::size_of::<Nonce>();
        println!("Option<{}> size: {} bytes", stringify!(Nonce), option_size);
        println!("Raw {} size: {} bytes", stringify!(Nonce), type_size);
    }
}
#[cfg(test)]
mod test_broadcasttransactionresponserequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionResponseRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionResponseRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionResponseRequester::default();
        let b = BroadcastTransactionResponseRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionResponseRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionResponseRequester =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionResponseRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionResponseRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionResponseRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionResponseRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionResponseRequester>();
        let align = std::mem::align_of::<BroadcastTransactionResponseRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(BroadcastTransactionResponseRequester)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionResponseRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionResponseRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionResponseRequester>>();
        let type_size = std::mem::size_of::<BroadcastTransactionResponseRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionResponseRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionResponseRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = BroadcastTransactionResponseRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = BroadcastTransactionResponseRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = BroadcastTransactionResponseRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BroadcastTransactionResponseRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_broadcasttransactionresponsestatus {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionResponseStatus::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionResponseStatus::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionResponseStatus::default();
        let b = BroadcastTransactionResponseStatus::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionResponseStatus::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionResponseStatus =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionResponseStatus::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionResponseStatus =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionResponseStatus::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionResponseStatus =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionResponseStatus::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionResponseStatus>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionResponseStatus>();
        let align = std::mem::align_of::<BroadcastTransactionResponseStatus>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(BroadcastTransactionResponseStatus)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionResponseStatus::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionResponseStatus>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionResponseStatus>>();
        let type_size = std::mem::size_of::<BroadcastTransactionResponseStatus>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionResponseStatus),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionResponseStatus),
            type_size
        );
    }
}
#[cfg(test)]
mod test_broadcasttransactionrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionRequest::default();
        let b = BroadcastTransactionRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionRequest>();
        let align = std::mem::align_of::<BroadcastTransactionRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BroadcastTransactionRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionRequest>>();
        let type_size = std::mem::size_of::<BroadcastTransactionRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = BroadcastTransactionRequest::default();
        let _: BroadcastTransactionBody = instance.body;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = BroadcastTransactionRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BroadcastTransactionRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_broadcasttransactionbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BroadcastTransactionBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BroadcastTransactionBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BroadcastTransactionBody::default();
        let b = BroadcastTransactionBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BroadcastTransactionBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BroadcastTransactionBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BroadcastTransactionBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BroadcastTransactionBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BroadcastTransactionBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BroadcastTransactionBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BroadcastTransactionBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BroadcastTransactionBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BroadcastTransactionBody>();
        let align = std::mem::align_of::<BroadcastTransactionBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BroadcastTransactionBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BroadcastTransactionBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BroadcastTransactionBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BroadcastTransactionBody>>();
        let type_size = std::mem::size_of::<BroadcastTransactionBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BroadcastTransactionBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BroadcastTransactionBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = BroadcastTransactionBody::default();
        let _: BroadcastTransactionBodyKind = instance.kind;
    }
    #[test]
    fn test_field_transaction() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.transaction;
    }
    #[test]
    fn test_field_data() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.data;
    }
    #[test]
    fn test_field_gas_limit() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.gas_limit;
    }
    #[test]
    fn test_field_nonce() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<Nonce> = instance.nonce;
    }
    #[test]
    fn test_field_to() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.to;
    }
    #[test]
    fn test_field_value() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.value;
    }
    #[test]
    fn test_field_max_fee_per_gas() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.max_fee_per_gas;
    }
    #[test]
    fn test_field_max_priority_fee_per_gas() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.max_priority_fee_per_gas;
    }
    #[test]
    fn test_field_gas_price() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.gas_price;
    }
    #[test]
    fn test_field_psbt() {
        let instance = BroadcastTransactionBody::default();
        let _: Option<String> = instance.psbt;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BroadcastTransactionBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwalletbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWalletBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWalletBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWalletBody::default();
        let b = CreateWalletBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWalletBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWalletBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWalletBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWalletBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWalletBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWalletBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWalletBody>();
        let align = std::mem::align_of::<CreateWalletBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWalletBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWalletBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWalletBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWalletBody>>();
        let type_size = std::mem::size_of::<CreateWalletBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWalletBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWalletBody),
            type_size
        );
    }
    #[test]
    fn test_field_delay_delegation() {
        let instance = CreateWalletBody::default();
        let _: Option<bool> = instance.delay_delegation;
    }
    #[test]
    fn test_field_delegate_to() {
        let instance = CreateWalletBody::default();
        let _: Option<String> = instance.delegate_to;
    }
    #[test]
    fn test_field_external_id() {
        let instance = CreateWalletBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_name() {
        let instance = CreateWalletBody::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = CreateWalletBody::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_signing_key() {
        let instance = CreateWalletBody::default();
        let _: Option<CreateWalletBodySigningKey> = instance.signing_key;
    }
    #[test]
    fn test_field_tags() {
        let instance = CreateWalletBody::default();
        let _: Option<Vec<String>> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWalletBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwalletbodynetwork {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWalletBodyNetwork::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWalletBodyNetwork::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWalletBodyNetwork::default();
        let b = CreateWalletBodyNetwork::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWalletBodyNetwork::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWalletBodyNetwork = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWalletBodyNetwork::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWalletBodyNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWalletBodyNetwork::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWalletBodyNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWalletBodyNetwork::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWalletBodyNetwork>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWalletBodyNetwork>();
        let align = std::mem::align_of::<CreateWalletBodyNetwork>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWalletBodyNetwork));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWalletBodyNetwork::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWalletBodyNetwork>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWalletBodyNetwork>>();
        let type_size = std::mem::size_of::<CreateWalletBodyNetwork>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWalletBodyNetwork),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWalletBodyNetwork),
            type_size
        );
    }
}
#[cfg(test)]
mod test_createwalletbodysigningkey {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWalletBodySigningKey::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWalletBodySigningKey::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWalletBodySigningKey::default();
        let b = CreateWalletBodySigningKey::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWalletBodySigningKey::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWalletBodySigningKey = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWalletBodySigningKey::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWalletBodySigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWalletBodySigningKey::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWalletBodySigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWalletBodySigningKey::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWalletBodySigningKey>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWalletBodySigningKey>();
        let align = std::mem::align_of::<CreateWalletBodySigningKey>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWalletBodySigningKey));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWalletBodySigningKey::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWalletBodySigningKey>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWalletBodySigningKey>>();
        let type_size = std::mem::size_of::<CreateWalletBodySigningKey>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWalletBodySigningKey),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWalletBodySigningKey),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = CreateWalletBodySigningKey::default();
        let _: Option<Curve> = instance.curve;
    }
    #[test]
    fn test_field_scheme() {
        let instance = CreateWalletBodySigningKey::default();
        let _: Option<Scheme> = instance.scheme;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWalletBodySigningKey::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_curve {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Curve::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Curve::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Curve::default();
        let b = Curve::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Curve::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Curve = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Curve::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Curve = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Curve::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Curve =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Curve::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Curve>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Curve>();
        let align = std::mem::align_of::<Curve>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Curve));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Curve::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Curve>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Curve>>();
        let type_size = std::mem::size_of::<Curve>();
        println!("Option<{}> size: {} bytes", stringify!(Curve), option_size);
        println!("Raw {} size: {} bytes", stringify!(Curve), type_size);
    }
}
#[cfg(test)]
mod test_scheme {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Scheme::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Scheme::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Scheme::default();
        let b = Scheme::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Scheme::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Scheme = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Scheme::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Scheme =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Scheme::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Scheme =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Scheme::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Scheme>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Scheme>();
        let align = std::mem::align_of::<Scheme>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Scheme));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Scheme::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Scheme>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Scheme>>();
        let type_size = std::mem::size_of::<Scheme>();
        println!("Option<{}> size: {} bytes", stringify!(Scheme), option_size);
        println!("Raw {} size: {} bytes", stringify!(Scheme), type_size);
    }
}
#[cfg(test)]
mod test_createwalletresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWalletResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWalletResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWalletResponse::default();
        let b = CreateWalletResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWalletResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWalletResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWalletResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWalletResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWalletResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWalletResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWalletResponse>();
        let align = std::mem::align_of::<CreateWalletResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWalletResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWalletResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWalletResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWalletResponse>>();
        let type_size = std::mem::size_of::<CreateWalletResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWalletResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWalletResponse),
            type_size
        );
    }
    #[test]
    fn test_field_address() {
        let instance = CreateWalletResponse::default();
        let _: Option<String> = instance.address;
    }
    #[test]
    fn test_field_custodial() {
        let instance = CreateWalletResponse::default();
        let _: bool = instance.custodial;
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateWalletResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_exported() {
        let instance = CreateWalletResponse::default();
        let _: Option<String> = instance.date_exported;
    }
    #[test]
    fn test_field_exported() {
        let instance = CreateWalletResponse::default();
        let _: Option<bool> = instance.exported;
    }
    #[test]
    fn test_field_external_id() {
        let instance = CreateWalletResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateWalletResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_imported() {
        let instance = CreateWalletResponse::default();
        let _: Option<bool> = instance.imported;
    }
    #[test]
    fn test_field_name() {
        let instance = CreateWalletResponse::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = CreateWalletResponse::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_signing_key() {
        let instance = CreateWalletResponse::default();
        let _: CreateWalletResponseSigningKey = instance.signing_key;
    }
    #[test]
    fn test_field_status() {
        let instance = CreateWalletResponse::default();
        let _: CreateWalletResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tags() {
        let instance = CreateWalletResponse::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWalletResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwalletresponsesigningkey {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWalletResponseSigningKey::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWalletResponseSigningKey::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWalletResponseSigningKey::default();
        let b = CreateWalletResponseSigningKey::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWalletResponseSigningKey::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWalletResponseSigningKey =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWalletResponseSigningKey::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWalletResponseSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWalletResponseSigningKey::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWalletResponseSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWalletResponseSigningKey::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWalletResponseSigningKey>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWalletResponseSigningKey>();
        let align = std::mem::align_of::<CreateWalletResponseSigningKey>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateWalletResponseSigningKey)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWalletResponseSigningKey::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWalletResponseSigningKey>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWalletResponseSigningKey>>();
        let type_size = std::mem::size_of::<CreateWalletResponseSigningKey>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWalletResponseSigningKey),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWalletResponseSigningKey),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = CreateWalletResponseSigningKey::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_public_key() {
        let instance = CreateWalletResponseSigningKey::default();
        let _: String = instance.public_key;
    }
    #[test]
    fn test_field_scheme() {
        let instance = CreateWalletResponseSigningKey::default();
        let _: Scheme = instance.scheme;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWalletResponseSigningKey::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwalletresponsestatus {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWalletResponseStatus::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWalletResponseStatus::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWalletResponseStatus::default();
        let b = CreateWalletResponseStatus::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWalletResponseStatus::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWalletResponseStatus = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWalletResponseStatus::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWalletResponseStatus =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWalletResponseStatus::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWalletResponseStatus =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWalletResponseStatus::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWalletResponseStatus>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWalletResponseStatus>();
        let align = std::mem::align_of::<CreateWalletResponseStatus>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWalletResponseStatus));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWalletResponseStatus::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWalletResponseStatus>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWalletResponseStatus>>();
        let type_size = std::mem::size_of::<CreateWalletResponseStatus>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWalletResponseStatus),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWalletResponseStatus),
            type_size
        );
    }
}
#[cfg(test)]
mod test_createwalletrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWalletRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWalletRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWalletRequest::default();
        let b = CreateWalletRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWalletRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWalletRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWalletRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWalletRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWalletRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWalletRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWalletRequest>();
        let align = std::mem::align_of::<CreateWalletRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWalletRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWalletRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWalletRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWalletRequest>>();
        let type_size = std::mem::size_of::<CreateWalletRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWalletRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWalletRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = CreateWalletRequest::default();
        let _: CreateWalletRequestBody = instance.body;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWalletRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwalletrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWalletRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWalletRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWalletRequestBody::default();
        let b = CreateWalletRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWalletRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWalletRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWalletRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWalletRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWalletRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWalletRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWalletRequestBody>();
        let align = std::mem::align_of::<CreateWalletRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWalletRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWalletRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWalletRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWalletRequestBody>>();
        let type_size = std::mem::size_of::<CreateWalletRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWalletRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWalletRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_delay_delegation() {
        let instance = CreateWalletRequestBody::default();
        let _: Option<bool> = instance.delay_delegation;
    }
    #[test]
    fn test_field_delegate_to() {
        let instance = CreateWalletRequestBody::default();
        let _: Option<String> = instance.delegate_to;
    }
    #[test]
    fn test_field_external_id() {
        let instance = CreateWalletRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_name() {
        let instance = CreateWalletRequestBody::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = CreateWalletRequestBody::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_signing_key() {
        let instance = CreateWalletRequestBody::default();
        let _: Option<BodySigningKey> = instance.signing_key;
    }
    #[test]
    fn test_field_tags() {
        let instance = CreateWalletRequestBody::default();
        let _: Option<Vec<String>> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWalletRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_bodysigningkey {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BodySigningKey::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BodySigningKey::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BodySigningKey::default();
        let b = BodySigningKey::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BodySigningKey::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BodySigningKey = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BodySigningKey::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BodySigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BodySigningKey::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BodySigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BodySigningKey::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BodySigningKey>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BodySigningKey>();
        let align = std::mem::align_of::<BodySigningKey>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BodySigningKey));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BodySigningKey::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BodySigningKey>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BodySigningKey>>();
        let type_size = std::mem::size_of::<BodySigningKey>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BodySigningKey),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BodySigningKey),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = BodySigningKey::default();
        let _: Option<Curve> = instance.curve;
    }
    #[test]
    fn test_field_scheme() {
        let instance = BodySigningKey::default();
        let _: Option<Scheme> = instance.scheme;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BodySigningKey::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_delegatewalletbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DelegateWalletBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DelegateWalletBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DelegateWalletBody::default();
        let b = DelegateWalletBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DelegateWalletBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DelegateWalletBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DelegateWalletBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DelegateWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DelegateWalletBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DelegateWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DelegateWalletBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DelegateWalletBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DelegateWalletBody>();
        let align = std::mem::align_of::<DelegateWalletBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DelegateWalletBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DelegateWalletBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DelegateWalletBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DelegateWalletBody>>();
        let type_size = std::mem::size_of::<DelegateWalletBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DelegateWalletBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DelegateWalletBody),
            type_size
        );
    }
    #[test]
    fn test_field_user_id() {
        let instance = DelegateWalletBody::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DelegateWalletBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_delegatewalletparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DelegateWalletParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DelegateWalletParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DelegateWalletParams::default();
        let b = DelegateWalletParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DelegateWalletParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DelegateWalletParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DelegateWalletParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DelegateWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DelegateWalletParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DelegateWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DelegateWalletParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DelegateWalletParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DelegateWalletParams>();
        let align = std::mem::align_of::<DelegateWalletParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DelegateWalletParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DelegateWalletParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DelegateWalletParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DelegateWalletParams>>();
        let type_size = std::mem::size_of::<DelegateWalletParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DelegateWalletParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DelegateWalletParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = DelegateWalletParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DelegateWalletParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_delegatewalletresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DelegateWalletResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DelegateWalletResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DelegateWalletResponse::default();
        let b = DelegateWalletResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DelegateWalletResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DelegateWalletResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DelegateWalletResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DelegateWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DelegateWalletResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DelegateWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DelegateWalletResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DelegateWalletResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DelegateWalletResponse>();
        let align = std::mem::align_of::<DelegateWalletResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DelegateWalletResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DelegateWalletResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DelegateWalletResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DelegateWalletResponse>>();
        let type_size = std::mem::size_of::<DelegateWalletResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DelegateWalletResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DelegateWalletResponse),
            type_size
        );
    }
    #[test]
    fn test_field_status() {
        let instance = DelegateWalletResponse::default();
        let _: DelegateWalletResponseStatus = instance.status;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = DelegateWalletResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DelegateWalletResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_delegatewalletresponsestatus {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DelegateWalletResponseStatus::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DelegateWalletResponseStatus::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DelegateWalletResponseStatus::default();
        let b = DelegateWalletResponseStatus::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DelegateWalletResponseStatus::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DelegateWalletResponseStatus =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DelegateWalletResponseStatus::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DelegateWalletResponseStatus =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DelegateWalletResponseStatus::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DelegateWalletResponseStatus =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DelegateWalletResponseStatus::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DelegateWalletResponseStatus>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DelegateWalletResponseStatus>();
        let align = std::mem::align_of::<DelegateWalletResponseStatus>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DelegateWalletResponseStatus));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DelegateWalletResponseStatus::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DelegateWalletResponseStatus>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DelegateWalletResponseStatus>>();
        let type_size = std::mem::size_of::<DelegateWalletResponseStatus>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DelegateWalletResponseStatus),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DelegateWalletResponseStatus),
            type_size
        );
    }
}
#[cfg(test)]
mod test_delegatewalletrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DelegateWalletRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DelegateWalletRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DelegateWalletRequest::default();
        let b = DelegateWalletRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DelegateWalletRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DelegateWalletRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DelegateWalletRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DelegateWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DelegateWalletRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DelegateWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DelegateWalletRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DelegateWalletRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DelegateWalletRequest>();
        let align = std::mem::align_of::<DelegateWalletRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DelegateWalletRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DelegateWalletRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DelegateWalletRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DelegateWalletRequest>>();
        let type_size = std::mem::size_of::<DelegateWalletRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DelegateWalletRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DelegateWalletRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = DelegateWalletRequest::default();
        let _: DelegateWalletRequestBody = instance.body;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = DelegateWalletRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DelegateWalletRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_delegatewalletrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DelegateWalletRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DelegateWalletRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DelegateWalletRequestBody::default();
        let b = DelegateWalletRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DelegateWalletRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DelegateWalletRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DelegateWalletRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DelegateWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DelegateWalletRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DelegateWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DelegateWalletRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DelegateWalletRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DelegateWalletRequestBody>();
        let align = std::mem::align_of::<DelegateWalletRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DelegateWalletRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DelegateWalletRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DelegateWalletRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DelegateWalletRequestBody>>();
        let type_size = std::mem::size_of::<DelegateWalletRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DelegateWalletRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DelegateWalletRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_user_id() {
        let instance = DelegateWalletRequestBody::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DelegateWalletRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_exportwalletbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ExportWalletBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ExportWalletBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ExportWalletBody::default();
        let b = ExportWalletBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ExportWalletBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ExportWalletBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ExportWalletBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ExportWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ExportWalletBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ExportWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ExportWalletBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ExportWalletBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ExportWalletBody>();
        let align = std::mem::align_of::<ExportWalletBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ExportWalletBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ExportWalletBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ExportWalletBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ExportWalletBody>>();
        let type_size = std::mem::size_of::<ExportWalletBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ExportWalletBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ExportWalletBody),
            type_size
        );
    }
    #[test]
    fn test_field_encryption_key() {
        let instance = ExportWalletBody::default();
        let _: String = instance.encryption_key;
    }
    #[test]
    fn test_field_supported_schemes() {
        let instance = ExportWalletBody::default();
        let _: Vec<ExportWalletBodySupportedScheme> = instance.supported_schemes;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ExportWalletBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_exportwalletbodysupportedscheme {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ExportWalletBodySupportedScheme::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ExportWalletBodySupportedScheme::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ExportWalletBodySupportedScheme::default();
        let b = ExportWalletBodySupportedScheme::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ExportWalletBodySupportedScheme::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ExportWalletBodySupportedScheme =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ExportWalletBodySupportedScheme::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ExportWalletBodySupportedScheme =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ExportWalletBodySupportedScheme::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ExportWalletBodySupportedScheme =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ExportWalletBodySupportedScheme::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ExportWalletBodySupportedScheme>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ExportWalletBodySupportedScheme>();
        let align = std::mem::align_of::<ExportWalletBodySupportedScheme>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ExportWalletBodySupportedScheme)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ExportWalletBodySupportedScheme::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ExportWalletBodySupportedScheme>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ExportWalletBodySupportedScheme>>();
        let type_size = std::mem::size_of::<ExportWalletBodySupportedScheme>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ExportWalletBodySupportedScheme),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ExportWalletBodySupportedScheme),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = ExportWalletBodySupportedScheme::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_protocol() {
        let instance = ExportWalletBodySupportedScheme::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ExportWalletBodySupportedScheme::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_protocol {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Protocol::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Protocol::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Protocol::default();
        let b = Protocol::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Protocol::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Protocol = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Protocol::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Protocol =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Protocol::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Protocol =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Protocol::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Protocol>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Protocol>();
        let align = std::mem::align_of::<Protocol>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Protocol));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Protocol::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Protocol>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Protocol>>();
        let type_size = std::mem::size_of::<Protocol>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(Protocol),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(Protocol), type_size);
    }
}
#[cfg(test)]
mod test_exportwalletparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ExportWalletParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ExportWalletParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ExportWalletParams::default();
        let b = ExportWalletParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ExportWalletParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ExportWalletParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ExportWalletParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ExportWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ExportWalletParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ExportWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ExportWalletParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ExportWalletParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ExportWalletParams>();
        let align = std::mem::align_of::<ExportWalletParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ExportWalletParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ExportWalletParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ExportWalletParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ExportWalletParams>>();
        let type_size = std::mem::size_of::<ExportWalletParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ExportWalletParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ExportWalletParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ExportWalletParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ExportWalletParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_exportwalletresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ExportWalletResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ExportWalletResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ExportWalletResponse::default();
        let b = ExportWalletResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ExportWalletResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ExportWalletResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ExportWalletResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ExportWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ExportWalletResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ExportWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ExportWalletResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ExportWalletResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ExportWalletResponse>();
        let align = std::mem::align_of::<ExportWalletResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ExportWalletResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ExportWalletResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ExportWalletResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ExportWalletResponse>>();
        let type_size = std::mem::size_of::<ExportWalletResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ExportWalletResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ExportWalletResponse),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = ExportWalletResponse::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_encrypted_key_shares() {
        let instance = ExportWalletResponse::default();
        let _: Vec<ExportWalletResponseEncryptedKeyShare> = instance.encrypted_key_shares;
    }
    #[test]
    fn test_field_min_signers() {
        let instance = ExportWalletResponse::default();
        let _: f64 = instance.min_signers;
    }
    #[test]
    fn test_field_protocol() {
        let instance = ExportWalletResponse::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_public_key() {
        let instance = ExportWalletResponse::default();
        let _: String = instance.public_key;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ExportWalletResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_exportwalletresponseencryptedkeyshare {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ExportWalletResponseEncryptedKeyShare::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ExportWalletResponseEncryptedKeyShare::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ExportWalletResponseEncryptedKeyShare::default();
        let b = ExportWalletResponseEncryptedKeyShare::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ExportWalletResponseEncryptedKeyShare::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ExportWalletResponseEncryptedKeyShare =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ExportWalletResponseEncryptedKeyShare::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ExportWalletResponseEncryptedKeyShare =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ExportWalletResponseEncryptedKeyShare::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ExportWalletResponseEncryptedKeyShare =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ExportWalletResponseEncryptedKeyShare::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ExportWalletResponseEncryptedKeyShare>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ExportWalletResponseEncryptedKeyShare>();
        let align = std::mem::align_of::<ExportWalletResponseEncryptedKeyShare>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ExportWalletResponseEncryptedKeyShare)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ExportWalletResponseEncryptedKeyShare::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ExportWalletResponseEncryptedKeyShare>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ExportWalletResponseEncryptedKeyShare>>();
        let type_size = std::mem::size_of::<ExportWalletResponseEncryptedKeyShare>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ExportWalletResponseEncryptedKeyShare),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ExportWalletResponseEncryptedKeyShare),
            type_size
        );
    }
    #[test]
    fn test_field_encrypted_key_share() {
        let instance = ExportWalletResponseEncryptedKeyShare::default();
        let _: String = instance.encrypted_key_share;
    }
    #[test]
    fn test_field_signer_id() {
        let instance = ExportWalletResponseEncryptedKeyShare::default();
        let _: String = instance.signer_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ExportWalletResponseEncryptedKeyShare::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_exportwalletrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ExportWalletRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ExportWalletRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ExportWalletRequest::default();
        let b = ExportWalletRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ExportWalletRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ExportWalletRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ExportWalletRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ExportWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ExportWalletRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ExportWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ExportWalletRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ExportWalletRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ExportWalletRequest>();
        let align = std::mem::align_of::<ExportWalletRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ExportWalletRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ExportWalletRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ExportWalletRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ExportWalletRequest>>();
        let type_size = std::mem::size_of::<ExportWalletRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ExportWalletRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ExportWalletRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = ExportWalletRequest::default();
        let _: ExportWalletRequestBody = instance.body;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ExportWalletRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ExportWalletRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_exportwalletrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ExportWalletRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ExportWalletRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ExportWalletRequestBody::default();
        let b = ExportWalletRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ExportWalletRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ExportWalletRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ExportWalletRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ExportWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ExportWalletRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ExportWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ExportWalletRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ExportWalletRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ExportWalletRequestBody>();
        let align = std::mem::align_of::<ExportWalletRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ExportWalletRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ExportWalletRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ExportWalletRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ExportWalletRequestBody>>();
        let type_size = std::mem::size_of::<ExportWalletRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ExportWalletRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ExportWalletRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_encryption_key() {
        let instance = ExportWalletRequestBody::default();
        let _: String = instance.encryption_key;
    }
    #[test]
    fn test_field_supported_schemes() {
        let instance = ExportWalletRequestBody::default();
        let _: Vec<BodySupportedScheme> = instance.supported_schemes;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ExportWalletRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_bodysupportedscheme {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BodySupportedScheme::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BodySupportedScheme::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BodySupportedScheme::default();
        let b = BodySupportedScheme::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BodySupportedScheme::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BodySupportedScheme = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BodySupportedScheme::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BodySupportedScheme =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BodySupportedScheme::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BodySupportedScheme =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BodySupportedScheme::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BodySupportedScheme>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BodySupportedScheme>();
        let align = std::mem::align_of::<BodySupportedScheme>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BodySupportedScheme));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BodySupportedScheme::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BodySupportedScheme>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BodySupportedScheme>>();
        let type_size = std::mem::size_of::<BodySupportedScheme>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BodySupportedScheme),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BodySupportedScheme),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = BodySupportedScheme::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_protocol() {
        let instance = BodySupportedScheme::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BodySupportedScheme::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_generatesignatureparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureParams::default();
        let b = GenerateSignatureParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureParams>();
        let align = std::mem::align_of::<GenerateSignatureParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GenerateSignatureParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureParams>>();
        let type_size = std::mem::size_of::<GenerateSignatureParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GenerateSignatureParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GenerateSignatureParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_generatesignatureresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureResponse::default();
        let b = GenerateSignatureResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureResponse>();
        let align = std::mem::align_of::<GenerateSignatureResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GenerateSignatureResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureResponse>>();
        let type_size = std::mem::size_of::<GenerateSignatureResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureResponse),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = GenerateSignatureResponse::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_date_signed() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.date_signed;
    }
    #[test]
    fn test_field_external_id() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = GenerateSignatureResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_network() {
        let instance = GenerateSignatureResponse::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = GenerateSignatureResponse::default();
        let _: GenerateSignatureResponseRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = GenerateSignatureResponse::default();
        let _: GenerateSignatureResponseRequester = instance.requester;
    }
    #[test]
    fn test_field_signature() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<PurpleSignature> = instance.signature;
    }
    #[test]
    fn test_field_signatures() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<Vec<FluffySignature>> = instance.signatures;
    }
    #[test]
    fn test_field_signed_data() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.signed_data;
    }
    #[test]
    fn test_field_status() {
        let instance = GenerateSignatureResponse::default();
        let _: GenerateSignatureResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = GenerateSignatureResponse::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GenerateSignatureResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GenerateSignatureResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_generatesignatureresponserequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureResponseRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureResponseRequestBody::default();
        let b = GenerateSignatureResponseRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureResponseRequestBody =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureResponseRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureResponseRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureResponseRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureResponseRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureResponseRequestBody>();
        let align = std::mem::align_of::<GenerateSignatureResponseRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(GenerateSignatureResponseRequestBody)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureResponseRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureResponseRequestBody>>();
        let type_size = std::mem::size_of::<GenerateSignatureResponseRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureResponseRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureResponseRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: GenerateSignatureBodyKind = instance.kind;
    }
    #[test]
    fn test_field_sign_doc() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<String> = instance.sign_doc;
    }
    #[test]
    fn test_field_hash() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<String> = instance.hash;
    }
    #[test]
    fn test_field_taproot_merkle_root() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<String> = instance.taproot_merkle_root;
    }
    #[test]
    fn test_field_message() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<Message> = instance.message;
    }
    #[test]
    fn test_field_transaction() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<String> = instance.transaction;
    }
    #[test]
    fn test_field_domain() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<PurpleDomain> = instance.domain;
    }
    #[test]
    fn test_field_types() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<HashMap<String, Vec<PurpleType>>> = instance.types;
    }
    #[test]
    fn test_field_psbt() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<String> = instance.psbt;
    }
    #[test]
    fn test_field_format() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let _: Option<Format> = instance.format;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GenerateSignatureResponseRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_purpledomain {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PurpleDomain::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PurpleDomain::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PurpleDomain::default();
        let b = PurpleDomain::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PurpleDomain::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PurpleDomain = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PurpleDomain::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PurpleDomain =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PurpleDomain::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PurpleDomain =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PurpleDomain::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PurpleDomain>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PurpleDomain>();
        let align = std::mem::align_of::<PurpleDomain>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PurpleDomain));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PurpleDomain::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PurpleDomain>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PurpleDomain>>();
        let type_size = std::mem::size_of::<PurpleDomain>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PurpleDomain),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(PurpleDomain), type_size);
    }
    #[test]
    fn test_field_chain_id() {
        let instance = PurpleDomain::default();
        let _: Option<Nonce> = instance.chain_id;
    }
    #[test]
    fn test_field_name() {
        let instance = PurpleDomain::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_salt() {
        let instance = PurpleDomain::default();
        let _: Option<String> = instance.salt;
    }
    #[test]
    fn test_field_verifying_contract() {
        let instance = PurpleDomain::default();
        let _: Option<String> = instance.verifying_contract;
    }
    #[test]
    fn test_field_version() {
        let instance = PurpleDomain::default();
        let _: Option<String> = instance.version;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PurpleDomain::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_format {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Format::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Format::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Format::default();
        let b = Format::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Format::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Format = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Format::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Format =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Format::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Format =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Format::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Format>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Format>();
        let align = std::mem::align_of::<Format>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Format));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Format::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Format>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Format>>();
        let type_size = std::mem::size_of::<Format>();
        println!("Option<{}> size: {} bytes", stringify!(Format), option_size);
        println!("Raw {} size: {} bytes", stringify!(Format), type_size);
    }
}
#[cfg(test)]
mod test_generatesignaturebodykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureBodyKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureBodyKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureBodyKind::default();
        let b = GenerateSignatureBodyKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureBodyKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureBodyKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureBodyKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureBodyKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureBodyKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureBodyKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureBodyKind>();
        let align = std::mem::align_of::<GenerateSignatureBodyKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GenerateSignatureBodyKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureBodyKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureBodyKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureBodyKind>>();
        let type_size = std::mem::size_of::<GenerateSignatureBodyKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureBodyKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureBodyKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_message {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Message::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Message::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Message::default();
        let b = Message::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Message::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Message = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Message::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Message =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Message::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Message =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Message::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Message>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Message>();
        let align = std::mem::align_of::<Message>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Message));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Message::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Message>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Message>>();
        let type_size = std::mem::size_of::<Message>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(Message),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(Message), type_size);
    }
}
#[cfg(test)]
mod test_purpletype {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PurpleType::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PurpleType::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PurpleType::default();
        let b = PurpleType::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PurpleType::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PurpleType = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PurpleType::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PurpleType =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PurpleType::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PurpleType =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PurpleType::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PurpleType>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PurpleType>();
        let align = std::mem::align_of::<PurpleType>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PurpleType));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PurpleType::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PurpleType>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PurpleType>>();
        let type_size = std::mem::size_of::<PurpleType>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PurpleType),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(PurpleType), type_size);
    }
    #[test]
    fn test_field_name() {
        let instance = PurpleType::default();
        let _: String = instance.name;
    }
    #[test]
    fn test_field_type_type() {
        let instance = PurpleType::default();
        let _: String = instance.type_type;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PurpleType::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_generatesignatureresponserequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureResponseRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureResponseRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureResponseRequester::default();
        let b = GenerateSignatureResponseRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureResponseRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureResponseRequester =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureResponseRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureResponseRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureResponseRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureResponseRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureResponseRequester>();
        let align = std::mem::align_of::<GenerateSignatureResponseRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(GenerateSignatureResponseRequester)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureResponseRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureResponseRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureResponseRequester>>();
        let type_size = std::mem::size_of::<GenerateSignatureResponseRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureResponseRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureResponseRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = GenerateSignatureResponseRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = GenerateSignatureResponseRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = GenerateSignatureResponseRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GenerateSignatureResponseRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_purplesignature {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PurpleSignature::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PurpleSignature::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PurpleSignature::default();
        let b = PurpleSignature::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PurpleSignature::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PurpleSignature = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PurpleSignature::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PurpleSignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PurpleSignature::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PurpleSignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PurpleSignature::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PurpleSignature>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PurpleSignature>();
        let align = std::mem::align_of::<PurpleSignature>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PurpleSignature));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PurpleSignature::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PurpleSignature>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PurpleSignature>>();
        let type_size = std::mem::size_of::<PurpleSignature>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PurpleSignature),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(PurpleSignature),
            type_size
        );
    }
    #[test]
    fn test_field_encoded() {
        let instance = PurpleSignature::default();
        let _: Option<String> = instance.encoded;
    }
    #[test]
    fn test_field_r() {
        let instance = PurpleSignature::default();
        let _: String = instance.r;
    }
    #[test]
    fn test_field_recid() {
        let instance = PurpleSignature::default();
        let _: Option<f64> = instance.recid;
    }
    #[test]
    fn test_field_s() {
        let instance = PurpleSignature::default();
        let _: String = instance.s;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PurpleSignature::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fluffysignature {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = FluffySignature::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = FluffySignature::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = FluffySignature::default();
        let b = FluffySignature::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = FluffySignature::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: FluffySignature = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = FluffySignature::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FluffySignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = FluffySignature::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: FluffySignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = FluffySignature::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<FluffySignature>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<FluffySignature>();
        let align = std::mem::align_of::<FluffySignature>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(FluffySignature));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = FluffySignature::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<FluffySignature>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<FluffySignature>>();
        let type_size = std::mem::size_of::<FluffySignature>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(FluffySignature),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(FluffySignature),
            type_size
        );
    }
    #[test]
    fn test_field_encoded() {
        let instance = FluffySignature::default();
        let _: Option<String> = instance.encoded;
    }
    #[test]
    fn test_field_r() {
        let instance = FluffySignature::default();
        let _: String = instance.r;
    }
    #[test]
    fn test_field_recid() {
        let instance = FluffySignature::default();
        let _: Option<f64> = instance.recid;
    }
    #[test]
    fn test_field_s() {
        let instance = FluffySignature::default();
        let _: String = instance.s;
    }
    #[test]
    fn check_field_attributes() {
        let instance = FluffySignature::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_generatesignatureresponsestatus {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureResponseStatus::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureResponseStatus::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureResponseStatus::default();
        let b = GenerateSignatureResponseStatus::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureResponseStatus::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureResponseStatus =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureResponseStatus::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureResponseStatus =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureResponseStatus::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureResponseStatus =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureResponseStatus::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureResponseStatus>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureResponseStatus>();
        let align = std::mem::align_of::<GenerateSignatureResponseStatus>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(GenerateSignatureResponseStatus)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureResponseStatus::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureResponseStatus>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureResponseStatus>>();
        let type_size = std::mem::size_of::<GenerateSignatureResponseStatus>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureResponseStatus),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureResponseStatus),
            type_size
        );
    }
}
#[cfg(test)]
mod test_generatesignaturerequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureRequest::default();
        let b = GenerateSignatureRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureRequest>();
        let align = std::mem::align_of::<GenerateSignatureRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GenerateSignatureRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureRequest>>();
        let type_size = std::mem::size_of::<GenerateSignatureRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = GenerateSignatureRequest::default();
        let _: GenerateSignatureBody = instance.body;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GenerateSignatureRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GenerateSignatureRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_generatesignaturebody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureBody::default();
        let b = GenerateSignatureBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureBody>();
        let align = std::mem::align_of::<GenerateSignatureBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GenerateSignatureBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureBody>>();
        let type_size = std::mem::size_of::<GenerateSignatureBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = GenerateSignatureBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = GenerateSignatureBody::default();
        let _: GenerateSignatureBodyKind = instance.kind;
    }
    #[test]
    fn test_field_sign_doc() {
        let instance = GenerateSignatureBody::default();
        let _: Option<String> = instance.sign_doc;
    }
    #[test]
    fn test_field_hash() {
        let instance = GenerateSignatureBody::default();
        let _: Option<String> = instance.hash;
    }
    #[test]
    fn test_field_taproot_merkle_root() {
        let instance = GenerateSignatureBody::default();
        let _: Option<String> = instance.taproot_merkle_root;
    }
    #[test]
    fn test_field_message() {
        let instance = GenerateSignatureBody::default();
        let _: Option<Message> = instance.message;
    }
    #[test]
    fn test_field_transaction() {
        let instance = GenerateSignatureBody::default();
        let _: Option<String> = instance.transaction;
    }
    #[test]
    fn test_field_domain() {
        let instance = GenerateSignatureBody::default();
        let _: Option<GenerateSignatureBodyDomain> = instance.domain;
    }
    #[test]
    fn test_field_types() {
        let instance = GenerateSignatureBody::default();
        let _: Option<HashMap<String, Vec<GenerateSignatureBodyType>>> = instance.types;
    }
    #[test]
    fn test_field_psbt() {
        let instance = GenerateSignatureBody::default();
        let _: Option<String> = instance.psbt;
    }
    #[test]
    fn test_field_format() {
        let instance = GenerateSignatureBody::default();
        let _: Option<Format> = instance.format;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GenerateSignatureBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_generatesignaturebodydomain {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureBodyDomain::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureBodyDomain::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureBodyDomain::default();
        let b = GenerateSignatureBodyDomain::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureBodyDomain::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureBodyDomain = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureBodyDomain::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureBodyDomain =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureBodyDomain::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureBodyDomain =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureBodyDomain::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureBodyDomain>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureBodyDomain>();
        let align = std::mem::align_of::<GenerateSignatureBodyDomain>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GenerateSignatureBodyDomain));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureBodyDomain::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureBodyDomain>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureBodyDomain>>();
        let type_size = std::mem::size_of::<GenerateSignatureBodyDomain>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureBodyDomain),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureBodyDomain),
            type_size
        );
    }
    #[test]
    fn test_field_chain_id() {
        let instance = GenerateSignatureBodyDomain::default();
        let _: Option<Nonce> = instance.chain_id;
    }
    #[test]
    fn test_field_name() {
        let instance = GenerateSignatureBodyDomain::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_salt() {
        let instance = GenerateSignatureBodyDomain::default();
        let _: Option<String> = instance.salt;
    }
    #[test]
    fn test_field_verifying_contract() {
        let instance = GenerateSignatureBodyDomain::default();
        let _: Option<String> = instance.verifying_contract;
    }
    #[test]
    fn test_field_version() {
        let instance = GenerateSignatureBodyDomain::default();
        let _: Option<String> = instance.version;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GenerateSignatureBodyDomain::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_generatesignaturebodytype {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GenerateSignatureBodyType::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GenerateSignatureBodyType::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GenerateSignatureBodyType::default();
        let b = GenerateSignatureBodyType::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GenerateSignatureBodyType::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GenerateSignatureBodyType = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GenerateSignatureBodyType::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GenerateSignatureBodyType =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GenerateSignatureBodyType::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GenerateSignatureBodyType =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GenerateSignatureBodyType::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GenerateSignatureBodyType>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GenerateSignatureBodyType>();
        let align = std::mem::align_of::<GenerateSignatureBodyType>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GenerateSignatureBodyType));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GenerateSignatureBodyType::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GenerateSignatureBodyType>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GenerateSignatureBodyType>>();
        let type_size = std::mem::size_of::<GenerateSignatureBodyType>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GenerateSignatureBodyType),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GenerateSignatureBodyType),
            type_size
        );
    }
    #[test]
    fn test_field_name() {
        let instance = GenerateSignatureBodyType::default();
        let _: String = instance.name;
    }
    #[test]
    fn test_field_type_type() {
        let instance = GenerateSignatureBodyType::default();
        let _: String = instance.type_type;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GenerateSignatureBodyType::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getsignatureparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetSignatureParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetSignatureParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetSignatureParams::default();
        let b = GetSignatureParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetSignatureParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetSignatureParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetSignatureParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetSignatureParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetSignatureParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetSignatureParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetSignatureParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetSignatureParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetSignatureParams>();
        let align = std::mem::align_of::<GetSignatureParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetSignatureParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetSignatureParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetSignatureParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetSignatureParams>>();
        let type_size = std::mem::size_of::<GetSignatureParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetSignatureParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetSignatureParams),
            type_size
        );
    }
    #[test]
    fn test_field_signature_id() {
        let instance = GetSignatureParams::default();
        let _: String = instance.signature_id;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetSignatureParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetSignatureParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getsignatureresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetSignatureResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetSignatureResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetSignatureResponse::default();
        let b = GetSignatureResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetSignatureResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetSignatureResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetSignatureResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetSignatureResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetSignatureResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetSignatureResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetSignatureResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetSignatureResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetSignatureResponse>();
        let align = std::mem::align_of::<GetSignatureResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetSignatureResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetSignatureResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetSignatureResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetSignatureResponse>>();
        let type_size = std::mem::size_of::<GetSignatureResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetSignatureResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetSignatureResponse),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = GetSignatureResponse::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_date_signed() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.date_signed;
    }
    #[test]
    fn test_field_external_id() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = GetSignatureResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_network() {
        let instance = GetSignatureResponse::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = GetSignatureResponse::default();
        let _: GetSignatureResponseRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = GetSignatureResponse::default();
        let _: GetSignatureResponseRequester = instance.requester;
    }
    #[test]
    fn test_field_signature() {
        let instance = GetSignatureResponse::default();
        let _: Option<TentacledSignature> = instance.signature;
    }
    #[test]
    fn test_field_signatures() {
        let instance = GetSignatureResponse::default();
        let _: Option<Vec<StickySignature>> = instance.signatures;
    }
    #[test]
    fn test_field_signed_data() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.signed_data;
    }
    #[test]
    fn test_field_status() {
        let instance = GetSignatureResponse::default();
        let _: GenerateSignatureResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = GetSignatureResponse::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetSignatureResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetSignatureResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getsignatureresponserequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetSignatureResponseRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetSignatureResponseRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetSignatureResponseRequestBody::default();
        let b = GetSignatureResponseRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetSignatureResponseRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetSignatureResponseRequestBody =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetSignatureResponseRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetSignatureResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetSignatureResponseRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetSignatureResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetSignatureResponseRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetSignatureResponseRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetSignatureResponseRequestBody>();
        let align = std::mem::align_of::<GetSignatureResponseRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(GetSignatureResponseRequestBody)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetSignatureResponseRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetSignatureResponseRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetSignatureResponseRequestBody>>();
        let type_size = std::mem::size_of::<GetSignatureResponseRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetSignatureResponseRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetSignatureResponseRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: GenerateSignatureBodyKind = instance.kind;
    }
    #[test]
    fn test_field_sign_doc() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<String> = instance.sign_doc;
    }
    #[test]
    fn test_field_hash() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<String> = instance.hash;
    }
    #[test]
    fn test_field_taproot_merkle_root() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<String> = instance.taproot_merkle_root;
    }
    #[test]
    fn test_field_message() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<Message> = instance.message;
    }
    #[test]
    fn test_field_transaction() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<String> = instance.transaction;
    }
    #[test]
    fn test_field_domain() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<FluffyDomain> = instance.domain;
    }
    #[test]
    fn test_field_types() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<HashMap<String, Vec<FluffyType>>> = instance.types;
    }
    #[test]
    fn test_field_psbt() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<String> = instance.psbt;
    }
    #[test]
    fn test_field_format() {
        let instance = GetSignatureResponseRequestBody::default();
        let _: Option<Format> = instance.format;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetSignatureResponseRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fluffydomain {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = FluffyDomain::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = FluffyDomain::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = FluffyDomain::default();
        let b = FluffyDomain::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = FluffyDomain::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: FluffyDomain = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = FluffyDomain::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FluffyDomain =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = FluffyDomain::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: FluffyDomain =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = FluffyDomain::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<FluffyDomain>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<FluffyDomain>();
        let align = std::mem::align_of::<FluffyDomain>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(FluffyDomain));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = FluffyDomain::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<FluffyDomain>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<FluffyDomain>>();
        let type_size = std::mem::size_of::<FluffyDomain>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(FluffyDomain),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(FluffyDomain), type_size);
    }
    #[test]
    fn test_field_chain_id() {
        let instance = FluffyDomain::default();
        let _: Option<Nonce> = instance.chain_id;
    }
    #[test]
    fn test_field_name() {
        let instance = FluffyDomain::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_salt() {
        let instance = FluffyDomain::default();
        let _: Option<String> = instance.salt;
    }
    #[test]
    fn test_field_verifying_contract() {
        let instance = FluffyDomain::default();
        let _: Option<String> = instance.verifying_contract;
    }
    #[test]
    fn test_field_version() {
        let instance = FluffyDomain::default();
        let _: Option<String> = instance.version;
    }
    #[test]
    fn check_field_attributes() {
        let instance = FluffyDomain::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fluffytype {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = FluffyType::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = FluffyType::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = FluffyType::default();
        let b = FluffyType::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = FluffyType::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: FluffyType = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = FluffyType::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FluffyType =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = FluffyType::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: FluffyType =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = FluffyType::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<FluffyType>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<FluffyType>();
        let align = std::mem::align_of::<FluffyType>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(FluffyType));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = FluffyType::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<FluffyType>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<FluffyType>>();
        let type_size = std::mem::size_of::<FluffyType>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(FluffyType),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(FluffyType), type_size);
    }
    #[test]
    fn test_field_name() {
        let instance = FluffyType::default();
        let _: String = instance.name;
    }
    #[test]
    fn test_field_type_type() {
        let instance = FluffyType::default();
        let _: String = instance.type_type;
    }
    #[test]
    fn check_field_attributes() {
        let instance = FluffyType::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getsignatureresponserequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetSignatureResponseRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetSignatureResponseRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetSignatureResponseRequester::default();
        let b = GetSignatureResponseRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetSignatureResponseRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetSignatureResponseRequester =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetSignatureResponseRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetSignatureResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetSignatureResponseRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetSignatureResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetSignatureResponseRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetSignatureResponseRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetSignatureResponseRequester>();
        let align = std::mem::align_of::<GetSignatureResponseRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(GetSignatureResponseRequester)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetSignatureResponseRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetSignatureResponseRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetSignatureResponseRequester>>();
        let type_size = std::mem::size_of::<GetSignatureResponseRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetSignatureResponseRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetSignatureResponseRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = GetSignatureResponseRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = GetSignatureResponseRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = GetSignatureResponseRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetSignatureResponseRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tentacledsignature {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TentacledSignature::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TentacledSignature::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TentacledSignature::default();
        let b = TentacledSignature::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TentacledSignature::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TentacledSignature = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TentacledSignature::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TentacledSignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TentacledSignature::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TentacledSignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TentacledSignature::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TentacledSignature>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TentacledSignature>();
        let align = std::mem::align_of::<TentacledSignature>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TentacledSignature));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TentacledSignature::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TentacledSignature>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TentacledSignature>>();
        let type_size = std::mem::size_of::<TentacledSignature>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TentacledSignature),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TentacledSignature),
            type_size
        );
    }
    #[test]
    fn test_field_encoded() {
        let instance = TentacledSignature::default();
        let _: Option<String> = instance.encoded;
    }
    #[test]
    fn test_field_r() {
        let instance = TentacledSignature::default();
        let _: String = instance.r;
    }
    #[test]
    fn test_field_recid() {
        let instance = TentacledSignature::default();
        let _: Option<f64> = instance.recid;
    }
    #[test]
    fn test_field_s() {
        let instance = TentacledSignature::default();
        let _: String = instance.s;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TentacledSignature::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_stickysignature {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = StickySignature::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = StickySignature::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = StickySignature::default();
        let b = StickySignature::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = StickySignature::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: StickySignature = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = StickySignature::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: StickySignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = StickySignature::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: StickySignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = StickySignature::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<StickySignature>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<StickySignature>();
        let align = std::mem::align_of::<StickySignature>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(StickySignature));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = StickySignature::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<StickySignature>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<StickySignature>>();
        let type_size = std::mem::size_of::<StickySignature>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(StickySignature),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(StickySignature),
            type_size
        );
    }
    #[test]
    fn test_field_encoded() {
        let instance = StickySignature::default();
        let _: Option<String> = instance.encoded;
    }
    #[test]
    fn test_field_r() {
        let instance = StickySignature::default();
        let _: String = instance.r;
    }
    #[test]
    fn test_field_recid() {
        let instance = StickySignature::default();
        let _: Option<f64> = instance.recid;
    }
    #[test]
    fn test_field_s() {
        let instance = StickySignature::default();
        let _: String = instance.s;
    }
    #[test]
    fn check_field_attributes() {
        let instance = StickySignature::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getsignaturerequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetSignatureRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetSignatureRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetSignatureRequest::default();
        let b = GetSignatureRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetSignatureRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetSignatureRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetSignatureRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetSignatureRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetSignatureRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetSignatureRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetSignatureRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetSignatureRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetSignatureRequest>();
        let align = std::mem::align_of::<GetSignatureRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetSignatureRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetSignatureRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetSignatureRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetSignatureRequest>>();
        let type_size = std::mem::size_of::<GetSignatureRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetSignatureRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetSignatureRequest),
            type_size
        );
    }
    #[test]
    fn test_field_signature_id() {
        let instance = GetSignatureRequest::default();
        let _: String = instance.signature_id;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetSignatureRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetSignatureRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransactionparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransactionParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransactionParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransactionParams::default();
        let b = GetTransactionParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransactionParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransactionParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransactionParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransactionParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransactionParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransactionParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransactionParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransactionParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransactionParams>();
        let align = std::mem::align_of::<GetTransactionParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetTransactionParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransactionParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransactionParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransactionParams>>();
        let type_size = std::mem::size_of::<GetTransactionParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransactionParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransactionParams),
            type_size
        );
    }
    #[test]
    fn test_field_transaction_id() {
        let instance = GetTransactionParams::default();
        let _: String = instance.transaction_id;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetTransactionParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransactionParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransactionresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransactionResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransactionResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransactionResponse::default();
        let b = GetTransactionResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransactionResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransactionResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransactionResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransactionResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransactionResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransactionResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransactionResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransactionResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransactionResponse>();
        let align = std::mem::align_of::<GetTransactionResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetTransactionResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransactionResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransactionResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransactionResponse>>();
        let type_size = std::mem::size_of::<GetTransactionResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransactionResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransactionResponse),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = GetTransactionResponse::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_broadcasted() {
        let instance = GetTransactionResponse::default();
        let _: Option<String> = instance.date_broadcasted;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = GetTransactionResponse::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = GetTransactionResponse::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = GetTransactionResponse::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_external_id() {
        let instance = GetTransactionResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = GetTransactionResponse::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = GetTransactionResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_network() {
        let instance = GetTransactionResponse::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = GetTransactionResponse::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = GetTransactionResponse::default();
        let _: GetTransactionResponseRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = GetTransactionResponse::default();
        let _: GetTransactionResponseRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = GetTransactionResponse::default();
        let _: BroadcastTransactionResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = GetTransactionResponse::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetTransactionResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransactionResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransactionresponserequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransactionResponseRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransactionResponseRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransactionResponseRequestBody::default();
        let b = GetTransactionResponseRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransactionResponseRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransactionResponseRequestBody =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransactionResponseRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransactionResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransactionResponseRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransactionResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransactionResponseRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransactionResponseRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransactionResponseRequestBody>();
        let align = std::mem::align_of::<GetTransactionResponseRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(GetTransactionResponseRequestBody)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransactionResponseRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransactionResponseRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransactionResponseRequestBody>>();
        let type_size = std::mem::size_of::<GetTransactionResponseRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransactionResponseRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransactionResponseRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: BroadcastTransactionBodyKind = instance.kind;
    }
    #[test]
    fn test_field_transaction() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.transaction;
    }
    #[test]
    fn test_field_data() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.data;
    }
    #[test]
    fn test_field_gas_limit() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.gas_limit;
    }
    #[test]
    fn test_field_nonce() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<Nonce> = instance.nonce;
    }
    #[test]
    fn test_field_to() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.to;
    }
    #[test]
    fn test_field_value() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.value;
    }
    #[test]
    fn test_field_max_fee_per_gas() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.max_fee_per_gas;
    }
    #[test]
    fn test_field_max_priority_fee_per_gas() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.max_priority_fee_per_gas;
    }
    #[test]
    fn test_field_gas_price() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.gas_price;
    }
    #[test]
    fn test_field_psbt() {
        let instance = GetTransactionResponseRequestBody::default();
        let _: Option<String> = instance.psbt;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransactionResponseRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransactionresponserequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransactionResponseRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransactionResponseRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransactionResponseRequester::default();
        let b = GetTransactionResponseRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransactionResponseRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransactionResponseRequester =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransactionResponseRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransactionResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransactionResponseRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransactionResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransactionResponseRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransactionResponseRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransactionResponseRequester>();
        let align = std::mem::align_of::<GetTransactionResponseRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(GetTransactionResponseRequester)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransactionResponseRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransactionResponseRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransactionResponseRequester>>();
        let type_size = std::mem::size_of::<GetTransactionResponseRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransactionResponseRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransactionResponseRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = GetTransactionResponseRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = GetTransactionResponseRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = GetTransactionResponseRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransactionResponseRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransactionrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransactionRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransactionRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransactionRequest::default();
        let b = GetTransactionRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransactionRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransactionRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransactionRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransactionRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransactionRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransactionRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransactionRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransactionRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransactionRequest>();
        let align = std::mem::align_of::<GetTransactionRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetTransactionRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransactionRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransactionRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransactionRequest>>();
        let type_size = std::mem::size_of::<GetTransactionRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransactionRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransactionRequest),
            type_size
        );
    }
    #[test]
    fn test_field_transaction_id() {
        let instance = GetTransactionRequest::default();
        let _: String = instance.transaction_id;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetTransactionRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransactionRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransferparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransferParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransferParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransferParams::default();
        let b = GetTransferParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransferParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransferParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransferParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransferParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransferParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransferParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransferParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransferParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransferParams>();
        let align = std::mem::align_of::<GetTransferParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetTransferParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransferParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransferParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransferParams>>();
        let type_size = std::mem::size_of::<GetTransferParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransferParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransferParams),
            type_size
        );
    }
    #[test]
    fn test_field_transfer_id() {
        let instance = GetTransferParams::default();
        let _: String = instance.transfer_id;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetTransferParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransferParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransferresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransferResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransferResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransferResponse::default();
        let b = GetTransferResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransferResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransferResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransferResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransferResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransferResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransferResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransferResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransferResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransferResponse>();
        let align = std::mem::align_of::<GetTransferResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetTransferResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransferResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransferResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransferResponse>>();
        let type_size = std::mem::size_of::<GetTransferResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransferResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransferResponse),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = GetTransferResponse::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_broadcasted() {
        let instance = GetTransferResponse::default();
        let _: Option<String> = instance.date_broadcasted;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = GetTransferResponse::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = GetTransferResponse::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = GetTransferResponse::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_external_id() {
        let instance = GetTransferResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = GetTransferResponse::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = GetTransferResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_metadata() {
        let instance = GetTransferResponse::default();
        let _: GetTransferResponseMetadata = instance.metadata;
    }
    #[test]
    fn test_field_network() {
        let instance = GetTransferResponse::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = GetTransferResponse::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = GetTransferResponse::default();
        let _: GetTransferResponseRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = GetTransferResponse::default();
        let _: GetTransferResponseRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = GetTransferResponse::default();
        let _: BroadcastTransactionResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = GetTransferResponse::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetTransferResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransferResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransferresponsemetadata {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransferResponseMetadata::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransferResponseMetadata::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransferResponseMetadata::default();
        let b = GetTransferResponseMetadata::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransferResponseMetadata::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransferResponseMetadata = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransferResponseMetadata::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransferResponseMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransferResponseMetadata::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransferResponseMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransferResponseMetadata::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransferResponseMetadata>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransferResponseMetadata>();
        let align = std::mem::align_of::<GetTransferResponseMetadata>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetTransferResponseMetadata));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransferResponseMetadata::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransferResponseMetadata>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransferResponseMetadata>>();
        let type_size = std::mem::size_of::<GetTransferResponseMetadata>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransferResponseMetadata),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransferResponseMetadata),
            type_size
        );
    }
    #[test]
    fn test_field_asset() {
        let instance = GetTransferResponseMetadata::default();
        let _: PurpleAsset = instance.asset;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransferResponseMetadata::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_purpleasset {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PurpleAsset::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PurpleAsset::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PurpleAsset::default();
        let b = PurpleAsset::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PurpleAsset::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PurpleAsset = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PurpleAsset::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PurpleAsset =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PurpleAsset::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PurpleAsset =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PurpleAsset::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PurpleAsset>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PurpleAsset>();
        let align = std::mem::align_of::<PurpleAsset>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PurpleAsset));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PurpleAsset::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PurpleAsset>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PurpleAsset>>();
        let type_size = std::mem::size_of::<PurpleAsset>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PurpleAsset),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(PurpleAsset), type_size);
    }
    #[test]
    fn test_field_decimals() {
        let instance = PurpleAsset::default();
        let _: Option<f64> = instance.decimals;
    }
    #[test]
    fn test_field_quotes() {
        let instance = PurpleAsset::default();
        let _: Option<HashMap<String, f64>> = instance.quotes;
    }
    #[test]
    fn test_field_symbol() {
        let instance = PurpleAsset::default();
        let _: Option<String> = instance.symbol;
    }
    #[test]
    fn test_field_verified() {
        let instance = PurpleAsset::default();
        let _: Option<bool> = instance.verified;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PurpleAsset::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransferresponserequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransferResponseRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransferResponseRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransferResponseRequestBody::default();
        let b = GetTransferResponseRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransferResponseRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransferResponseRequestBody =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransferResponseRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransferResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransferResponseRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransferResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransferResponseRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransferResponseRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransferResponseRequestBody>();
        let align = std::mem::align_of::<GetTransferResponseRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(GetTransferResponseRequestBody)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransferResponseRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransferResponseRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransferResponseRequestBody>>();
        let type_size = std::mem::size_of::<GetTransferResponseRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransferResponseRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransferResponseRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.amount;
    }
    #[test]
    fn test_field_create_destination_account() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<bool> = instance.create_destination_account;
    }
    #[test]
    fn test_field_external_id() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetTransferResponseRequestBody::default();
        let _: TransferAssetBodyKind = instance.kind;
    }
    #[test]
    fn test_field_memo() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.memo;
    }
    #[test]
    fn test_field_priority() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<Priority> = instance.priority;
    }
    #[test]
    fn test_field_to() {
        let instance = GetTransferResponseRequestBody::default();
        let _: String = instance.to;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_metadata() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.metadata;
    }
    #[test]
    fn test_field_contract() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = GetTransferResponseRequestBody::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransferResponseRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_transferassetbodykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TransferAssetBodyKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TransferAssetBodyKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TransferAssetBodyKind::default();
        let b = TransferAssetBodyKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TransferAssetBodyKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TransferAssetBodyKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TransferAssetBodyKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TransferAssetBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TransferAssetBodyKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TransferAssetBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TransferAssetBodyKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TransferAssetBodyKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TransferAssetBodyKind>();
        let align = std::mem::align_of::<TransferAssetBodyKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TransferAssetBodyKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TransferAssetBodyKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TransferAssetBodyKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TransferAssetBodyKind>>();
        let type_size = std::mem::size_of::<TransferAssetBodyKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TransferAssetBodyKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TransferAssetBodyKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_priority {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Priority::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Priority::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Priority::default();
        let b = Priority::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Priority::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Priority = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Priority::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Priority =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Priority::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Priority =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Priority::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Priority>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Priority>();
        let align = std::mem::align_of::<Priority>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Priority));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Priority::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Priority>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Priority>>();
        let type_size = std::mem::size_of::<Priority>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(Priority),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(Priority), type_size);
    }
}
#[cfg(test)]
mod test_gettransferresponserequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransferResponseRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransferResponseRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransferResponseRequester::default();
        let b = GetTransferResponseRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransferResponseRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransferResponseRequester =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransferResponseRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransferResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransferResponseRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransferResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransferResponseRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransferResponseRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransferResponseRequester>();
        let align = std::mem::align_of::<GetTransferResponseRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetTransferResponseRequester));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransferResponseRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransferResponseRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransferResponseRequester>>();
        let type_size = std::mem::size_of::<GetTransferResponseRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransferResponseRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransferResponseRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = GetTransferResponseRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = GetTransferResponseRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = GetTransferResponseRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransferResponseRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_gettransferrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetTransferRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetTransferRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetTransferRequest::default();
        let b = GetTransferRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetTransferRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetTransferRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetTransferRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetTransferRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetTransferRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetTransferRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetTransferRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetTransferRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetTransferRequest>();
        let align = std::mem::align_of::<GetTransferRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetTransferRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetTransferRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetTransferRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetTransferRequest>>();
        let type_size = std::mem::size_of::<GetTransferRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetTransferRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetTransferRequest),
            type_size
        );
    }
    #[test]
    fn test_field_transfer_id() {
        let instance = GetTransferRequest::default();
        let _: String = instance.transfer_id;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetTransferRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetTransferRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletParams::default();
        let b = GetWalletParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletParams>();
        let align = std::mem::align_of::<GetWalletParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletParams>>();
        let type_size = std::mem::size_of::<GetWalletParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletResponse::default();
        let b = GetWalletResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletResponse>();
        let align = std::mem::align_of::<GetWalletResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletResponse>>();
        let type_size = std::mem::size_of::<GetWalletResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletResponse),
            type_size
        );
    }
    #[test]
    fn test_field_address() {
        let instance = GetWalletResponse::default();
        let _: Option<String> = instance.address;
    }
    #[test]
    fn test_field_custodial() {
        let instance = GetWalletResponse::default();
        let _: bool = instance.custodial;
    }
    #[test]
    fn test_field_date_created() {
        let instance = GetWalletResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_exported() {
        let instance = GetWalletResponse::default();
        let _: Option<String> = instance.date_exported;
    }
    #[test]
    fn test_field_exported() {
        let instance = GetWalletResponse::default();
        let _: Option<bool> = instance.exported;
    }
    #[test]
    fn test_field_external_id() {
        let instance = GetWalletResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_id() {
        let instance = GetWalletResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_imported() {
        let instance = GetWalletResponse::default();
        let _: Option<bool> = instance.imported;
    }
    #[test]
    fn test_field_name() {
        let instance = GetWalletResponse::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = GetWalletResponse::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_signing_key() {
        let instance = GetWalletResponse::default();
        let _: GetWalletResponseSigningKey = instance.signing_key;
    }
    #[test]
    fn test_field_status() {
        let instance = GetWalletResponse::default();
        let _: CreateWalletResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tags() {
        let instance = GetWalletResponse::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletresponsesigningkey {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletResponseSigningKey::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletResponseSigningKey::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletResponseSigningKey::default();
        let b = GetWalletResponseSigningKey::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletResponseSigningKey::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletResponseSigningKey = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletResponseSigningKey::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletResponseSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletResponseSigningKey::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletResponseSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletResponseSigningKey::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletResponseSigningKey>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletResponseSigningKey>();
        let align = std::mem::align_of::<GetWalletResponseSigningKey>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletResponseSigningKey));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletResponseSigningKey::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletResponseSigningKey>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletResponseSigningKey>>();
        let type_size = std::mem::size_of::<GetWalletResponseSigningKey>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletResponseSigningKey),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletResponseSigningKey),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = GetWalletResponseSigningKey::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_public_key() {
        let instance = GetWalletResponseSigningKey::default();
        let _: String = instance.public_key;
    }
    #[test]
    fn test_field_scheme() {
        let instance = GetWalletResponseSigningKey::default();
        let _: Scheme = instance.scheme;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletResponseSigningKey::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletRequest::default();
        let b = GetWalletRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletRequest>();
        let align = std::mem::align_of::<GetWalletRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletRequest>>();
        let type_size = std::mem::size_of::<GetWalletRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletRequest),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletassetsparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletAssetsParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletAssetsParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletAssetsParams::default();
        let b = GetWalletAssetsParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletAssetsParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletAssetsParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletAssetsParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletAssetsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletAssetsParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletAssetsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletAssetsParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletAssetsParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletAssetsParams>();
        let align = std::mem::align_of::<GetWalletAssetsParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletAssetsParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletAssetsParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletAssetsParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletAssetsParams>>();
        let type_size = std::mem::size_of::<GetWalletAssetsParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletAssetsParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletAssetsParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletAssetsParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletAssetsParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletassetsquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletAssetsQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletAssetsQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletAssetsQuery::default();
        let b = GetWalletAssetsQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletAssetsQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletAssetsQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletAssetsQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletAssetsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletAssetsQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletAssetsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletAssetsQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletAssetsQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletAssetsQuery>();
        let align = std::mem::align_of::<GetWalletAssetsQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletAssetsQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletAssetsQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletAssetsQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletAssetsQuery>>();
        let type_size = std::mem::size_of::<GetWalletAssetsQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletAssetsQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletAssetsQuery),
            type_size
        );
    }
    #[test]
    fn test_field_net_worth() {
        let instance = GetWalletAssetsQuery::default();
        let _: Option<NetWorth> = instance.net_worth;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletAssetsQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_networth {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = NetWorth::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = NetWorth::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = NetWorth::default();
        let b = NetWorth::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = NetWorth::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: NetWorth = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = NetWorth::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: NetWorth =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = NetWorth::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: NetWorth =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = NetWorth::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<NetWorth>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<NetWorth>();
        let align = std::mem::align_of::<NetWorth>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(NetWorth));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = NetWorth::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<NetWorth>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<NetWorth>>();
        let type_size = std::mem::size_of::<NetWorth>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(NetWorth),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(NetWorth), type_size);
    }
}
#[cfg(test)]
mod test_getwalletassetsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletAssetsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletAssetsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletAssetsResponse::default();
        let b = GetWalletAssetsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletAssetsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletAssetsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletAssetsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletAssetsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletAssetsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletAssetsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletAssetsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletAssetsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletAssetsResponse>();
        let align = std::mem::align_of::<GetWalletAssetsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletAssetsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletAssetsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletAssetsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletAssetsResponse>>();
        let type_size = std::mem::size_of::<GetWalletAssetsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletAssetsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletAssetsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_assets() {
        let instance = GetWalletAssetsResponse::default();
        let _: Vec<AssetElement> = instance.assets;
    }
    #[test]
    fn test_field_network() {
        let instance = GetWalletAssetsResponse::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_net_worth() {
        let instance = GetWalletAssetsResponse::default();
        let _: Option<HashMap<String, f64>> = instance.net_worth;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletAssetsResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletAssetsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_assetelement {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = AssetElement::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = AssetElement::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = AssetElement::default();
        let b = AssetElement::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = AssetElement::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: AssetElement = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = AssetElement::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: AssetElement =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = AssetElement::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: AssetElement =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = AssetElement::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<AssetElement>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<AssetElement>();
        let align = std::mem::align_of::<AssetElement>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(AssetElement));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = AssetElement::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<AssetElement>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<AssetElement>>();
        let type_size = std::mem::size_of::<AssetElement>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(AssetElement),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(AssetElement), type_size);
    }
    #[test]
    fn test_field_balance() {
        let instance = AssetElement::default();
        let _: String = instance.balance;
    }
    #[test]
    fn test_field_decimals() {
        let instance = AssetElement::default();
        let _: f64 = instance.decimals;
    }
    #[test]
    fn test_field_kind() {
        let instance = AssetElement::default();
        let _: AssetKind = instance.kind;
    }
    #[test]
    fn test_field_quotes() {
        let instance = AssetElement::default();
        let _: Option<HashMap<String, f64>> = instance.quotes;
    }
    #[test]
    fn test_field_symbol() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.symbol;
    }
    #[test]
    fn test_field_verified() {
        let instance = AssetElement::default();
        let _: Option<bool> = instance.verified;
    }
    #[test]
    fn test_field_metadata() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.metadata;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_contract() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_token_id() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_mint() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = AssetElement::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = AssetElement::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_assetkind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = AssetKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = AssetKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = AssetKind::default();
        let b = AssetKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = AssetKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: AssetKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = AssetKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: AssetKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = AssetKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: AssetKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = AssetKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<AssetKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<AssetKind>();
        let align = std::mem::align_of::<AssetKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(AssetKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = AssetKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<AssetKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<AssetKind>>();
        let type_size = std::mem::size_of::<AssetKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(AssetKind),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(AssetKind), type_size);
    }
}
#[cfg(test)]
mod test_getwalletassetsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletAssetsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletAssetsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletAssetsRequest::default();
        let b = GetWalletAssetsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletAssetsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletAssetsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletAssetsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletAssetsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletAssetsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletAssetsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletAssetsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletAssetsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletAssetsRequest>();
        let align = std::mem::align_of::<GetWalletAssetsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletAssetsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletAssetsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletAssetsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletAssetsRequest>>();
        let type_size = std::mem::size_of::<GetWalletAssetsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletAssetsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletAssetsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = GetWalletAssetsRequest::default();
        let _: Option<GetWalletAssetsRequestQuery> = instance.query;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletAssetsRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletAssetsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletassetsrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletAssetsRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletAssetsRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletAssetsRequestQuery::default();
        let b = GetWalletAssetsRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletAssetsRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletAssetsRequestQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletAssetsRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletAssetsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletAssetsRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletAssetsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletAssetsRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletAssetsRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletAssetsRequestQuery>();
        let align = std::mem::align_of::<GetWalletAssetsRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletAssetsRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletAssetsRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletAssetsRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletAssetsRequestQuery>>();
        let type_size = std::mem::size_of::<GetWalletAssetsRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletAssetsRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletAssetsRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_net_worth() {
        let instance = GetWalletAssetsRequestQuery::default();
        let _: Option<NetWorth> = instance.net_worth;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletAssetsRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwallethistoryparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletHistoryParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletHistoryParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletHistoryParams::default();
        let b = GetWalletHistoryParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletHistoryParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletHistoryParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletHistoryParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletHistoryParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletHistoryParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletHistoryParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletHistoryParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletHistoryParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletHistoryParams>();
        let align = std::mem::align_of::<GetWalletHistoryParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletHistoryParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletHistoryParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletHistoryParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletHistoryParams>>();
        let type_size = std::mem::size_of::<GetWalletHistoryParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletHistoryParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletHistoryParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletHistoryParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletHistoryParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwallethistoryquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletHistoryQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletHistoryQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletHistoryQuery::default();
        let b = GetWalletHistoryQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletHistoryQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletHistoryQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletHistoryQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletHistoryQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletHistoryQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletHistoryQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletHistoryQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletHistoryQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletHistoryQuery>();
        let align = std::mem::align_of::<GetWalletHistoryQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletHistoryQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletHistoryQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletHistoryQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletHistoryQuery>>();
        let type_size = std::mem::size_of::<GetWalletHistoryQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletHistoryQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletHistoryQuery),
            type_size
        );
    }
    #[test]
    fn test_field_contract() {
        let instance = GetWalletHistoryQuery::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_direction() {
        let instance = GetWalletHistoryQuery::default();
        let _: Option<Direction> = instance.direction;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetWalletHistoryQuery::default();
        let _: Option<GetWalletHistoryQueryKind> = instance.kind;
    }
    #[test]
    fn test_field_limit() {
        let instance = GetWalletHistoryQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = GetWalletHistoryQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletHistoryQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_direction {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Direction::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Direction::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Direction::default();
        let b = Direction::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Direction::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Direction = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Direction::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Direction =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Direction::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Direction =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Direction::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Direction>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Direction>();
        let align = std::mem::align_of::<Direction>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Direction));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Direction::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Direction>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Direction>>();
        let type_size = std::mem::size_of::<Direction>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(Direction),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(Direction), type_size);
    }
}
#[cfg(test)]
mod test_getwallethistoryquerykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletHistoryQueryKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletHistoryQueryKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletHistoryQueryKind::default();
        let b = GetWalletHistoryQueryKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletHistoryQueryKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletHistoryQueryKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletHistoryQueryKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletHistoryQueryKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletHistoryQueryKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletHistoryQueryKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletHistoryQueryKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletHistoryQueryKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletHistoryQueryKind>();
        let align = std::mem::align_of::<GetWalletHistoryQueryKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletHistoryQueryKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletHistoryQueryKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletHistoryQueryKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletHistoryQueryKind>>();
        let type_size = std::mem::size_of::<GetWalletHistoryQueryKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletHistoryQueryKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletHistoryQueryKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_getwallethistoryresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletHistoryResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletHistoryResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletHistoryResponse::default();
        let b = GetWalletHistoryResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletHistoryResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletHistoryResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletHistoryResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletHistoryResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletHistoryResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletHistoryResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletHistoryResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletHistoryResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletHistoryResponse>();
        let align = std::mem::align_of::<GetWalletHistoryResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletHistoryResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletHistoryResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletHistoryResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletHistoryResponse>>();
        let type_size = std::mem::size_of::<GetWalletHistoryResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletHistoryResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletHistoryResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = GetWalletHistoryResponse::default();
        let _: Vec<GetWalletHistoryResponseItem> = instance.items;
    }
    #[test]
    fn test_field_network() {
        let instance = GetWalletHistoryResponse::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = GetWalletHistoryResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletHistoryResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletHistoryResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwallethistoryresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletHistoryResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletHistoryResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletHistoryResponseItem::default();
        let b = GetWalletHistoryResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletHistoryResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletHistoryResponseItem =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletHistoryResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletHistoryResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletHistoryResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletHistoryResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletHistoryResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletHistoryResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletHistoryResponseItem>();
        let align = std::mem::align_of::<GetWalletHistoryResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletHistoryResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletHistoryResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletHistoryResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletHistoryResponseItem>>();
        let type_size = std::mem::size_of::<GetWalletHistoryResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletHistoryResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletHistoryResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_balance_id() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.balance_id;
    }
    #[test]
    fn test_field_block_number() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: f64 = instance.block_number;
    }
    #[test]
    fn test_field_decimals() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<f64> = instance.decimals;
    }
    #[test]
    fn test_field_direction() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Direction = instance.direction;
    }
    #[test]
    fn test_field_fee() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_from() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.from;
    }
    #[test]
    fn test_field_index() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.index;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: GetWalletHistoryQueryKind = instance.kind;
    }
    #[test]
    fn test_field_liquidity_pool() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.liquidity_pool;
    }
    #[test]
    fn test_field_memo() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.memo;
    }
    #[test]
    fn test_field_metadata() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: PurpleMetadata = instance.metadata;
    }
    #[test]
    fn test_field_network() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_symbol() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.symbol;
    }
    #[test]
    fn test_field_timestamp() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: String = instance.timestamp;
    }
    #[test]
    fn test_field_to() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.to;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: String = instance.tx_hash;
    }
    #[test]
    fn test_field_value() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.value;
    }
    #[test]
    fn test_field_verified() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<bool> = instance.verified;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn test_field_metadata_address() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.metadata_address;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_clawback() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<bool> = instance.clawback;
    }
    #[test]
    fn test_field_opt_in() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<bool> = instance.opt_in;
    }
    #[test]
    fn test_field_opt_out() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<bool> = instance.opt_out;
    }
    #[test]
    fn test_field_contract() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn test_field_froms() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<Vec<String>> = instance.froms;
    }
    #[test]
    fn test_field_tos() {
        let instance = GetWalletHistoryResponseItem::default();
        let _: Option<Vec<String>> = instance.tos;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletHistoryResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_purplemetadata {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PurpleMetadata::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PurpleMetadata::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PurpleMetadata::default();
        let b = PurpleMetadata::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PurpleMetadata::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PurpleMetadata = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PurpleMetadata::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PurpleMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PurpleMetadata::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PurpleMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PurpleMetadata::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PurpleMetadata>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PurpleMetadata>();
        let align = std::mem::align_of::<PurpleMetadata>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PurpleMetadata));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PurpleMetadata::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PurpleMetadata>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PurpleMetadata>>();
        let type_size = std::mem::size_of::<PurpleMetadata>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PurpleMetadata),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(PurpleMetadata),
            type_size
        );
    }
    #[test]
    fn test_field_asset() {
        let instance = PurpleMetadata::default();
        let _: FluffyAsset = instance.asset;
    }
    #[test]
    fn test_field_fee() {
        let instance = PurpleMetadata::default();
        let _: Option<Fee> = instance.fee;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PurpleMetadata::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fluffyasset {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = FluffyAsset::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = FluffyAsset::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = FluffyAsset::default();
        let b = FluffyAsset::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = FluffyAsset::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: FluffyAsset = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = FluffyAsset::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FluffyAsset =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = FluffyAsset::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: FluffyAsset =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = FluffyAsset::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<FluffyAsset>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<FluffyAsset>();
        let align = std::mem::align_of::<FluffyAsset>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(FluffyAsset));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = FluffyAsset::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<FluffyAsset>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<FluffyAsset>>();
        let type_size = std::mem::size_of::<FluffyAsset>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(FluffyAsset),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(FluffyAsset), type_size);
    }
    #[test]
    fn test_field_decimals() {
        let instance = FluffyAsset::default();
        let _: Option<f64> = instance.decimals;
    }
    #[test]
    fn test_field_quotes() {
        let instance = FluffyAsset::default();
        let _: Option<HashMap<String, f64>> = instance.quotes;
    }
    #[test]
    fn test_field_symbol() {
        let instance = FluffyAsset::default();
        let _: Option<String> = instance.symbol;
    }
    #[test]
    fn test_field_verified() {
        let instance = FluffyAsset::default();
        let _: Option<bool> = instance.verified;
    }
    #[test]
    fn check_field_attributes() {
        let instance = FluffyAsset::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fee {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Fee::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Fee::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Fee::default();
        let b = Fee::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Fee::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Fee = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Fee::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Fee = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Fee::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Fee =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Fee::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Fee>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Fee>();
        let align = std::mem::align_of::<Fee>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Fee));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Fee::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Fee>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Fee>>();
        let type_size = std::mem::size_of::<Fee>();
        println!("Option<{}> size: {} bytes", stringify!(Fee), option_size);
        println!("Raw {} size: {} bytes", stringify!(Fee), type_size);
    }
    #[test]
    fn test_field_decimals() {
        let instance = Fee::default();
        let _: Option<f64> = instance.decimals;
    }
    #[test]
    fn test_field_quotes() {
        let instance = Fee::default();
        let _: Option<HashMap<String, f64>> = instance.quotes;
    }
    #[test]
    fn test_field_symbol() {
        let instance = Fee::default();
        let _: Option<String> = instance.symbol;
    }
    #[test]
    fn test_field_verified() {
        let instance = Fee::default();
        let _: Option<bool> = instance.verified;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Fee::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwallethistoryrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletHistoryRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletHistoryRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletHistoryRequest::default();
        let b = GetWalletHistoryRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletHistoryRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletHistoryRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletHistoryRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletHistoryRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletHistoryRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletHistoryRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletHistoryRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletHistoryRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletHistoryRequest>();
        let align = std::mem::align_of::<GetWalletHistoryRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletHistoryRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletHistoryRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletHistoryRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletHistoryRequest>>();
        let type_size = std::mem::size_of::<GetWalletHistoryRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletHistoryRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletHistoryRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = GetWalletHistoryRequest::default();
        let _: Option<GetWalletHistoryRequestQuery> = instance.query;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletHistoryRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletHistoryRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwallethistoryrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletHistoryRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletHistoryRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletHistoryRequestQuery::default();
        let b = GetWalletHistoryRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletHistoryRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletHistoryRequestQuery =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletHistoryRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletHistoryRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletHistoryRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletHistoryRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletHistoryRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletHistoryRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletHistoryRequestQuery>();
        let align = std::mem::align_of::<GetWalletHistoryRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletHistoryRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletHistoryRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletHistoryRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletHistoryRequestQuery>>();
        let type_size = std::mem::size_of::<GetWalletHistoryRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletHistoryRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletHistoryRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_contract() {
        let instance = GetWalletHistoryRequestQuery::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_direction() {
        let instance = GetWalletHistoryRequestQuery::default();
        let _: Option<Direction> = instance.direction;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetWalletHistoryRequestQuery::default();
        let _: Option<GetWalletHistoryQueryKind> = instance.kind;
    }
    #[test]
    fn test_field_limit() {
        let instance = GetWalletHistoryRequestQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = GetWalletHistoryRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletHistoryRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletnftsparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletNftsParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletNftsParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletNftsParams::default();
        let b = GetWalletNftsParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletNftsParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletNftsParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletNftsParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletNftsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletNftsParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletNftsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletNftsParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletNftsParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletNftsParams>();
        let align = std::mem::align_of::<GetWalletNftsParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletNftsParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletNftsParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletNftsParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletNftsParams>>();
        let type_size = std::mem::size_of::<GetWalletNftsParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletNftsParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletNftsParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletNftsParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletNftsParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwalletnftsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletNftsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletNftsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletNftsResponse::default();
        let b = GetWalletNftsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletNftsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletNftsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletNftsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletNftsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletNftsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletNftsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletNftsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletNftsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletNftsResponse>();
        let align = std::mem::align_of::<GetWalletNftsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletNftsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletNftsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletNftsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletNftsResponse>>();
        let type_size = std::mem::size_of::<GetWalletNftsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletNftsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletNftsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_network() {
        let instance = GetWalletNftsResponse::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_nfts() {
        let instance = GetWalletNftsResponse::default();
        let _: Vec<Nft> = instance.nfts;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletNftsResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletNftsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_nft {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Nft::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Nft::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Nft::default();
        let b = Nft::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Nft::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Nft = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Nft::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Nft = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Nft::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Nft =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Nft::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Nft>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Nft>();
        let align = std::mem::align_of::<Nft>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Nft));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Nft::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Nft>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Nft>>();
        let type_size = std::mem::size_of::<Nft>();
        println!("Option<{}> size: {} bytes", stringify!(Nft), option_size);
        println!("Raw {} size: {} bytes", stringify!(Nft), type_size);
    }
    #[test]
    fn test_field_asset_id() {
        let instance = Nft::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = Nft::default();
        let _: NftKind = instance.kind;
    }
    #[test]
    fn test_field_symbol() {
        let instance = Nft::default();
        let _: Option<String> = instance.symbol;
    }
    #[test]
    fn test_field_token_uri() {
        let instance = Nft::default();
        let _: Option<String> = instance.token_uri;
    }
    #[test]
    fn test_field_contract() {
        let instance = Nft::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = Nft::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Nft::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_nftkind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = NftKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = NftKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = NftKind::default();
        let b = NftKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = NftKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: NftKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = NftKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: NftKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = NftKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: NftKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = NftKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<NftKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<NftKind>();
        let align = std::mem::align_of::<NftKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(NftKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = NftKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<NftKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<NftKind>>();
        let type_size = std::mem::size_of::<NftKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(NftKind),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(NftKind), type_size);
    }
}
#[cfg(test)]
mod test_getwalletnftsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWalletNftsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWalletNftsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWalletNftsRequest::default();
        let b = GetWalletNftsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWalletNftsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWalletNftsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWalletNftsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWalletNftsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWalletNftsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWalletNftsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWalletNftsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWalletNftsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWalletNftsRequest>();
        let align = std::mem::align_of::<GetWalletNftsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWalletNftsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWalletNftsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWalletNftsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWalletNftsRequest>>();
        let type_size = std::mem::size_of::<GetWalletNftsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWalletNftsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWalletNftsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = GetWalletNftsRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWalletNftsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_importwalletbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ImportWalletBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ImportWalletBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ImportWalletBody::default();
        let b = ImportWalletBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ImportWalletBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ImportWalletBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ImportWalletBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ImportWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ImportWalletBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ImportWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ImportWalletBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ImportWalletBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ImportWalletBody>();
        let align = std::mem::align_of::<ImportWalletBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ImportWalletBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ImportWalletBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ImportWalletBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ImportWalletBody>>();
        let type_size = std::mem::size_of::<ImportWalletBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ImportWalletBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ImportWalletBody),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = ImportWalletBody::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_encrypted_key_shares() {
        let instance = ImportWalletBody::default();
        let _: Vec<ImportWalletBodyEncryptedKeyShare> = instance.encrypted_key_shares;
    }
    #[test]
    fn test_field_external_id() {
        let instance = ImportWalletBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_min_signers() {
        let instance = ImportWalletBody::default();
        let _: f64 = instance.min_signers;
    }
    #[test]
    fn test_field_name() {
        let instance = ImportWalletBody::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = ImportWalletBody::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_protocol() {
        let instance = ImportWalletBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ImportWalletBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_importwalletbodyencryptedkeyshare {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ImportWalletBodyEncryptedKeyShare::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ImportWalletBodyEncryptedKeyShare::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ImportWalletBodyEncryptedKeyShare::default();
        let b = ImportWalletBodyEncryptedKeyShare::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ImportWalletBodyEncryptedKeyShare::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ImportWalletBodyEncryptedKeyShare =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ImportWalletBodyEncryptedKeyShare::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ImportWalletBodyEncryptedKeyShare =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ImportWalletBodyEncryptedKeyShare::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ImportWalletBodyEncryptedKeyShare =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ImportWalletBodyEncryptedKeyShare::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ImportWalletBodyEncryptedKeyShare>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ImportWalletBodyEncryptedKeyShare>();
        let align = std::mem::align_of::<ImportWalletBodyEncryptedKeyShare>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ImportWalletBodyEncryptedKeyShare)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ImportWalletBodyEncryptedKeyShare::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ImportWalletBodyEncryptedKeyShare>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ImportWalletBodyEncryptedKeyShare>>();
        let type_size = std::mem::size_of::<ImportWalletBodyEncryptedKeyShare>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ImportWalletBodyEncryptedKeyShare),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ImportWalletBodyEncryptedKeyShare),
            type_size
        );
    }
    #[test]
    fn test_field_encrypted_key_share() {
        let instance = ImportWalletBodyEncryptedKeyShare::default();
        let _: String = instance.encrypted_key_share;
    }
    #[test]
    fn test_field_signer_id() {
        let instance = ImportWalletBodyEncryptedKeyShare::default();
        let _: String = instance.signer_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ImportWalletBodyEncryptedKeyShare::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_importwalletresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ImportWalletResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ImportWalletResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ImportWalletResponse::default();
        let b = ImportWalletResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ImportWalletResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ImportWalletResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ImportWalletResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ImportWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ImportWalletResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ImportWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ImportWalletResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ImportWalletResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ImportWalletResponse>();
        let align = std::mem::align_of::<ImportWalletResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ImportWalletResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ImportWalletResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ImportWalletResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ImportWalletResponse>>();
        let type_size = std::mem::size_of::<ImportWalletResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ImportWalletResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ImportWalletResponse),
            type_size
        );
    }
    #[test]
    fn test_field_address() {
        let instance = ImportWalletResponse::default();
        let _: Option<String> = instance.address;
    }
    #[test]
    fn test_field_custodial() {
        let instance = ImportWalletResponse::default();
        let _: bool = instance.custodial;
    }
    #[test]
    fn test_field_date_created() {
        let instance = ImportWalletResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_exported() {
        let instance = ImportWalletResponse::default();
        let _: Option<String> = instance.date_exported;
    }
    #[test]
    fn test_field_exported() {
        let instance = ImportWalletResponse::default();
        let _: Option<bool> = instance.exported;
    }
    #[test]
    fn test_field_external_id() {
        let instance = ImportWalletResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_id() {
        let instance = ImportWalletResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_imported() {
        let instance = ImportWalletResponse::default();
        let _: Option<bool> = instance.imported;
    }
    #[test]
    fn test_field_name() {
        let instance = ImportWalletResponse::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = ImportWalletResponse::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_signing_key() {
        let instance = ImportWalletResponse::default();
        let _: ImportWalletResponseSigningKey = instance.signing_key;
    }
    #[test]
    fn test_field_status() {
        let instance = ImportWalletResponse::default();
        let _: CreateWalletResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tags() {
        let instance = ImportWalletResponse::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ImportWalletResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_importwalletresponsesigningkey {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ImportWalletResponseSigningKey::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ImportWalletResponseSigningKey::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ImportWalletResponseSigningKey::default();
        let b = ImportWalletResponseSigningKey::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ImportWalletResponseSigningKey::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ImportWalletResponseSigningKey =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ImportWalletResponseSigningKey::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ImportWalletResponseSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ImportWalletResponseSigningKey::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ImportWalletResponseSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ImportWalletResponseSigningKey::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ImportWalletResponseSigningKey>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ImportWalletResponseSigningKey>();
        let align = std::mem::align_of::<ImportWalletResponseSigningKey>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ImportWalletResponseSigningKey)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ImportWalletResponseSigningKey::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ImportWalletResponseSigningKey>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ImportWalletResponseSigningKey>>();
        let type_size = std::mem::size_of::<ImportWalletResponseSigningKey>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ImportWalletResponseSigningKey),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ImportWalletResponseSigningKey),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = ImportWalletResponseSigningKey::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_public_key() {
        let instance = ImportWalletResponseSigningKey::default();
        let _: String = instance.public_key;
    }
    #[test]
    fn test_field_scheme() {
        let instance = ImportWalletResponseSigningKey::default();
        let _: Scheme = instance.scheme;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ImportWalletResponseSigningKey::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_importwalletrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ImportWalletRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ImportWalletRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ImportWalletRequest::default();
        let b = ImportWalletRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ImportWalletRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ImportWalletRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ImportWalletRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ImportWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ImportWalletRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ImportWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ImportWalletRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ImportWalletRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ImportWalletRequest>();
        let align = std::mem::align_of::<ImportWalletRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ImportWalletRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ImportWalletRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ImportWalletRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ImportWalletRequest>>();
        let type_size = std::mem::size_of::<ImportWalletRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ImportWalletRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ImportWalletRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = ImportWalletRequest::default();
        let _: ImportWalletRequestBody = instance.body;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ImportWalletRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_importwalletrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ImportWalletRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ImportWalletRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ImportWalletRequestBody::default();
        let b = ImportWalletRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ImportWalletRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ImportWalletRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ImportWalletRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ImportWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ImportWalletRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ImportWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ImportWalletRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ImportWalletRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ImportWalletRequestBody>();
        let align = std::mem::align_of::<ImportWalletRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ImportWalletRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ImportWalletRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ImportWalletRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ImportWalletRequestBody>>();
        let type_size = std::mem::size_of::<ImportWalletRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ImportWalletRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ImportWalletRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = ImportWalletRequestBody::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_encrypted_key_shares() {
        let instance = ImportWalletRequestBody::default();
        let _: Vec<BodyEncryptedKeyShare> = instance.encrypted_key_shares;
    }
    #[test]
    fn test_field_external_id() {
        let instance = ImportWalletRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_min_signers() {
        let instance = ImportWalletRequestBody::default();
        let _: f64 = instance.min_signers;
    }
    #[test]
    fn test_field_name() {
        let instance = ImportWalletRequestBody::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = ImportWalletRequestBody::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_protocol() {
        let instance = ImportWalletRequestBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ImportWalletRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_bodyencryptedkeyshare {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BodyEncryptedKeyShare::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BodyEncryptedKeyShare::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BodyEncryptedKeyShare::default();
        let b = BodyEncryptedKeyShare::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BodyEncryptedKeyShare::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BodyEncryptedKeyShare = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BodyEncryptedKeyShare::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BodyEncryptedKeyShare =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BodyEncryptedKeyShare::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BodyEncryptedKeyShare =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BodyEncryptedKeyShare::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BodyEncryptedKeyShare>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BodyEncryptedKeyShare>();
        let align = std::mem::align_of::<BodyEncryptedKeyShare>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BodyEncryptedKeyShare));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BodyEncryptedKeyShare::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BodyEncryptedKeyShare>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BodyEncryptedKeyShare>>();
        let type_size = std::mem::size_of::<BodyEncryptedKeyShare>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BodyEncryptedKeyShare),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BodyEncryptedKeyShare),
            type_size
        );
    }
    #[test]
    fn test_field_encrypted_key_share() {
        let instance = BodyEncryptedKeyShare::default();
        let _: String = instance.encrypted_key_share;
    }
    #[test]
    fn test_field_signer_id() {
        let instance = BodyEncryptedKeyShare::default();
        let _: String = instance.signer_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BodyEncryptedKeyShare::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listsignaturesparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListSignaturesParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListSignaturesParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListSignaturesParams::default();
        let b = ListSignaturesParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListSignaturesParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListSignaturesParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListSignaturesParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListSignaturesParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListSignaturesParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListSignaturesParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListSignaturesParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListSignaturesParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListSignaturesParams>();
        let align = std::mem::align_of::<ListSignaturesParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListSignaturesParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListSignaturesParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListSignaturesParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListSignaturesParams>>();
        let type_size = std::mem::size_of::<ListSignaturesParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListSignaturesParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListSignaturesParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListSignaturesParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListSignaturesParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listsignaturesquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListSignaturesQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListSignaturesQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListSignaturesQuery::default();
        let b = ListSignaturesQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListSignaturesQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListSignaturesQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListSignaturesQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListSignaturesQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListSignaturesQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListSignaturesQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListSignaturesQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListSignaturesQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListSignaturesQuery>();
        let align = std::mem::align_of::<ListSignaturesQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListSignaturesQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListSignaturesQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListSignaturesQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListSignaturesQuery>>();
        let type_size = std::mem::size_of::<ListSignaturesQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListSignaturesQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListSignaturesQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListSignaturesQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListSignaturesQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListSignaturesQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listsignaturesresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListSignaturesResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListSignaturesResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListSignaturesResponse::default();
        let b = ListSignaturesResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListSignaturesResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListSignaturesResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListSignaturesResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListSignaturesResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListSignaturesResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListSignaturesResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListSignaturesResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListSignaturesResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListSignaturesResponse>();
        let align = std::mem::align_of::<ListSignaturesResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListSignaturesResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListSignaturesResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListSignaturesResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListSignaturesResponse>>();
        let type_size = std::mem::size_of::<ListSignaturesResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListSignaturesResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListSignaturesResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListSignaturesResponse::default();
        let _: Vec<ListSignaturesResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListSignaturesResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListSignaturesResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListSignaturesResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listsignaturesresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListSignaturesResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListSignaturesResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListSignaturesResponseItem::default();
        let b = ListSignaturesResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListSignaturesResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListSignaturesResponseItem = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListSignaturesResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListSignaturesResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListSignaturesResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListSignaturesResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListSignaturesResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListSignaturesResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListSignaturesResponseItem>();
        let align = std::mem::align_of::<ListSignaturesResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListSignaturesResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListSignaturesResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListSignaturesResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListSignaturesResponseItem>>();
        let type_size = std::mem::size_of::<ListSignaturesResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListSignaturesResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListSignaturesResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = ListSignaturesResponseItem::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_date_signed() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.date_signed;
    }
    #[test]
    fn test_field_external_id() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = ListSignaturesResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_network() {
        let instance = ListSignaturesResponseItem::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = ListSignaturesResponseItem::default();
        let _: PurpleRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = ListSignaturesResponseItem::default();
        let _: PurpleRequester = instance.requester;
    }
    #[test]
    fn test_field_signature() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<IndigoSignature> = instance.signature;
    }
    #[test]
    fn test_field_signatures() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<Vec<IndecentSignature>> = instance.signatures;
    }
    #[test]
    fn test_field_signed_data() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.signed_data;
    }
    #[test]
    fn test_field_status() {
        let instance = ListSignaturesResponseItem::default();
        let _: GenerateSignatureResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = ListSignaturesResponseItem::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListSignaturesResponseItem::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListSignaturesResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_purplerequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PurpleRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PurpleRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PurpleRequestBody::default();
        let b = PurpleRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PurpleRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PurpleRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PurpleRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PurpleRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PurpleRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PurpleRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PurpleRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PurpleRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PurpleRequestBody>();
        let align = std::mem::align_of::<PurpleRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PurpleRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PurpleRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PurpleRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PurpleRequestBody>>();
        let type_size = std::mem::size_of::<PurpleRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PurpleRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(PurpleRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = PurpleRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = PurpleRequestBody::default();
        let _: GenerateSignatureBodyKind = instance.kind;
    }
    #[test]
    fn test_field_sign_doc() {
        let instance = PurpleRequestBody::default();
        let _: Option<String> = instance.sign_doc;
    }
    #[test]
    fn test_field_hash() {
        let instance = PurpleRequestBody::default();
        let _: Option<String> = instance.hash;
    }
    #[test]
    fn test_field_taproot_merkle_root() {
        let instance = PurpleRequestBody::default();
        let _: Option<String> = instance.taproot_merkle_root;
    }
    #[test]
    fn test_field_message() {
        let instance = PurpleRequestBody::default();
        let _: Option<Message> = instance.message;
    }
    #[test]
    fn test_field_transaction() {
        let instance = PurpleRequestBody::default();
        let _: Option<String> = instance.transaction;
    }
    #[test]
    fn test_field_domain() {
        let instance = PurpleRequestBody::default();
        let _: Option<TentacledDomain> = instance.domain;
    }
    #[test]
    fn test_field_types() {
        let instance = PurpleRequestBody::default();
        let _: Option<HashMap<String, Vec<TentacledType>>> = instance.types;
    }
    #[test]
    fn test_field_psbt() {
        let instance = PurpleRequestBody::default();
        let _: Option<String> = instance.psbt;
    }
    #[test]
    fn test_field_format() {
        let instance = PurpleRequestBody::default();
        let _: Option<Format> = instance.format;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PurpleRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tentacleddomain {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TentacledDomain::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TentacledDomain::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TentacledDomain::default();
        let b = TentacledDomain::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TentacledDomain::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TentacledDomain = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TentacledDomain::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TentacledDomain =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TentacledDomain::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TentacledDomain =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TentacledDomain::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TentacledDomain>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TentacledDomain>();
        let align = std::mem::align_of::<TentacledDomain>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TentacledDomain));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TentacledDomain::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TentacledDomain>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TentacledDomain>>();
        let type_size = std::mem::size_of::<TentacledDomain>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TentacledDomain),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TentacledDomain),
            type_size
        );
    }
    #[test]
    fn test_field_chain_id() {
        let instance = TentacledDomain::default();
        let _: Option<Nonce> = instance.chain_id;
    }
    #[test]
    fn test_field_name() {
        let instance = TentacledDomain::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_salt() {
        let instance = TentacledDomain::default();
        let _: Option<String> = instance.salt;
    }
    #[test]
    fn test_field_verifying_contract() {
        let instance = TentacledDomain::default();
        let _: Option<String> = instance.verifying_contract;
    }
    #[test]
    fn test_field_version() {
        let instance = TentacledDomain::default();
        let _: Option<String> = instance.version;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TentacledDomain::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tentacledtype {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TentacledType::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TentacledType::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TentacledType::default();
        let b = TentacledType::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TentacledType::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TentacledType = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TentacledType::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TentacledType =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TentacledType::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TentacledType =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TentacledType::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TentacledType>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TentacledType>();
        let align = std::mem::align_of::<TentacledType>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TentacledType));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TentacledType::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TentacledType>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TentacledType>>();
        let type_size = std::mem::size_of::<TentacledType>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TentacledType),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TentacledType),
            type_size
        );
    }
    #[test]
    fn test_field_name() {
        let instance = TentacledType::default();
        let _: String = instance.name;
    }
    #[test]
    fn test_field_type_type() {
        let instance = TentacledType::default();
        let _: String = instance.type_type;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TentacledType::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_purplerequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PurpleRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PurpleRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PurpleRequester::default();
        let b = PurpleRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PurpleRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PurpleRequester = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PurpleRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PurpleRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PurpleRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PurpleRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PurpleRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PurpleRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PurpleRequester>();
        let align = std::mem::align_of::<PurpleRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PurpleRequester));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PurpleRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PurpleRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PurpleRequester>>();
        let type_size = std::mem::size_of::<PurpleRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PurpleRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(PurpleRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = PurpleRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = PurpleRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = PurpleRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PurpleRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_indigosignature {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = IndigoSignature::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = IndigoSignature::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = IndigoSignature::default();
        let b = IndigoSignature::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = IndigoSignature::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: IndigoSignature = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = IndigoSignature::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: IndigoSignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = IndigoSignature::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: IndigoSignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = IndigoSignature::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<IndigoSignature>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<IndigoSignature>();
        let align = std::mem::align_of::<IndigoSignature>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(IndigoSignature));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = IndigoSignature::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<IndigoSignature>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<IndigoSignature>>();
        let type_size = std::mem::size_of::<IndigoSignature>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(IndigoSignature),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(IndigoSignature),
            type_size
        );
    }
    #[test]
    fn test_field_encoded() {
        let instance = IndigoSignature::default();
        let _: Option<String> = instance.encoded;
    }
    #[test]
    fn test_field_r() {
        let instance = IndigoSignature::default();
        let _: String = instance.r;
    }
    #[test]
    fn test_field_recid() {
        let instance = IndigoSignature::default();
        let _: Option<f64> = instance.recid;
    }
    #[test]
    fn test_field_s() {
        let instance = IndigoSignature::default();
        let _: String = instance.s;
    }
    #[test]
    fn check_field_attributes() {
        let instance = IndigoSignature::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_indecentsignature {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = IndecentSignature::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = IndecentSignature::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = IndecentSignature::default();
        let b = IndecentSignature::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = IndecentSignature::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: IndecentSignature = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = IndecentSignature::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: IndecentSignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = IndecentSignature::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: IndecentSignature =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = IndecentSignature::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<IndecentSignature>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<IndecentSignature>();
        let align = std::mem::align_of::<IndecentSignature>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(IndecentSignature));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = IndecentSignature::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<IndecentSignature>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<IndecentSignature>>();
        let type_size = std::mem::size_of::<IndecentSignature>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(IndecentSignature),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(IndecentSignature),
            type_size
        );
    }
    #[test]
    fn test_field_encoded() {
        let instance = IndecentSignature::default();
        let _: Option<String> = instance.encoded;
    }
    #[test]
    fn test_field_r() {
        let instance = IndecentSignature::default();
        let _: String = instance.r;
    }
    #[test]
    fn test_field_recid() {
        let instance = IndecentSignature::default();
        let _: Option<f64> = instance.recid;
    }
    #[test]
    fn test_field_s() {
        let instance = IndecentSignature::default();
        let _: String = instance.s;
    }
    #[test]
    fn check_field_attributes() {
        let instance = IndecentSignature::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listsignaturesrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListSignaturesRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListSignaturesRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListSignaturesRequest::default();
        let b = ListSignaturesRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListSignaturesRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListSignaturesRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListSignaturesRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListSignaturesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListSignaturesRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListSignaturesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListSignaturesRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListSignaturesRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListSignaturesRequest>();
        let align = std::mem::align_of::<ListSignaturesRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListSignaturesRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListSignaturesRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListSignaturesRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListSignaturesRequest>>();
        let type_size = std::mem::size_of::<ListSignaturesRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListSignaturesRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListSignaturesRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListSignaturesRequest::default();
        let _: Option<ListSignaturesRequestQuery> = instance.query;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListSignaturesRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListSignaturesRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listsignaturesrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListSignaturesRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListSignaturesRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListSignaturesRequestQuery::default();
        let b = ListSignaturesRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListSignaturesRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListSignaturesRequestQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListSignaturesRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListSignaturesRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListSignaturesRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListSignaturesRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListSignaturesRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListSignaturesRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListSignaturesRequestQuery>();
        let align = std::mem::align_of::<ListSignaturesRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListSignaturesRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListSignaturesRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListSignaturesRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListSignaturesRequestQuery>>();
        let type_size = std::mem::size_of::<ListSignaturesRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListSignaturesRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListSignaturesRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListSignaturesRequestQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListSignaturesRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListSignaturesRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransactionsparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransactionsParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransactionsParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransactionsParams::default();
        let b = ListTransactionsParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransactionsParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransactionsParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransactionsParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransactionsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransactionsParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransactionsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransactionsParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransactionsParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransactionsParams>();
        let align = std::mem::align_of::<ListTransactionsParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransactionsParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransactionsParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransactionsParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransactionsParams>>();
        let type_size = std::mem::size_of::<ListTransactionsParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransactionsParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransactionsParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListTransactionsParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransactionsParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransactionsquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransactionsQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransactionsQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransactionsQuery::default();
        let b = ListTransactionsQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransactionsQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransactionsQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransactionsQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransactionsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransactionsQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransactionsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransactionsQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransactionsQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransactionsQuery>();
        let align = std::mem::align_of::<ListTransactionsQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransactionsQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransactionsQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransactionsQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransactionsQuery>>();
        let type_size = std::mem::size_of::<ListTransactionsQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransactionsQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransactionsQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListTransactionsQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListTransactionsQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransactionsQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransactionsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransactionsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransactionsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransactionsResponse::default();
        let b = ListTransactionsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransactionsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransactionsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransactionsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransactionsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransactionsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransactionsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransactionsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransactionsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransactionsResponse>();
        let align = std::mem::align_of::<ListTransactionsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransactionsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransactionsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransactionsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransactionsResponse>>();
        let type_size = std::mem::size_of::<ListTransactionsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransactionsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransactionsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListTransactionsResponse::default();
        let _: Vec<ListTransactionsResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListTransactionsResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListTransactionsResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransactionsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransactionsresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransactionsResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransactionsResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransactionsResponseItem::default();
        let b = ListTransactionsResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransactionsResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransactionsResponseItem =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransactionsResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransactionsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransactionsResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransactionsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransactionsResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransactionsResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransactionsResponseItem>();
        let align = std::mem::align_of::<ListTransactionsResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransactionsResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransactionsResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransactionsResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransactionsResponseItem>>();
        let type_size = std::mem::size_of::<ListTransactionsResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransactionsResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransactionsResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = ListTransactionsResponseItem::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_broadcasted() {
        let instance = ListTransactionsResponseItem::default();
        let _: Option<String> = instance.date_broadcasted;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = ListTransactionsResponseItem::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = ListTransactionsResponseItem::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = ListTransactionsResponseItem::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_external_id() {
        let instance = ListTransactionsResponseItem::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = ListTransactionsResponseItem::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = ListTransactionsResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_network() {
        let instance = ListTransactionsResponseItem::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = ListTransactionsResponseItem::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = ListTransactionsResponseItem::default();
        let _: FluffyRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = ListTransactionsResponseItem::default();
        let _: FluffyRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = ListTransactionsResponseItem::default();
        let _: BroadcastTransactionResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = ListTransactionsResponseItem::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListTransactionsResponseItem::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransactionsResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fluffyrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = FluffyRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = FluffyRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = FluffyRequestBody::default();
        let b = FluffyRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = FluffyRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: FluffyRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = FluffyRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FluffyRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = FluffyRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: FluffyRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = FluffyRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<FluffyRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<FluffyRequestBody>();
        let align = std::mem::align_of::<FluffyRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(FluffyRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = FluffyRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<FluffyRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<FluffyRequestBody>>();
        let type_size = std::mem::size_of::<FluffyRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(FluffyRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(FluffyRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = FluffyRequestBody::default();
        let _: BroadcastTransactionBodyKind = instance.kind;
    }
    #[test]
    fn test_field_transaction() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.transaction;
    }
    #[test]
    fn test_field_data() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.data;
    }
    #[test]
    fn test_field_gas_limit() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.gas_limit;
    }
    #[test]
    fn test_field_nonce() {
        let instance = FluffyRequestBody::default();
        let _: Option<Nonce> = instance.nonce;
    }
    #[test]
    fn test_field_to() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.to;
    }
    #[test]
    fn test_field_value() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.value;
    }
    #[test]
    fn test_field_max_fee_per_gas() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.max_fee_per_gas;
    }
    #[test]
    fn test_field_max_priority_fee_per_gas() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.max_priority_fee_per_gas;
    }
    #[test]
    fn test_field_gas_price() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.gas_price;
    }
    #[test]
    fn test_field_psbt() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.psbt;
    }
    #[test]
    fn check_field_attributes() {
        let instance = FluffyRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fluffyrequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = FluffyRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = FluffyRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = FluffyRequester::default();
        let b = FluffyRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = FluffyRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: FluffyRequester = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = FluffyRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FluffyRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = FluffyRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: FluffyRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = FluffyRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<FluffyRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<FluffyRequester>();
        let align = std::mem::align_of::<FluffyRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(FluffyRequester));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = FluffyRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<FluffyRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<FluffyRequester>>();
        let type_size = std::mem::size_of::<FluffyRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(FluffyRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(FluffyRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = FluffyRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = FluffyRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = FluffyRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = FluffyRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransactionsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransactionsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransactionsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransactionsRequest::default();
        let b = ListTransactionsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransactionsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransactionsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransactionsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransactionsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransactionsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransactionsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransactionsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransactionsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransactionsRequest>();
        let align = std::mem::align_of::<ListTransactionsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransactionsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransactionsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransactionsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransactionsRequest>>();
        let type_size = std::mem::size_of::<ListTransactionsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransactionsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransactionsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListTransactionsRequest::default();
        let _: Option<ListTransactionsRequestQuery> = instance.query;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListTransactionsRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransactionsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransactionsrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransactionsRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransactionsRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransactionsRequestQuery::default();
        let b = ListTransactionsRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransactionsRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransactionsRequestQuery =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransactionsRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransactionsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransactionsRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransactionsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransactionsRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransactionsRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransactionsRequestQuery>();
        let align = std::mem::align_of::<ListTransactionsRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransactionsRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransactionsRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransactionsRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransactionsRequestQuery>>();
        let type_size = std::mem::size_of::<ListTransactionsRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransactionsRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransactionsRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListTransactionsRequestQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListTransactionsRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransactionsRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransfersparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransfersParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransfersParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransfersParams::default();
        let b = ListTransfersParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransfersParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransfersParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransfersParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransfersParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransfersParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransfersParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransfersParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransfersParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransfersParams>();
        let align = std::mem::align_of::<ListTransfersParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransfersParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransfersParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransfersParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransfersParams>>();
        let type_size = std::mem::size_of::<ListTransfersParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransfersParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransfersParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListTransfersParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransfersParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransfersquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransfersQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransfersQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransfersQuery::default();
        let b = ListTransfersQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransfersQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransfersQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransfersQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransfersQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransfersQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransfersQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransfersQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransfersQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransfersQuery>();
        let align = std::mem::align_of::<ListTransfersQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransfersQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransfersQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransfersQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransfersQuery>>();
        let type_size = std::mem::size_of::<ListTransfersQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransfersQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransfersQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListTransfersQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListTransfersQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransfersQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransfersresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransfersResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransfersResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransfersResponse::default();
        let b = ListTransfersResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransfersResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransfersResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransfersResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransfersResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransfersResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransfersResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransfersResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransfersResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransfersResponse>();
        let align = std::mem::align_of::<ListTransfersResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransfersResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransfersResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransfersResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransfersResponse>>();
        let type_size = std::mem::size_of::<ListTransfersResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransfersResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransfersResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListTransfersResponse::default();
        let _: Vec<ListTransfersResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListTransfersResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListTransfersResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransfersResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransfersresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransfersResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransfersResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransfersResponseItem::default();
        let b = ListTransfersResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransfersResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransfersResponseItem = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransfersResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransfersResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransfersResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransfersResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransfersResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransfersResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransfersResponseItem>();
        let align = std::mem::align_of::<ListTransfersResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransfersResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransfersResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransfersResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransfersResponseItem>>();
        let type_size = std::mem::size_of::<ListTransfersResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransfersResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransfersResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = ListTransfersResponseItem::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_broadcasted() {
        let instance = ListTransfersResponseItem::default();
        let _: Option<String> = instance.date_broadcasted;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = ListTransfersResponseItem::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = ListTransfersResponseItem::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = ListTransfersResponseItem::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_external_id() {
        let instance = ListTransfersResponseItem::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = ListTransfersResponseItem::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = ListTransfersResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_metadata() {
        let instance = ListTransfersResponseItem::default();
        let _: FluffyMetadata = instance.metadata;
    }
    #[test]
    fn test_field_network() {
        let instance = ListTransfersResponseItem::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = ListTransfersResponseItem::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = ListTransfersResponseItem::default();
        let _: TentacledRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = ListTransfersResponseItem::default();
        let _: TentacledRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = ListTransfersResponseItem::default();
        let _: BroadcastTransactionResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = ListTransfersResponseItem::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListTransfersResponseItem::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransfersResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fluffymetadata {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = FluffyMetadata::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = FluffyMetadata::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = FluffyMetadata::default();
        let b = FluffyMetadata::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = FluffyMetadata::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: FluffyMetadata = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = FluffyMetadata::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FluffyMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = FluffyMetadata::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: FluffyMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = FluffyMetadata::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<FluffyMetadata>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<FluffyMetadata>();
        let align = std::mem::align_of::<FluffyMetadata>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(FluffyMetadata));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = FluffyMetadata::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<FluffyMetadata>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<FluffyMetadata>>();
        let type_size = std::mem::size_of::<FluffyMetadata>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(FluffyMetadata),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(FluffyMetadata),
            type_size
        );
    }
    #[test]
    fn test_field_asset() {
        let instance = FluffyMetadata::default();
        let _: TentacledAsset = instance.asset;
    }
    #[test]
    fn check_field_attributes() {
        let instance = FluffyMetadata::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tentacledasset {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TentacledAsset::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TentacledAsset::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TentacledAsset::default();
        let b = TentacledAsset::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TentacledAsset::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TentacledAsset = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TentacledAsset::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TentacledAsset =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TentacledAsset::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TentacledAsset =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TentacledAsset::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TentacledAsset>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TentacledAsset>();
        let align = std::mem::align_of::<TentacledAsset>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TentacledAsset));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TentacledAsset::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TentacledAsset>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TentacledAsset>>();
        let type_size = std::mem::size_of::<TentacledAsset>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TentacledAsset),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TentacledAsset),
            type_size
        );
    }
    #[test]
    fn test_field_decimals() {
        let instance = TentacledAsset::default();
        let _: Option<f64> = instance.decimals;
    }
    #[test]
    fn test_field_quotes() {
        let instance = TentacledAsset::default();
        let _: Option<HashMap<String, f64>> = instance.quotes;
    }
    #[test]
    fn test_field_symbol() {
        let instance = TentacledAsset::default();
        let _: Option<String> = instance.symbol;
    }
    #[test]
    fn test_field_verified() {
        let instance = TentacledAsset::default();
        let _: Option<bool> = instance.verified;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TentacledAsset::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tentacledrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TentacledRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TentacledRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TentacledRequestBody::default();
        let b = TentacledRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TentacledRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TentacledRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TentacledRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TentacledRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TentacledRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TentacledRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TentacledRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TentacledRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TentacledRequestBody>();
        let align = std::mem::align_of::<TentacledRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TentacledRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TentacledRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TentacledRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TentacledRequestBody>>();
        let type_size = std::mem::size_of::<TentacledRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TentacledRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TentacledRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.amount;
    }
    #[test]
    fn test_field_create_destination_account() {
        let instance = TentacledRequestBody::default();
        let _: Option<bool> = instance.create_destination_account;
    }
    #[test]
    fn test_field_external_id() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = TentacledRequestBody::default();
        let _: TransferAssetBodyKind = instance.kind;
    }
    #[test]
    fn test_field_memo() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.memo;
    }
    #[test]
    fn test_field_priority() {
        let instance = TentacledRequestBody::default();
        let _: Option<Priority> = instance.priority;
    }
    #[test]
    fn test_field_to() {
        let instance = TentacledRequestBody::default();
        let _: String = instance.to;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_metadata() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.metadata;
    }
    #[test]
    fn test_field_contract() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = TentacledRequestBody::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TentacledRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tentacledrequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TentacledRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TentacledRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TentacledRequester::default();
        let b = TentacledRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TentacledRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TentacledRequester = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TentacledRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TentacledRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TentacledRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TentacledRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TentacledRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TentacledRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TentacledRequester>();
        let align = std::mem::align_of::<TentacledRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TentacledRequester));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TentacledRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TentacledRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TentacledRequester>>();
        let type_size = std::mem::size_of::<TentacledRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TentacledRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TentacledRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = TentacledRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = TentacledRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = TentacledRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TentacledRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransfersrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransfersRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransfersRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransfersRequest::default();
        let b = ListTransfersRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransfersRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransfersRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransfersRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransfersRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransfersRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransfersRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransfersRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransfersRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransfersRequest>();
        let align = std::mem::align_of::<ListTransfersRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransfersRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransfersRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransfersRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransfersRequest>>();
        let type_size = std::mem::size_of::<ListTransfersRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransfersRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransfersRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListTransfersRequest::default();
        let _: Option<ListTransfersRequestQuery> = instance.query;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListTransfersRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransfersRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listtransfersrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListTransfersRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListTransfersRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListTransfersRequestQuery::default();
        let b = ListTransfersRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListTransfersRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListTransfersRequestQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListTransfersRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListTransfersRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListTransfersRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListTransfersRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListTransfersRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListTransfersRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListTransfersRequestQuery>();
        let align = std::mem::align_of::<ListTransfersRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListTransfersRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListTransfersRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListTransfersRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListTransfersRequestQuery>>();
        let type_size = std::mem::size_of::<ListTransfersRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListTransfersRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListTransfersRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListTransfersRequestQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListTransfersRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListTransfersRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwalletsquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWalletsQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWalletsQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWalletsQuery::default();
        let b = ListWalletsQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWalletsQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWalletsQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWalletsQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWalletsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWalletsQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWalletsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWalletsQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWalletsQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWalletsQuery>();
        let align = std::mem::align_of::<ListWalletsQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWalletsQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWalletsQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWalletsQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWalletsQuery>>();
        let type_size = std::mem::size_of::<ListWalletsQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWalletsQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWalletsQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListWalletsQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_owner_id() {
        let instance = ListWalletsQuery::default();
        let _: Option<String> = instance.owner_id;
    }
    #[test]
    fn test_field_owner_username() {
        let instance = ListWalletsQuery::default();
        let _: Option<String> = instance.owner_username;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListWalletsQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWalletsQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwalletsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWalletsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWalletsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWalletsResponse::default();
        let b = ListWalletsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWalletsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWalletsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWalletsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWalletsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWalletsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWalletsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWalletsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWalletsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWalletsResponse>();
        let align = std::mem::align_of::<ListWalletsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWalletsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWalletsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWalletsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWalletsResponse>>();
        let type_size = std::mem::size_of::<ListWalletsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWalletsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWalletsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListWalletsResponse::default();
        let _: Vec<ListWalletsResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListWalletsResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWalletsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwalletsresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWalletsResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWalletsResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWalletsResponseItem::default();
        let b = ListWalletsResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWalletsResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWalletsResponseItem = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWalletsResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWalletsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWalletsResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWalletsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWalletsResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWalletsResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWalletsResponseItem>();
        let align = std::mem::align_of::<ListWalletsResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWalletsResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWalletsResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWalletsResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWalletsResponseItem>>();
        let type_size = std::mem::size_of::<ListWalletsResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWalletsResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWalletsResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_address() {
        let instance = ListWalletsResponseItem::default();
        let _: Option<String> = instance.address;
    }
    #[test]
    fn test_field_custodial() {
        let instance = ListWalletsResponseItem::default();
        let _: bool = instance.custodial;
    }
    #[test]
    fn test_field_date_created() {
        let instance = ListWalletsResponseItem::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_exported() {
        let instance = ListWalletsResponseItem::default();
        let _: Option<String> = instance.date_exported;
    }
    #[test]
    fn test_field_exported() {
        let instance = ListWalletsResponseItem::default();
        let _: Option<bool> = instance.exported;
    }
    #[test]
    fn test_field_external_id() {
        let instance = ListWalletsResponseItem::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_id() {
        let instance = ListWalletsResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_imported() {
        let instance = ListWalletsResponseItem::default();
        let _: Option<bool> = instance.imported;
    }
    #[test]
    fn test_field_name() {
        let instance = ListWalletsResponseItem::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = ListWalletsResponseItem::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_signing_key() {
        let instance = ListWalletsResponseItem::default();
        let _: ItemSigningKey = instance.signing_key;
    }
    #[test]
    fn test_field_status() {
        let instance = ListWalletsResponseItem::default();
        let _: CreateWalletResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tags() {
        let instance = ListWalletsResponseItem::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWalletsResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_itemsigningkey {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ItemSigningKey::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ItemSigningKey::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ItemSigningKey::default();
        let b = ItemSigningKey::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ItemSigningKey::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ItemSigningKey = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ItemSigningKey::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ItemSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ItemSigningKey::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ItemSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ItemSigningKey::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ItemSigningKey>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ItemSigningKey>();
        let align = std::mem::align_of::<ItemSigningKey>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ItemSigningKey));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ItemSigningKey::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ItemSigningKey>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ItemSigningKey>>();
        let type_size = std::mem::size_of::<ItemSigningKey>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ItemSigningKey),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ItemSigningKey),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = ItemSigningKey::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_public_key() {
        let instance = ItemSigningKey::default();
        let _: String = instance.public_key;
    }
    #[test]
    fn test_field_scheme() {
        let instance = ItemSigningKey::default();
        let _: Scheme = instance.scheme;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ItemSigningKey::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwalletsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWalletsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWalletsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWalletsRequest::default();
        let b = ListWalletsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWalletsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWalletsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWalletsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWalletsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWalletsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWalletsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWalletsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWalletsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWalletsRequest>();
        let align = std::mem::align_of::<ListWalletsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWalletsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWalletsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWalletsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWalletsRequest>>();
        let type_size = std::mem::size_of::<ListWalletsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWalletsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWalletsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListWalletsRequest::default();
        let _: Option<ListWalletsRequestQuery> = instance.query;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWalletsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwalletsrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWalletsRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWalletsRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWalletsRequestQuery::default();
        let b = ListWalletsRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWalletsRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWalletsRequestQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWalletsRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWalletsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWalletsRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWalletsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWalletsRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWalletsRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWalletsRequestQuery>();
        let align = std::mem::align_of::<ListWalletsRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWalletsRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWalletsRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWalletsRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWalletsRequestQuery>>();
        let type_size = std::mem::size_of::<ListWalletsRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWalletsRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWalletsRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListWalletsRequestQuery::default();
        let _: Option<String> = instance.limit;
    }
    #[test]
    fn test_field_owner_id() {
        let instance = ListWalletsRequestQuery::default();
        let _: Option<String> = instance.owner_id;
    }
    #[test]
    fn test_field_owner_username() {
        let instance = ListWalletsRequestQuery::default();
        let _: Option<String> = instance.owner_username;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListWalletsRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWalletsRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tagwalletbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TagWalletBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TagWalletBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TagWalletBody::default();
        let b = TagWalletBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TagWalletBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TagWalletBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TagWalletBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TagWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TagWalletBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TagWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TagWalletBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TagWalletBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TagWalletBody>();
        let align = std::mem::align_of::<TagWalletBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TagWalletBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TagWalletBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TagWalletBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TagWalletBody>>();
        let type_size = std::mem::size_of::<TagWalletBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TagWalletBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TagWalletBody),
            type_size
        );
    }
    #[test]
    fn test_field_tags() {
        let instance = TagWalletBody::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TagWalletBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tagwalletparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TagWalletParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TagWalletParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TagWalletParams::default();
        let b = TagWalletParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TagWalletParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TagWalletParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TagWalletParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TagWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TagWalletParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TagWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TagWalletParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TagWalletParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TagWalletParams>();
        let align = std::mem::align_of::<TagWalletParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TagWalletParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TagWalletParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TagWalletParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TagWalletParams>>();
        let type_size = std::mem::size_of::<TagWalletParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TagWalletParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TagWalletParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = TagWalletParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TagWalletParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tagwalletrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TagWalletRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TagWalletRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TagWalletRequest::default();
        let b = TagWalletRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TagWalletRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TagWalletRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TagWalletRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TagWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TagWalletRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TagWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TagWalletRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TagWalletRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TagWalletRequest>();
        let align = std::mem::align_of::<TagWalletRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TagWalletRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TagWalletRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TagWalletRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TagWalletRequest>>();
        let type_size = std::mem::size_of::<TagWalletRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TagWalletRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TagWalletRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = TagWalletRequest::default();
        let _: TagWalletRequestBody = instance.body;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = TagWalletRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TagWalletRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_tagwalletrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TagWalletRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TagWalletRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TagWalletRequestBody::default();
        let b = TagWalletRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TagWalletRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TagWalletRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TagWalletRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TagWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TagWalletRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TagWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TagWalletRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TagWalletRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TagWalletRequestBody>();
        let align = std::mem::align_of::<TagWalletRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TagWalletRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TagWalletRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TagWalletRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TagWalletRequestBody>>();
        let type_size = std::mem::size_of::<TagWalletRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TagWalletRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TagWalletRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_tags() {
        let instance = TagWalletRequestBody::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TagWalletRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_transferassetparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TransferAssetParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TransferAssetParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TransferAssetParams::default();
        let b = TransferAssetParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TransferAssetParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TransferAssetParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TransferAssetParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TransferAssetParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TransferAssetParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TransferAssetParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TransferAssetParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TransferAssetParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TransferAssetParams>();
        let align = std::mem::align_of::<TransferAssetParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TransferAssetParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TransferAssetParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TransferAssetParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TransferAssetParams>>();
        let type_size = std::mem::size_of::<TransferAssetParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TransferAssetParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TransferAssetParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = TransferAssetParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TransferAssetParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_transferassetresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TransferAssetResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TransferAssetResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TransferAssetResponse::default();
        let b = TransferAssetResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TransferAssetResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TransferAssetResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TransferAssetResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TransferAssetResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TransferAssetResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TransferAssetResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TransferAssetResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TransferAssetResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TransferAssetResponse>();
        let align = std::mem::align_of::<TransferAssetResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TransferAssetResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TransferAssetResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TransferAssetResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TransferAssetResponse>>();
        let type_size = std::mem::size_of::<TransferAssetResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TransferAssetResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TransferAssetResponse),
            type_size
        );
    }
    #[test]
    fn test_field_approval_id() {
        let instance = TransferAssetResponse::default();
        let _: Option<String> = instance.approval_id;
    }
    #[test]
    fn test_field_date_broadcasted() {
        let instance = TransferAssetResponse::default();
        let _: Option<String> = instance.date_broadcasted;
    }
    #[test]
    fn test_field_date_confirmed() {
        let instance = TransferAssetResponse::default();
        let _: Option<String> = instance.date_confirmed;
    }
    #[test]
    fn test_field_date_policy_resolved() {
        let instance = TransferAssetResponse::default();
        let _: Option<String> = instance.date_policy_resolved;
    }
    #[test]
    fn test_field_date_requested() {
        let instance = TransferAssetResponse::default();
        let _: String = instance.date_requested;
    }
    #[test]
    fn test_field_external_id() {
        let instance = TransferAssetResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_fee() {
        let instance = TransferAssetResponse::default();
        let _: Option<String> = instance.fee;
    }
    #[test]
    fn test_field_id() {
        let instance = TransferAssetResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_metadata() {
        let instance = TransferAssetResponse::default();
        let _: TransferAssetResponseMetadata = instance.metadata;
    }
    #[test]
    fn test_field_network() {
        let instance = TransferAssetResponse::default();
        let _: BroadcastTransactionResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_reason() {
        let instance = TransferAssetResponse::default();
        let _: Option<String> = instance.reason;
    }
    #[test]
    fn test_field_request_body() {
        let instance = TransferAssetResponse::default();
        let _: TransferAssetResponseRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = TransferAssetResponse::default();
        let _: TransferAssetResponseRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = TransferAssetResponse::default();
        let _: BroadcastTransactionResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tx_hash() {
        let instance = TransferAssetResponse::default();
        let _: Option<String> = instance.tx_hash;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = TransferAssetResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TransferAssetResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_transferassetresponsemetadata {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TransferAssetResponseMetadata::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TransferAssetResponseMetadata::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TransferAssetResponseMetadata::default();
        let b = TransferAssetResponseMetadata::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TransferAssetResponseMetadata::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TransferAssetResponseMetadata =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TransferAssetResponseMetadata::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TransferAssetResponseMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TransferAssetResponseMetadata::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TransferAssetResponseMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TransferAssetResponseMetadata::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TransferAssetResponseMetadata>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TransferAssetResponseMetadata>();
        let align = std::mem::align_of::<TransferAssetResponseMetadata>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(TransferAssetResponseMetadata)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TransferAssetResponseMetadata::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TransferAssetResponseMetadata>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TransferAssetResponseMetadata>>();
        let type_size = std::mem::size_of::<TransferAssetResponseMetadata>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TransferAssetResponseMetadata),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TransferAssetResponseMetadata),
            type_size
        );
    }
    #[test]
    fn test_field_asset() {
        let instance = TransferAssetResponseMetadata::default();
        let _: StickyAsset = instance.asset;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TransferAssetResponseMetadata::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_stickyasset {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = StickyAsset::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = StickyAsset::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = StickyAsset::default();
        let b = StickyAsset::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = StickyAsset::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: StickyAsset = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = StickyAsset::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: StickyAsset =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = StickyAsset::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: StickyAsset =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = StickyAsset::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<StickyAsset>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<StickyAsset>();
        let align = std::mem::align_of::<StickyAsset>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(StickyAsset));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = StickyAsset::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<StickyAsset>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<StickyAsset>>();
        let type_size = std::mem::size_of::<StickyAsset>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(StickyAsset),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(StickyAsset), type_size);
    }
    #[test]
    fn test_field_decimals() {
        let instance = StickyAsset::default();
        let _: Option<f64> = instance.decimals;
    }
    #[test]
    fn test_field_quotes() {
        let instance = StickyAsset::default();
        let _: Option<HashMap<String, f64>> = instance.quotes;
    }
    #[test]
    fn test_field_symbol() {
        let instance = StickyAsset::default();
        let _: Option<String> = instance.symbol;
    }
    #[test]
    fn test_field_verified() {
        let instance = StickyAsset::default();
        let _: Option<bool> = instance.verified;
    }
    #[test]
    fn check_field_attributes() {
        let instance = StickyAsset::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_transferassetresponserequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TransferAssetResponseRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TransferAssetResponseRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TransferAssetResponseRequestBody::default();
        let b = TransferAssetResponseRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TransferAssetResponseRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TransferAssetResponseRequestBody =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TransferAssetResponseRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TransferAssetResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TransferAssetResponseRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TransferAssetResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TransferAssetResponseRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TransferAssetResponseRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TransferAssetResponseRequestBody>();
        let align = std::mem::align_of::<TransferAssetResponseRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(TransferAssetResponseRequestBody)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TransferAssetResponseRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TransferAssetResponseRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TransferAssetResponseRequestBody>>();
        let type_size = std::mem::size_of::<TransferAssetResponseRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TransferAssetResponseRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TransferAssetResponseRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.amount;
    }
    #[test]
    fn test_field_create_destination_account() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<bool> = instance.create_destination_account;
    }
    #[test]
    fn test_field_external_id() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: TransferAssetBodyKind = instance.kind;
    }
    #[test]
    fn test_field_memo() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.memo;
    }
    #[test]
    fn test_field_priority() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<Priority> = instance.priority;
    }
    #[test]
    fn test_field_to() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: String = instance.to;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_metadata() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.metadata;
    }
    #[test]
    fn test_field_contract() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = TransferAssetResponseRequestBody::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TransferAssetResponseRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_transferassetresponserequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TransferAssetResponseRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TransferAssetResponseRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TransferAssetResponseRequester::default();
        let b = TransferAssetResponseRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TransferAssetResponseRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TransferAssetResponseRequester =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TransferAssetResponseRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TransferAssetResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TransferAssetResponseRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TransferAssetResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TransferAssetResponseRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TransferAssetResponseRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TransferAssetResponseRequester>();
        let align = std::mem::align_of::<TransferAssetResponseRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(TransferAssetResponseRequester)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TransferAssetResponseRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TransferAssetResponseRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TransferAssetResponseRequester>>();
        let type_size = std::mem::size_of::<TransferAssetResponseRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TransferAssetResponseRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TransferAssetResponseRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = TransferAssetResponseRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = TransferAssetResponseRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = TransferAssetResponseRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TransferAssetResponseRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_transferassetrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TransferAssetRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TransferAssetRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TransferAssetRequest::default();
        let b = TransferAssetRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TransferAssetRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TransferAssetRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TransferAssetRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TransferAssetRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TransferAssetRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TransferAssetRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TransferAssetRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TransferAssetRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TransferAssetRequest>();
        let align = std::mem::align_of::<TransferAssetRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TransferAssetRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TransferAssetRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TransferAssetRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TransferAssetRequest>>();
        let type_size = std::mem::size_of::<TransferAssetRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TransferAssetRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TransferAssetRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = TransferAssetRequest::default();
        let _: TransferAssetBody = instance.body;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = TransferAssetRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TransferAssetRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_transferassetbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = TransferAssetBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = TransferAssetBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = TransferAssetBody::default();
        let b = TransferAssetBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = TransferAssetBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: TransferAssetBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = TransferAssetBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: TransferAssetBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = TransferAssetBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: TransferAssetBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = TransferAssetBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<TransferAssetBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<TransferAssetBody>();
        let align = std::mem::align_of::<TransferAssetBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(TransferAssetBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = TransferAssetBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<TransferAssetBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<TransferAssetBody>>();
        let type_size = std::mem::size_of::<TransferAssetBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(TransferAssetBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(TransferAssetBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.amount;
    }
    #[test]
    fn test_field_create_destination_account() {
        let instance = TransferAssetBody::default();
        let _: Option<bool> = instance.create_destination_account;
    }
    #[test]
    fn test_field_external_id() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = TransferAssetBody::default();
        let _: TransferAssetBodyKind = instance.kind;
    }
    #[test]
    fn test_field_memo() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.memo;
    }
    #[test]
    fn test_field_priority() {
        let instance = TransferAssetBody::default();
        let _: Option<Priority> = instance.priority;
    }
    #[test]
    fn test_field_to() {
        let instance = TransferAssetBody::default();
        let _: String = instance.to;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_metadata() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.metadata;
    }
    #[test]
    fn test_field_contract() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = TransferAssetBody::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TransferAssetBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_untagwalletbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UntagWalletBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UntagWalletBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UntagWalletBody::default();
        let b = UntagWalletBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UntagWalletBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UntagWalletBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UntagWalletBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UntagWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UntagWalletBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UntagWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UntagWalletBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UntagWalletBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UntagWalletBody>();
        let align = std::mem::align_of::<UntagWalletBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UntagWalletBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UntagWalletBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UntagWalletBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UntagWalletBody>>();
        let type_size = std::mem::size_of::<UntagWalletBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UntagWalletBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UntagWalletBody),
            type_size
        );
    }
    #[test]
    fn test_field_tags() {
        let instance = UntagWalletBody::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UntagWalletBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_untagwalletparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UntagWalletParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UntagWalletParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UntagWalletParams::default();
        let b = UntagWalletParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UntagWalletParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UntagWalletParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UntagWalletParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UntagWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UntagWalletParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UntagWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UntagWalletParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UntagWalletParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UntagWalletParams>();
        let align = std::mem::align_of::<UntagWalletParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UntagWalletParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UntagWalletParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UntagWalletParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UntagWalletParams>>();
        let type_size = std::mem::size_of::<UntagWalletParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UntagWalletParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UntagWalletParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = UntagWalletParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UntagWalletParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_untagwalletrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UntagWalletRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UntagWalletRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UntagWalletRequest::default();
        let b = UntagWalletRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UntagWalletRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UntagWalletRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UntagWalletRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UntagWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UntagWalletRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UntagWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UntagWalletRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UntagWalletRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UntagWalletRequest>();
        let align = std::mem::align_of::<UntagWalletRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UntagWalletRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UntagWalletRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UntagWalletRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UntagWalletRequest>>();
        let type_size = std::mem::size_of::<UntagWalletRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UntagWalletRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UntagWalletRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = UntagWalletRequest::default();
        let _: UntagWalletRequestBody = instance.body;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = UntagWalletRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UntagWalletRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_untagwalletrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UntagWalletRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UntagWalletRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UntagWalletRequestBody::default();
        let b = UntagWalletRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UntagWalletRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UntagWalletRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UntagWalletRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UntagWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UntagWalletRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UntagWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UntagWalletRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UntagWalletRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UntagWalletRequestBody>();
        let align = std::mem::align_of::<UntagWalletRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UntagWalletRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UntagWalletRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UntagWalletRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UntagWalletRequestBody>>();
        let type_size = std::mem::size_of::<UntagWalletRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UntagWalletRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UntagWalletRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_tags() {
        let instance = UntagWalletRequestBody::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UntagWalletRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewalletbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWalletBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWalletBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWalletBody::default();
        let b = UpdateWalletBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWalletBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWalletBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWalletBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWalletBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWalletBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWalletBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWalletBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWalletBody>();
        let align = std::mem::align_of::<UpdateWalletBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWalletBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWalletBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWalletBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWalletBody>>();
        let type_size = std::mem::size_of::<UpdateWalletBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWalletBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWalletBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = UpdateWalletBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_name() {
        let instance = UpdateWalletBody::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWalletBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewalletparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWalletParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWalletParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWalletParams::default();
        let b = UpdateWalletParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWalletParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWalletParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWalletParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWalletParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWalletParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWalletParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWalletParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWalletParams>();
        let align = std::mem::align_of::<UpdateWalletParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWalletParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWalletParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWalletParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWalletParams>>();
        let type_size = std::mem::size_of::<UpdateWalletParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWalletParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWalletParams),
            type_size
        );
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = UpdateWalletParams::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWalletParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewalletresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWalletResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWalletResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWalletResponse::default();
        let b = UpdateWalletResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWalletResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWalletResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWalletResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWalletResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWalletResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWalletResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWalletResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWalletResponse>();
        let align = std::mem::align_of::<UpdateWalletResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWalletResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWalletResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWalletResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWalletResponse>>();
        let type_size = std::mem::size_of::<UpdateWalletResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWalletResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWalletResponse),
            type_size
        );
    }
    #[test]
    fn test_field_address() {
        let instance = UpdateWalletResponse::default();
        let _: Option<String> = instance.address;
    }
    #[test]
    fn test_field_custodial() {
        let instance = UpdateWalletResponse::default();
        let _: bool = instance.custodial;
    }
    #[test]
    fn test_field_date_created() {
        let instance = UpdateWalletResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_exported() {
        let instance = UpdateWalletResponse::default();
        let _: Option<String> = instance.date_exported;
    }
    #[test]
    fn test_field_exported() {
        let instance = UpdateWalletResponse::default();
        let _: Option<bool> = instance.exported;
    }
    #[test]
    fn test_field_external_id() {
        let instance = UpdateWalletResponse::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_id() {
        let instance = UpdateWalletResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_imported() {
        let instance = UpdateWalletResponse::default();
        let _: Option<bool> = instance.imported;
    }
    #[test]
    fn test_field_name() {
        let instance = UpdateWalletResponse::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_network() {
        let instance = UpdateWalletResponse::default();
        let _: CreateWalletBodyNetwork = instance.network;
    }
    #[test]
    fn test_field_signing_key() {
        let instance = UpdateWalletResponse::default();
        let _: UpdateWalletResponseSigningKey = instance.signing_key;
    }
    #[test]
    fn test_field_status() {
        let instance = UpdateWalletResponse::default();
        let _: CreateWalletResponseStatus = instance.status;
    }
    #[test]
    fn test_field_tags() {
        let instance = UpdateWalletResponse::default();
        let _: Vec<String> = instance.tags;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWalletResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewalletresponsesigningkey {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWalletResponseSigningKey::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWalletResponseSigningKey::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWalletResponseSigningKey::default();
        let b = UpdateWalletResponseSigningKey::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWalletResponseSigningKey::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWalletResponseSigningKey =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWalletResponseSigningKey::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWalletResponseSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWalletResponseSigningKey::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWalletResponseSigningKey =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWalletResponseSigningKey::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWalletResponseSigningKey>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWalletResponseSigningKey>();
        let align = std::mem::align_of::<UpdateWalletResponseSigningKey>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(UpdateWalletResponseSigningKey)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWalletResponseSigningKey::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWalletResponseSigningKey>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWalletResponseSigningKey>>();
        let type_size = std::mem::size_of::<UpdateWalletResponseSigningKey>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWalletResponseSigningKey),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWalletResponseSigningKey),
            type_size
        );
    }
    #[test]
    fn test_field_curve() {
        let instance = UpdateWalletResponseSigningKey::default();
        let _: Curve = instance.curve;
    }
    #[test]
    fn test_field_public_key() {
        let instance = UpdateWalletResponseSigningKey::default();
        let _: String = instance.public_key;
    }
    #[test]
    fn test_field_scheme() {
        let instance = UpdateWalletResponseSigningKey::default();
        let _: Scheme = instance.scheme;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWalletResponseSigningKey::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewalletrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWalletRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWalletRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWalletRequest::default();
        let b = UpdateWalletRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWalletRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWalletRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWalletRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWalletRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWalletRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWalletRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWalletRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWalletRequest>();
        let align = std::mem::align_of::<UpdateWalletRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWalletRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWalletRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWalletRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWalletRequest>>();
        let type_size = std::mem::size_of::<UpdateWalletRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWalletRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWalletRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = UpdateWalletRequest::default();
        let _: UpdateWalletRequestBody = instance.body;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = UpdateWalletRequest::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWalletRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewalletrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWalletRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWalletRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWalletRequestBody::default();
        let b = UpdateWalletRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWalletRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWalletRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWalletRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWalletRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWalletRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWalletRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWalletRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWalletRequestBody>();
        let align = std::mem::align_of::<UpdateWalletRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWalletRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWalletRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWalletRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWalletRequestBody>>();
        let type_size = std::mem::size_of::<UpdateWalletRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWalletRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWalletRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_external_id() {
        let instance = UpdateWalletRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_name() {
        let instance = UpdateWalletRequestBody::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWalletRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
