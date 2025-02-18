/// @dfns-sdk-rs/tests/unit/api/exchanges/types_test.rs

#[path = "../../../../src/api/exchanges/types.rs"]
mod parent;
use parent::{
    Body, BodyReadConfiguration, BodyWriteConfiguration, CreateDepositBody, CreateDepositBodyKind,
    CreateDepositParams, CreateDepositRequest, CreateDepositResponse, CreateDepositResponseKind,
    CreateDepositResponseRequestBody, CreateDepositResponseRequester, CreateExchangeBody,
    CreateExchangeBodyKind, CreateExchangeBodyReadConfiguration,
    CreateExchangeBodyWriteConfiguration, CreateExchangeRequest, CreateExchangeResponse,
    CreateWithdrawalBody, CreateWithdrawalParams, CreateWithdrawalRequest,
    CreateWithdrawalResponse, CreateWithdrawalResponseRequestBody,
    CreateWithdrawalResponseRequester, DeleteExchangeParams, DeleteExchangeRequest,
    DeleteExchangeResponse, GetExchangeParams, GetExchangeRequest, GetExchangeResponse,
    ListAccountAssetsParams, ListAccountAssetsQuery, ListAccountAssetsRequest,
    ListAccountAssetsRequestQuery, ListAccountAssetsResponse, ListAccountAssetsResponseItem,
    ListAccountsParams, ListAccountsQuery, ListAccountsRequest, ListAccountsRequestQuery,
    ListAccountsResponse, ListAccountsResponseItem, ListAssetWithdrawalNetworksParams,
    ListAssetWithdrawalNetworksRequest, ListAssetWithdrawalNetworksResponseElement,
    ListAssetWithdrawalNetworksResponseKind, ListExchangesQuery, ListExchangesRequest,
    ListExchangesRequestQuery, ListExchangesResponse, ListExchangesResponseItem, Network, Priority,
};
use serde_json;
use std::collections::HashMap;
use std::mem;
#[cfg(test)]
mod test_createdepositparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateDepositParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateDepositParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateDepositParams::default();
        let b = CreateDepositParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateDepositParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateDepositParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateDepositParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateDepositParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateDepositParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateDepositParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateDepositParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateDepositParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateDepositParams>();
        let align = std::mem::align_of::<CreateDepositParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateDepositParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateDepositParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateDepositParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateDepositParams>>();
        let type_size = std::mem::size_of::<CreateDepositParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateDepositParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateDepositParams),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = CreateDepositParams::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = CreateDepositParams::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateDepositParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createdepositresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateDepositResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateDepositResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateDepositResponse::default();
        let b = CreateDepositResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateDepositResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateDepositResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateDepositResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateDepositResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateDepositResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateDepositResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateDepositResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateDepositResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateDepositResponse>();
        let align = std::mem::align_of::<CreateDepositResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateDepositResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateDepositResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateDepositResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateDepositResponse>>();
        let type_size = std::mem::size_of::<CreateDepositResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateDepositResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateDepositResponse),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = CreateDepositResponse::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateDepositResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = CreateDepositResponse::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn test_field_exchange_reference() {
        let instance = CreateDepositResponse::default();
        let _: Option<String> = instance.exchange_reference;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateDepositResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateDepositResponse::default();
        let _: CreateDepositResponseKind = instance.kind;
    }
    #[test]
    fn test_field_request_body() {
        let instance = CreateDepositResponse::default();
        let _: CreateDepositResponseRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = CreateDepositResponse::default();
        let _: CreateDepositResponseRequester = instance.requester;
    }
    #[test]
    fn test_field_transfer_id() {
        let instance = CreateDepositResponse::default();
        let _: Option<String> = instance.transfer_id;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateDepositResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateDepositResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createdepositresponsekind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateDepositResponseKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateDepositResponseKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateDepositResponseKind::default();
        let b = CreateDepositResponseKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateDepositResponseKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateDepositResponseKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateDepositResponseKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateDepositResponseKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateDepositResponseKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateDepositResponseKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateDepositResponseKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateDepositResponseKind>();
        let align = std::mem::align_of::<CreateDepositResponseKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateDepositResponseKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateDepositResponseKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateDepositResponseKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateDepositResponseKind>>();
        let type_size = std::mem::size_of::<CreateDepositResponseKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateDepositResponseKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateDepositResponseKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_createdepositresponserequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateDepositResponseRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateDepositResponseRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateDepositResponseRequestBody::default();
        let b = CreateDepositResponseRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateDepositResponseRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateDepositResponseRequestBody =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateDepositResponseRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateDepositResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateDepositResponseRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateDepositResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateDepositResponseRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateDepositResponseRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateDepositResponseRequestBody>();
        let align = std::mem::align_of::<CreateDepositResponseRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateDepositResponseRequestBody)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateDepositResponseRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateDepositResponseRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateDepositResponseRequestBody>>();
        let type_size = std::mem::size_of::<CreateDepositResponseRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateDepositResponseRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateDepositResponseRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_create_destination_account() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<bool> = instance.create_destination_account;
    }
    #[test]
    fn test_field_external_id() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: CreateDepositBodyKind = instance.kind;
    }
    #[test]
    fn test_field_otp() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.otp;
    }
    #[test]
    fn test_field_priority() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<Priority> = instance.priority;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn test_field_contract() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = CreateDepositResponseRequestBody::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateDepositResponseRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createdepositbodykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateDepositBodyKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateDepositBodyKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateDepositBodyKind::default();
        let b = CreateDepositBodyKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateDepositBodyKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateDepositBodyKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateDepositBodyKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateDepositBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateDepositBodyKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateDepositBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateDepositBodyKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateDepositBodyKind>();
        let align = std::mem::align_of::<CreateDepositBodyKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateDepositBodyKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateDepositBodyKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateDepositBodyKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateDepositBodyKind>>();
        let type_size = std::mem::size_of::<CreateDepositBodyKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateDepositBodyKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateDepositBodyKind),
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
mod test_createdepositresponserequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateDepositResponseRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateDepositResponseRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateDepositResponseRequester::default();
        let b = CreateDepositResponseRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateDepositResponseRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateDepositResponseRequester =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateDepositResponseRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateDepositResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateDepositResponseRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateDepositResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateDepositResponseRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateDepositResponseRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateDepositResponseRequester>();
        let align = std::mem::align_of::<CreateDepositResponseRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateDepositResponseRequester)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateDepositResponseRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateDepositResponseRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateDepositResponseRequester>>();
        let type_size = std::mem::size_of::<CreateDepositResponseRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateDepositResponseRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateDepositResponseRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = CreateDepositResponseRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = CreateDepositResponseRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = CreateDepositResponseRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateDepositResponseRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createdepositrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateDepositRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateDepositRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateDepositRequest::default();
        let b = CreateDepositRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateDepositRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateDepositRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateDepositRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateDepositRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateDepositRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateDepositRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateDepositRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateDepositRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateDepositRequest>();
        let align = std::mem::align_of::<CreateDepositRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateDepositRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateDepositRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateDepositRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateDepositRequest>>();
        let type_size = std::mem::size_of::<CreateDepositRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateDepositRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateDepositRequest),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = CreateDepositRequest::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_body() {
        let instance = CreateDepositRequest::default();
        let _: CreateDepositBody = instance.body;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = CreateDepositRequest::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateDepositRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createdepositbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateDepositBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateDepositBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateDepositBody::default();
        let b = CreateDepositBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateDepositBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateDepositBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateDepositBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateDepositBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateDepositBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateDepositBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateDepositBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateDepositBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateDepositBody>();
        let align = std::mem::align_of::<CreateDepositBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateDepositBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateDepositBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateDepositBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateDepositBody>>();
        let type_size = std::mem::size_of::<CreateDepositBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateDepositBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateDepositBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = CreateDepositBody::default();
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_create_destination_account() {
        let instance = CreateDepositBody::default();
        let _: Option<bool> = instance.create_destination_account;
    }
    #[test]
    fn test_field_external_id() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateDepositBody::default();
        let _: CreateDepositBodyKind = instance.kind;
    }
    #[test]
    fn test_field_otp() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.otp;
    }
    #[test]
    fn test_field_priority() {
        let instance = CreateDepositBody::default();
        let _: Option<Priority> = instance.priority;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateDepositBody::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn test_field_contract() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = CreateDepositBody::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateDepositBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createexchangebody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateExchangeBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateExchangeBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateExchangeBody::default();
        let b = CreateExchangeBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateExchangeBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateExchangeBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateExchangeBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateExchangeBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateExchangeBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateExchangeBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateExchangeBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateExchangeBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateExchangeBody>();
        let align = std::mem::align_of::<CreateExchangeBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateExchangeBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateExchangeBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateExchangeBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateExchangeBody>>();
        let type_size = std::mem::size_of::<CreateExchangeBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateExchangeBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateExchangeBody),
            type_size
        );
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateExchangeBody::default();
        let _: CreateExchangeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_name() {
        let instance = CreateExchangeBody::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_read_configuration() {
        let instance = CreateExchangeBody::default();
        let _: CreateExchangeBodyReadConfiguration = instance.read_configuration;
    }
    #[test]
    fn test_field_write_configuration() {
        let instance = CreateExchangeBody::default();
        let _: CreateExchangeBodyWriteConfiguration = instance.write_configuration;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateExchangeBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createexchangebodykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateExchangeBodyKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateExchangeBodyKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateExchangeBodyKind::default();
        let b = CreateExchangeBodyKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateExchangeBodyKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateExchangeBodyKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateExchangeBodyKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateExchangeBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateExchangeBodyKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateExchangeBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateExchangeBodyKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateExchangeBodyKind>();
        let align = std::mem::align_of::<CreateExchangeBodyKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateExchangeBodyKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateExchangeBodyKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateExchangeBodyKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateExchangeBodyKind>>();
        let type_size = std::mem::size_of::<CreateExchangeBodyKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateExchangeBodyKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateExchangeBodyKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_createexchangebodyreadconfiguration {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateExchangeBodyReadConfiguration::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateExchangeBodyReadConfiguration::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateExchangeBodyReadConfiguration::default();
        let b = CreateExchangeBodyReadConfiguration::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateExchangeBodyReadConfiguration::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateExchangeBodyReadConfiguration =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateExchangeBodyReadConfiguration::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateExchangeBodyReadConfiguration =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateExchangeBodyReadConfiguration::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateExchangeBodyReadConfiguration =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateExchangeBodyReadConfiguration::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateExchangeBodyReadConfiguration>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateExchangeBodyReadConfiguration>();
        let align = std::mem::align_of::<CreateExchangeBodyReadConfiguration>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateExchangeBodyReadConfiguration)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateExchangeBodyReadConfiguration::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateExchangeBodyReadConfiguration>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateExchangeBodyReadConfiguration>>();
        let type_size = std::mem::size_of::<CreateExchangeBodyReadConfiguration>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateExchangeBodyReadConfiguration),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateExchangeBodyReadConfiguration),
            type_size
        );
    }
    #[test]
    fn test_field_otp() {
        let instance = CreateExchangeBodyReadConfiguration::default();
        let _: Option<String> = instance.otp;
    }
    #[test]
    fn test_field_password() {
        let instance = CreateExchangeBodyReadConfiguration::default();
        let _: Option<String> = instance.password;
    }
    #[test]
    fn test_field_private_api_key() {
        let instance = CreateExchangeBodyReadConfiguration::default();
        let _: String = instance.private_api_key;
    }
    #[test]
    fn test_field_public_api_key() {
        let instance = CreateExchangeBodyReadConfiguration::default();
        let _: String = instance.public_api_key;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateExchangeBodyReadConfiguration::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createexchangebodywriteconfiguration {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateExchangeBodyWriteConfiguration::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateExchangeBodyWriteConfiguration::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateExchangeBodyWriteConfiguration::default();
        let b = CreateExchangeBodyWriteConfiguration::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateExchangeBodyWriteConfiguration::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateExchangeBodyWriteConfiguration =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateExchangeBodyWriteConfiguration::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateExchangeBodyWriteConfiguration =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateExchangeBodyWriteConfiguration::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateExchangeBodyWriteConfiguration =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateExchangeBodyWriteConfiguration::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateExchangeBodyWriteConfiguration>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateExchangeBodyWriteConfiguration>();
        let align = std::mem::align_of::<CreateExchangeBodyWriteConfiguration>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateExchangeBodyWriteConfiguration)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateExchangeBodyWriteConfiguration::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateExchangeBodyWriteConfiguration>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateExchangeBodyWriteConfiguration>>();
        let type_size = std::mem::size_of::<CreateExchangeBodyWriteConfiguration>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateExchangeBodyWriteConfiguration),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateExchangeBodyWriteConfiguration),
            type_size
        );
    }
    #[test]
    fn test_field_otp() {
        let instance = CreateExchangeBodyWriteConfiguration::default();
        let _: Option<String> = instance.otp;
    }
    #[test]
    fn test_field_password() {
        let instance = CreateExchangeBodyWriteConfiguration::default();
        let _: Option<String> = instance.password;
    }
    #[test]
    fn test_field_private_api_key() {
        let instance = CreateExchangeBodyWriteConfiguration::default();
        let _: String = instance.private_api_key;
    }
    #[test]
    fn test_field_public_api_key() {
        let instance = CreateExchangeBodyWriteConfiguration::default();
        let _: String = instance.public_api_key;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateExchangeBodyWriteConfiguration::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createexchangeresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateExchangeResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateExchangeResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateExchangeResponse::default();
        let b = CreateExchangeResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateExchangeResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateExchangeResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateExchangeResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateExchangeResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateExchangeResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateExchangeResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateExchangeResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateExchangeResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateExchangeResponse>();
        let align = std::mem::align_of::<CreateExchangeResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateExchangeResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateExchangeResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateExchangeResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateExchangeResponse>>();
        let type_size = std::mem::size_of::<CreateExchangeResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateExchangeResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateExchangeResponse),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateExchangeResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateExchangeResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateExchangeResponse::default();
        let _: CreateExchangeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_name() {
        let instance = CreateExchangeResponse::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateExchangeResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createexchangerequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateExchangeRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateExchangeRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateExchangeRequest::default();
        let b = CreateExchangeRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateExchangeRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateExchangeRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateExchangeRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateExchangeRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateExchangeRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateExchangeRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateExchangeRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateExchangeRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateExchangeRequest>();
        let align = std::mem::align_of::<CreateExchangeRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateExchangeRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateExchangeRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateExchangeRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateExchangeRequest>>();
        let type_size = std::mem::size_of::<CreateExchangeRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateExchangeRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateExchangeRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = CreateExchangeRequest::default();
        let _: Body = instance.body;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateExchangeRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_body {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Body::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Body::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Body::default();
        let b = Body::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Body::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Body = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Body::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Body = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Body::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Body =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Body::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Body>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Body>();
        let align = std::mem::align_of::<Body>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Body));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Body::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Body>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Body>>();
        let type_size = std::mem::size_of::<Body>();
        println!("Option<{}> size: {} bytes", stringify!(Body), option_size);
        println!("Raw {} size: {} bytes", stringify!(Body), type_size);
    }
    #[test]
    fn test_field_kind() {
        let instance = Body::default();
        let _: CreateExchangeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_name() {
        let instance = Body::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn test_field_read_configuration() {
        let instance = Body::default();
        let _: BodyReadConfiguration = instance.read_configuration;
    }
    #[test]
    fn test_field_write_configuration() {
        let instance = Body::default();
        let _: BodyWriteConfiguration = instance.write_configuration;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Body::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_bodyreadconfiguration {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BodyReadConfiguration::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BodyReadConfiguration::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BodyReadConfiguration::default();
        let b = BodyReadConfiguration::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BodyReadConfiguration::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BodyReadConfiguration = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BodyReadConfiguration::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BodyReadConfiguration =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BodyReadConfiguration::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BodyReadConfiguration =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BodyReadConfiguration::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BodyReadConfiguration>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BodyReadConfiguration>();
        let align = std::mem::align_of::<BodyReadConfiguration>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BodyReadConfiguration));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BodyReadConfiguration::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BodyReadConfiguration>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BodyReadConfiguration>>();
        let type_size = std::mem::size_of::<BodyReadConfiguration>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BodyReadConfiguration),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BodyReadConfiguration),
            type_size
        );
    }
    #[test]
    fn test_field_otp() {
        let instance = BodyReadConfiguration::default();
        let _: Option<String> = instance.otp;
    }
    #[test]
    fn test_field_password() {
        let instance = BodyReadConfiguration::default();
        let _: Option<String> = instance.password;
    }
    #[test]
    fn test_field_private_api_key() {
        let instance = BodyReadConfiguration::default();
        let _: String = instance.private_api_key;
    }
    #[test]
    fn test_field_public_api_key() {
        let instance = BodyReadConfiguration::default();
        let _: String = instance.public_api_key;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BodyReadConfiguration::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_bodywriteconfiguration {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = BodyWriteConfiguration::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = BodyWriteConfiguration::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = BodyWriteConfiguration::default();
        let b = BodyWriteConfiguration::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = BodyWriteConfiguration::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: BodyWriteConfiguration = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = BodyWriteConfiguration::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: BodyWriteConfiguration =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = BodyWriteConfiguration::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: BodyWriteConfiguration =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = BodyWriteConfiguration::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<BodyWriteConfiguration>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<BodyWriteConfiguration>();
        let align = std::mem::align_of::<BodyWriteConfiguration>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(BodyWriteConfiguration));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = BodyWriteConfiguration::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<BodyWriteConfiguration>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<BodyWriteConfiguration>>();
        let type_size = std::mem::size_of::<BodyWriteConfiguration>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(BodyWriteConfiguration),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(BodyWriteConfiguration),
            type_size
        );
    }
    #[test]
    fn test_field_otp() {
        let instance = BodyWriteConfiguration::default();
        let _: Option<String> = instance.otp;
    }
    #[test]
    fn test_field_password() {
        let instance = BodyWriteConfiguration::default();
        let _: Option<String> = instance.password;
    }
    #[test]
    fn test_field_private_api_key() {
        let instance = BodyWriteConfiguration::default();
        let _: String = instance.private_api_key;
    }
    #[test]
    fn test_field_public_api_key() {
        let instance = BodyWriteConfiguration::default();
        let _: String = instance.public_api_key;
    }
    #[test]
    fn check_field_attributes() {
        let instance = BodyWriteConfiguration::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwithdrawalparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWithdrawalParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWithdrawalParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWithdrawalParams::default();
        let b = CreateWithdrawalParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWithdrawalParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWithdrawalParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWithdrawalParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWithdrawalParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWithdrawalParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWithdrawalParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWithdrawalParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWithdrawalParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWithdrawalParams>();
        let align = std::mem::align_of::<CreateWithdrawalParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWithdrawalParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWithdrawalParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWithdrawalParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWithdrawalParams>>();
        let type_size = std::mem::size_of::<CreateWithdrawalParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWithdrawalParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWithdrawalParams),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = CreateWithdrawalParams::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = CreateWithdrawalParams::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWithdrawalParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwithdrawalresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWithdrawalResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWithdrawalResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWithdrawalResponse::default();
        let b = CreateWithdrawalResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWithdrawalResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWithdrawalResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWithdrawalResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWithdrawalResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWithdrawalResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWithdrawalResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWithdrawalResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWithdrawalResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWithdrawalResponse>();
        let align = std::mem::align_of::<CreateWithdrawalResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWithdrawalResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWithdrawalResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWithdrawalResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWithdrawalResponse>>();
        let type_size = std::mem::size_of::<CreateWithdrawalResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWithdrawalResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWithdrawalResponse),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = CreateWithdrawalResponse::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateWithdrawalResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = CreateWithdrawalResponse::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn test_field_exchange_reference() {
        let instance = CreateWithdrawalResponse::default();
        let _: Option<String> = instance.exchange_reference;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateWithdrawalResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateWithdrawalResponse::default();
        let _: CreateDepositResponseKind = instance.kind;
    }
    #[test]
    fn test_field_request_body() {
        let instance = CreateWithdrawalResponse::default();
        let _: CreateWithdrawalResponseRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = CreateWithdrawalResponse::default();
        let _: CreateWithdrawalResponseRequester = instance.requester;
    }
    #[test]
    fn test_field_transfer_id() {
        let instance = CreateWithdrawalResponse::default();
        let _: Option<String> = instance.transfer_id;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateWithdrawalResponse::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWithdrawalResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwithdrawalresponserequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWithdrawalResponseRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWithdrawalResponseRequestBody::default();
        let b = CreateWithdrawalResponseRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWithdrawalResponseRequestBody =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWithdrawalResponseRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWithdrawalResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWithdrawalResponseRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWithdrawalResponseRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWithdrawalResponseRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWithdrawalResponseRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWithdrawalResponseRequestBody>();
        let align = std::mem::align_of::<CreateWithdrawalResponseRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateWithdrawalResponseRequestBody)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWithdrawalResponseRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWithdrawalResponseRequestBody>>();
        let type_size = std::mem::size_of::<CreateWithdrawalResponseRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWithdrawalResponseRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWithdrawalResponseRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_create_destination_account() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<bool> = instance.create_destination_account;
    }
    #[test]
    fn test_field_external_id() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: CreateDepositBodyKind = instance.kind;
    }
    #[test]
    fn test_field_otp() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.otp;
    }
    #[test]
    fn test_field_priority() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<Priority> = instance.priority;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn test_field_contract() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWithdrawalResponseRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwithdrawalresponserequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWithdrawalResponseRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWithdrawalResponseRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWithdrawalResponseRequester::default();
        let b = CreateWithdrawalResponseRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWithdrawalResponseRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWithdrawalResponseRequester =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWithdrawalResponseRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWithdrawalResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWithdrawalResponseRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWithdrawalResponseRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWithdrawalResponseRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWithdrawalResponseRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWithdrawalResponseRequester>();
        let align = std::mem::align_of::<CreateWithdrawalResponseRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateWithdrawalResponseRequester)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWithdrawalResponseRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWithdrawalResponseRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWithdrawalResponseRequester>>();
        let type_size = std::mem::size_of::<CreateWithdrawalResponseRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWithdrawalResponseRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWithdrawalResponseRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = CreateWithdrawalResponseRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = CreateWithdrawalResponseRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = CreateWithdrawalResponseRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWithdrawalResponseRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwithdrawalrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWithdrawalRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWithdrawalRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWithdrawalRequest::default();
        let b = CreateWithdrawalRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWithdrawalRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWithdrawalRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWithdrawalRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWithdrawalRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWithdrawalRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWithdrawalRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWithdrawalRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWithdrawalRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWithdrawalRequest>();
        let align = std::mem::align_of::<CreateWithdrawalRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWithdrawalRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWithdrawalRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWithdrawalRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWithdrawalRequest>>();
        let type_size = std::mem::size_of::<CreateWithdrawalRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWithdrawalRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWithdrawalRequest),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = CreateWithdrawalRequest::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_body() {
        let instance = CreateWithdrawalRequest::default();
        let _: CreateWithdrawalBody = instance.body;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = CreateWithdrawalRequest::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWithdrawalRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwithdrawalbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWithdrawalBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWithdrawalBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWithdrawalBody::default();
        let b = CreateWithdrawalBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWithdrawalBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWithdrawalBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWithdrawalBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWithdrawalBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWithdrawalBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWithdrawalBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWithdrawalBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWithdrawalBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWithdrawalBody>();
        let align = std::mem::align_of::<CreateWithdrawalBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWithdrawalBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWithdrawalBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWithdrawalBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWithdrawalBody>>();
        let type_size = std::mem::size_of::<CreateWithdrawalBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWithdrawalBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWithdrawalBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = CreateWithdrawalBody::default();
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_create_destination_account() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<bool> = instance.create_destination_account;
    }
    #[test]
    fn test_field_external_id() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.external_id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateWithdrawalBody::default();
        let _: CreateDepositBodyKind = instance.kind;
    }
    #[test]
    fn test_field_otp() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.otp;
    }
    #[test]
    fn test_field_priority() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<Priority> = instance.priority;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateWithdrawalBody::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn test_field_contract() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_token_id() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_mint() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = CreateWithdrawalBody::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWithdrawalBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_deleteexchangeparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DeleteExchangeParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DeleteExchangeParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DeleteExchangeParams::default();
        let b = DeleteExchangeParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DeleteExchangeParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DeleteExchangeParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DeleteExchangeParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DeleteExchangeParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DeleteExchangeParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DeleteExchangeParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DeleteExchangeParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DeleteExchangeParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DeleteExchangeParams>();
        let align = std::mem::align_of::<DeleteExchangeParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DeleteExchangeParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DeleteExchangeParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DeleteExchangeParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DeleteExchangeParams>>();
        let type_size = std::mem::size_of::<DeleteExchangeParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DeleteExchangeParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DeleteExchangeParams),
            type_size
        );
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = DeleteExchangeParams::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DeleteExchangeParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_deleteexchangeresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DeleteExchangeResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DeleteExchangeResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DeleteExchangeResponse::default();
        let b = DeleteExchangeResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DeleteExchangeResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DeleteExchangeResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DeleteExchangeResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DeleteExchangeResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DeleteExchangeResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DeleteExchangeResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DeleteExchangeResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DeleteExchangeResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DeleteExchangeResponse>();
        let align = std::mem::align_of::<DeleteExchangeResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DeleteExchangeResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DeleteExchangeResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DeleteExchangeResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DeleteExchangeResponse>>();
        let type_size = std::mem::size_of::<DeleteExchangeResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DeleteExchangeResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DeleteExchangeResponse),
            type_size
        );
    }
    #[test]
    fn test_field_deleted() {
        let instance = DeleteExchangeResponse::default();
        let _: bool = instance.deleted;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DeleteExchangeResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_deleteexchangerequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DeleteExchangeRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DeleteExchangeRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DeleteExchangeRequest::default();
        let b = DeleteExchangeRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DeleteExchangeRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DeleteExchangeRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DeleteExchangeRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DeleteExchangeRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DeleteExchangeRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DeleteExchangeRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DeleteExchangeRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DeleteExchangeRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DeleteExchangeRequest>();
        let align = std::mem::align_of::<DeleteExchangeRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DeleteExchangeRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DeleteExchangeRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DeleteExchangeRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DeleteExchangeRequest>>();
        let type_size = std::mem::size_of::<DeleteExchangeRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DeleteExchangeRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DeleteExchangeRequest),
            type_size
        );
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = DeleteExchangeRequest::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DeleteExchangeRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getexchangeparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetExchangeParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetExchangeParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetExchangeParams::default();
        let b = GetExchangeParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetExchangeParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetExchangeParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetExchangeParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetExchangeParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetExchangeParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetExchangeParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetExchangeParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetExchangeParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetExchangeParams>();
        let align = std::mem::align_of::<GetExchangeParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetExchangeParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetExchangeParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetExchangeParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetExchangeParams>>();
        let type_size = std::mem::size_of::<GetExchangeParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetExchangeParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetExchangeParams),
            type_size
        );
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = GetExchangeParams::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetExchangeParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getexchangeresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetExchangeResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetExchangeResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetExchangeResponse::default();
        let b = GetExchangeResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetExchangeResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetExchangeResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetExchangeResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetExchangeResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetExchangeResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetExchangeResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetExchangeResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetExchangeResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetExchangeResponse>();
        let align = std::mem::align_of::<GetExchangeResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetExchangeResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetExchangeResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetExchangeResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetExchangeResponse>>();
        let type_size = std::mem::size_of::<GetExchangeResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetExchangeResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetExchangeResponse),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = GetExchangeResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = GetExchangeResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetExchangeResponse::default();
        let _: CreateExchangeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_name() {
        let instance = GetExchangeResponse::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetExchangeResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getexchangerequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetExchangeRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetExchangeRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetExchangeRequest::default();
        let b = GetExchangeRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetExchangeRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetExchangeRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetExchangeRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetExchangeRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetExchangeRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetExchangeRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetExchangeRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetExchangeRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetExchangeRequest>();
        let align = std::mem::align_of::<GetExchangeRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetExchangeRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetExchangeRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetExchangeRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetExchangeRequest>>();
        let type_size = std::mem::size_of::<GetExchangeRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetExchangeRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetExchangeRequest),
            type_size
        );
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = GetExchangeRequest::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetExchangeRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountassetsparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountAssetsParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountAssetsParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountAssetsParams::default();
        let b = ListAccountAssetsParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountAssetsParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountAssetsParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountAssetsParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountAssetsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountAssetsParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountAssetsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountAssetsParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountAssetsParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountAssetsParams>();
        let align = std::mem::align_of::<ListAccountAssetsParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountAssetsParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountAssetsParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountAssetsParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountAssetsParams>>();
        let type_size = std::mem::size_of::<ListAccountAssetsParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountAssetsParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountAssetsParams),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = ListAccountAssetsParams::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = ListAccountAssetsParams::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountAssetsParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountassetsquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountAssetsQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountAssetsQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountAssetsQuery::default();
        let b = ListAccountAssetsQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountAssetsQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountAssetsQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountAssetsQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountAssetsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountAssetsQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountAssetsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountAssetsQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountAssetsQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountAssetsQuery>();
        let align = std::mem::align_of::<ListAccountAssetsQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountAssetsQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountAssetsQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountAssetsQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountAssetsQuery>>();
        let type_size = std::mem::size_of::<ListAccountAssetsQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountAssetsQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountAssetsQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListAccountAssetsQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListAccountAssetsQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountAssetsQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountassetsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountAssetsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountAssetsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountAssetsResponse::default();
        let b = ListAccountAssetsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountAssetsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountAssetsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountAssetsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountAssetsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountAssetsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountAssetsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountAssetsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountAssetsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountAssetsResponse>();
        let align = std::mem::align_of::<ListAccountAssetsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountAssetsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountAssetsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountAssetsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountAssetsResponse>>();
        let type_size = std::mem::size_of::<ListAccountAssetsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountAssetsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountAssetsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListAccountAssetsResponse::default();
        let _: Vec<ListAccountAssetsResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListAccountAssetsResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountAssetsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountassetsresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountAssetsResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountAssetsResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountAssetsResponseItem::default();
        let b = ListAccountAssetsResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountAssetsResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountAssetsResponseItem =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountAssetsResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountAssetsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountAssetsResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountAssetsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountAssetsResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountAssetsResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountAssetsResponseItem>();
        let align = std::mem::align_of::<ListAccountAssetsResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ListAccountAssetsResponseItem)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountAssetsResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountAssetsResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountAssetsResponseItem>>();
        let type_size = std::mem::size_of::<ListAccountAssetsResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountAssetsResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountAssetsResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_balance() {
        let instance = ListAccountAssetsResponseItem::default();
        let _: String = instance.balance;
    }
    #[test]
    fn test_field_symbol() {
        let instance = ListAccountAssetsResponseItem::default();
        let _: String = instance.symbol;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountAssetsResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountassetsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountAssetsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountAssetsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountAssetsRequest::default();
        let b = ListAccountAssetsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountAssetsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountAssetsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountAssetsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountAssetsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountAssetsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountAssetsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountAssetsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountAssetsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountAssetsRequest>();
        let align = std::mem::align_of::<ListAccountAssetsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountAssetsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountAssetsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountAssetsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountAssetsRequest>>();
        let type_size = std::mem::size_of::<ListAccountAssetsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountAssetsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountAssetsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = ListAccountAssetsRequest::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = ListAccountAssetsRequest::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn test_field_query() {
        let instance = ListAccountAssetsRequest::default();
        let _: Option<ListAccountAssetsRequestQuery> = instance.query;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountAssetsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountassetsrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountAssetsRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountAssetsRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountAssetsRequestQuery::default();
        let b = ListAccountAssetsRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountAssetsRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountAssetsRequestQuery =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountAssetsRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountAssetsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountAssetsRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountAssetsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountAssetsRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountAssetsRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountAssetsRequestQuery>();
        let align = std::mem::align_of::<ListAccountAssetsRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ListAccountAssetsRequestQuery)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountAssetsRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountAssetsRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountAssetsRequestQuery>>();
        let type_size = std::mem::size_of::<ListAccountAssetsRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountAssetsRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountAssetsRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListAccountAssetsRequestQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListAccountAssetsRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountAssetsRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountsparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountsParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountsParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountsParams::default();
        let b = ListAccountsParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountsParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountsParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountsParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountsParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountsParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountsParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountsParams>();
        let align = std::mem::align_of::<ListAccountsParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountsParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountsParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountsParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountsParams>>();
        let type_size = std::mem::size_of::<ListAccountsParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountsParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountsParams),
            type_size
        );
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = ListAccountsParams::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountsParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountsquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountsQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountsQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountsQuery::default();
        let b = ListAccountsQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountsQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountsQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountsQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountsQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountsQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountsQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountsQuery>();
        let align = std::mem::align_of::<ListAccountsQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountsQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountsQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountsQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountsQuery>>();
        let type_size = std::mem::size_of::<ListAccountsQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountsQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountsQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListAccountsQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListAccountsQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountsQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountsResponse::default();
        let b = ListAccountsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountsResponse>();
        let align = std::mem::align_of::<ListAccountsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountsResponse>>();
        let type_size = std::mem::size_of::<ListAccountsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListAccountsResponse::default();
        let _: Vec<ListAccountsResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListAccountsResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountsresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountsResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountsResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountsResponseItem::default();
        let b = ListAccountsResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountsResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountsResponseItem = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountsResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountsResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountsResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountsResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountsResponseItem>();
        let align = std::mem::align_of::<ListAccountsResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountsResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountsResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountsResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountsResponseItem>>();
        let type_size = std::mem::size_of::<ListAccountsResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountsResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountsResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = ListAccountsResponseItem::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn test_field_exchange_name() {
        let instance = ListAccountsResponseItem::default();
        let _: Option<String> = instance.exchange_name;
    }
    #[test]
    fn test_field_id() {
        let instance = ListAccountsResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_name() {
        let instance = ListAccountsResponseItem::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountsResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountsRequest::default();
        let b = ListAccountsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountsRequest>();
        let align = std::mem::align_of::<ListAccountsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountsRequest>>();
        let type_size = std::mem::size_of::<ListAccountsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = ListAccountsRequest::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn test_field_query() {
        let instance = ListAccountsRequest::default();
        let _: Option<ListAccountsRequestQuery> = instance.query;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listaccountsrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAccountsRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAccountsRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAccountsRequestQuery::default();
        let b = ListAccountsRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAccountsRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAccountsRequestQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAccountsRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAccountsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAccountsRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAccountsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAccountsRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAccountsRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAccountsRequestQuery>();
        let align = std::mem::align_of::<ListAccountsRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListAccountsRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAccountsRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAccountsRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAccountsRequestQuery>>();
        let type_size = std::mem::size_of::<ListAccountsRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAccountsRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAccountsRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListAccountsRequestQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListAccountsRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAccountsRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listassetwithdrawalnetworksparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAssetWithdrawalNetworksParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAssetWithdrawalNetworksParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAssetWithdrawalNetworksParams::default();
        let b = ListAssetWithdrawalNetworksParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAssetWithdrawalNetworksParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAssetWithdrawalNetworksParams =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAssetWithdrawalNetworksParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAssetWithdrawalNetworksParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAssetWithdrawalNetworksParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAssetWithdrawalNetworksParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAssetWithdrawalNetworksParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAssetWithdrawalNetworksParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAssetWithdrawalNetworksParams>();
        let align = std::mem::align_of::<ListAssetWithdrawalNetworksParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ListAssetWithdrawalNetworksParams)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAssetWithdrawalNetworksParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAssetWithdrawalNetworksParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAssetWithdrawalNetworksParams>>();
        let type_size = std::mem::size_of::<ListAssetWithdrawalNetworksParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAssetWithdrawalNetworksParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAssetWithdrawalNetworksParams),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = ListAssetWithdrawalNetworksParams::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_asset() {
        let instance = ListAssetWithdrawalNetworksParams::default();
        let _: String = instance.asset;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = ListAssetWithdrawalNetworksParams::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAssetWithdrawalNetworksParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listassetwithdrawalnetworksresponseelement {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAssetWithdrawalNetworksResponseElement::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAssetWithdrawalNetworksResponseElement::default();
        let b = ListAssetWithdrawalNetworksResponseElement::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAssetWithdrawalNetworksResponseElement =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAssetWithdrawalNetworksResponseElement::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAssetWithdrawalNetworksResponseElement =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAssetWithdrawalNetworksResponseElement::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAssetWithdrawalNetworksResponseElement =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAssetWithdrawalNetworksResponseElement::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result =
            serde_json::from_str::<ListAssetWithdrawalNetworksResponseElement>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAssetWithdrawalNetworksResponseElement>();
        let align = std::mem::align_of::<ListAssetWithdrawalNetworksResponseElement>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ListAssetWithdrawalNetworksResponseElement)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAssetWithdrawalNetworksResponseElement>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAssetWithdrawalNetworksResponseElement>>();
        let type_size = std::mem::size_of::<ListAssetWithdrawalNetworksResponseElement>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAssetWithdrawalNetworksResponseElement),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAssetWithdrawalNetworksResponseElement),
            type_size
        );
    }
    #[test]
    fn test_field_decimals() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: f64 = instance.decimals;
    }
    #[test]
    fn test_field_kind() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: ListAssetWithdrawalNetworksResponseKind = instance.kind;
    }
    #[test]
    fn test_field_network() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Network = instance.network;
    }
    #[test]
    fn test_field_metadata() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Option<String> = instance.metadata;
    }
    #[test]
    fn test_field_asset_id() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Option<String> = instance.asset_id;
    }
    #[test]
    fn test_field_contract() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Option<String> = instance.contract;
    }
    #[test]
    fn test_field_asset_code() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Option<String> = instance.asset_code;
    }
    #[test]
    fn test_field_issuer() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Option<String> = instance.issuer;
    }
    #[test]
    fn test_field_token_id() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_mint() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Option<String> = instance.mint;
    }
    #[test]
    fn test_field_master() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let _: Option<String> = instance.master;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAssetWithdrawalNetworksResponseElement::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listassetwithdrawalnetworksresponsekind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAssetWithdrawalNetworksResponseKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAssetWithdrawalNetworksResponseKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAssetWithdrawalNetworksResponseKind::default();
        let b = ListAssetWithdrawalNetworksResponseKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAssetWithdrawalNetworksResponseKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAssetWithdrawalNetworksResponseKind =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAssetWithdrawalNetworksResponseKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAssetWithdrawalNetworksResponseKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAssetWithdrawalNetworksResponseKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAssetWithdrawalNetworksResponseKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAssetWithdrawalNetworksResponseKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAssetWithdrawalNetworksResponseKind>();
        let align = std::mem::align_of::<ListAssetWithdrawalNetworksResponseKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ListAssetWithdrawalNetworksResponseKind)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAssetWithdrawalNetworksResponseKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAssetWithdrawalNetworksResponseKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAssetWithdrawalNetworksResponseKind>>();
        let type_size = std::mem::size_of::<ListAssetWithdrawalNetworksResponseKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAssetWithdrawalNetworksResponseKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAssetWithdrawalNetworksResponseKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_network {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Network::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Network::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Network::default();
        let b = Network::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Network::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Network = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Network::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Network =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Network::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Network =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }

    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Network>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Network>();
        let align = std::mem::align_of::<Network>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Network));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Network::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Network>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Network>>();
        let type_size = std::mem::size_of::<Network>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(Network),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(Network), type_size);
    }
}
#[cfg(test)]
mod test_listassetwithdrawalnetworksrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListAssetWithdrawalNetworksRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListAssetWithdrawalNetworksRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListAssetWithdrawalNetworksRequest::default();
        let b = ListAssetWithdrawalNetworksRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListAssetWithdrawalNetworksRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListAssetWithdrawalNetworksRequest =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListAssetWithdrawalNetworksRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListAssetWithdrawalNetworksRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListAssetWithdrawalNetworksRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListAssetWithdrawalNetworksRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListAssetWithdrawalNetworksRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListAssetWithdrawalNetworksRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListAssetWithdrawalNetworksRequest>();
        let align = std::mem::align_of::<ListAssetWithdrawalNetworksRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ListAssetWithdrawalNetworksRequest)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListAssetWithdrawalNetworksRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListAssetWithdrawalNetworksRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListAssetWithdrawalNetworksRequest>>();
        let type_size = std::mem::size_of::<ListAssetWithdrawalNetworksRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListAssetWithdrawalNetworksRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListAssetWithdrawalNetworksRequest),
            type_size
        );
    }
    #[test]
    fn test_field_account_id() {
        let instance = ListAssetWithdrawalNetworksRequest::default();
        let _: String = instance.account_id;
    }
    #[test]
    fn test_field_asset() {
        let instance = ListAssetWithdrawalNetworksRequest::default();
        let _: String = instance.asset;
    }
    #[test]
    fn test_field_exchange_id() {
        let instance = ListAssetWithdrawalNetworksRequest::default();
        let _: String = instance.exchange_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListAssetWithdrawalNetworksRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listexchangesquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListExchangesQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListExchangesQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListExchangesQuery::default();
        let b = ListExchangesQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListExchangesQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListExchangesQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListExchangesQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListExchangesQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListExchangesQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListExchangesQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListExchangesQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListExchangesQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListExchangesQuery>();
        let align = std::mem::align_of::<ListExchangesQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListExchangesQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListExchangesQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListExchangesQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListExchangesQuery>>();
        let type_size = std::mem::size_of::<ListExchangesQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListExchangesQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListExchangesQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListExchangesQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListExchangesQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListExchangesQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listexchangesresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListExchangesResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListExchangesResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListExchangesResponse::default();
        let b = ListExchangesResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListExchangesResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListExchangesResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListExchangesResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListExchangesResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListExchangesResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListExchangesResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListExchangesResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListExchangesResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListExchangesResponse>();
        let align = std::mem::align_of::<ListExchangesResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListExchangesResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListExchangesResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListExchangesResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListExchangesResponse>>();
        let type_size = std::mem::size_of::<ListExchangesResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListExchangesResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListExchangesResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListExchangesResponse::default();
        let _: Vec<ListExchangesResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListExchangesResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListExchangesResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listexchangesresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListExchangesResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListExchangesResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListExchangesResponseItem::default();
        let b = ListExchangesResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListExchangesResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListExchangesResponseItem = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListExchangesResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListExchangesResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListExchangesResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListExchangesResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListExchangesResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListExchangesResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListExchangesResponseItem>();
        let align = std::mem::align_of::<ListExchangesResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListExchangesResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListExchangesResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListExchangesResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListExchangesResponseItem>>();
        let type_size = std::mem::size_of::<ListExchangesResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListExchangesResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListExchangesResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = ListExchangesResponseItem::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = ListExchangesResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = ListExchangesResponseItem::default();
        let _: CreateExchangeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_name() {
        let instance = ListExchangesResponseItem::default();
        let _: Option<String> = instance.name;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListExchangesResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listexchangesrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListExchangesRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListExchangesRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListExchangesRequest::default();
        let b = ListExchangesRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListExchangesRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListExchangesRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListExchangesRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListExchangesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListExchangesRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListExchangesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListExchangesRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListExchangesRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListExchangesRequest>();
        let align = std::mem::align_of::<ListExchangesRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListExchangesRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListExchangesRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListExchangesRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListExchangesRequest>>();
        let type_size = std::mem::size_of::<ListExchangesRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListExchangesRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListExchangesRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListExchangesRequest::default();
        let _: Option<ListExchangesRequestQuery> = instance.query;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListExchangesRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listexchangesrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListExchangesRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListExchangesRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListExchangesRequestQuery::default();
        let b = ListExchangesRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListExchangesRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListExchangesRequestQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListExchangesRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListExchangesRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListExchangesRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListExchangesRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListExchangesRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListExchangesRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListExchangesRequestQuery>();
        let align = std::mem::align_of::<ListExchangesRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListExchangesRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListExchangesRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListExchangesRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListExchangesRequestQuery>>();
        let type_size = std::mem::size_of::<ListExchangesRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListExchangesRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListExchangesRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListExchangesRequestQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListExchangesRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListExchangesRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
