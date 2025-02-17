/// @dfns-sdk-rs/tests/unit/api/staking/types_test.rs

#[path = "../../../../src/api/staking/types.rs"]
mod parent;
use parent::{
    Body, CreateStakeActionBody, CreateStakeActionBodyKind, CreateStakeActionParams,
    CreateStakeActionRequest, CreateStakeActionResponse, CreateStakeActionResponseStake,
    CreateStakeActionResponseStakeAction, CreateStakeBody, CreateStakeBodyKind, CreateStakeRequest,
    CreateStakeResponse, CreateStakeResponseStake, CreateStakeResponseStakeAction,
    FluffyRequestBody, FluffyRequester, GetStakeRewardsParams, GetStakeRewardsRequest,
    GetStakeRewardsResponse, IndecentRequester, IndigoRequester, ItemRequestBody,
    ItemRequestBodyClass, ListStakeActionsQuery, ListStakeActionsRequest,
    ListStakeActionsRequestQuery, ListStakeActionsResponse, ListStakeActionsResponseItem,
    ListStakesQuery, ListStakesRequest, ListStakesRequestQuery, ListStakesResponse,
    ListStakesResponseItem, Protocol, Provider, PurpleKind, PurpleRequestBody, PurpleRequester,
    StakeActionKind, Status, StickyRequestBody, StickyRequester, TentacledRequestBody,
    TentacledRequester,
};
use serde_json;
use std::collections::HashMap;
use std::mem;
#[cfg(test)]
mod test_createstakebody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeBody::default();
        let b = CreateStakeBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeBody>();
        let align = std::mem::align_of::<CreateStakeBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeBody>>();
        let type_size = std::mem::size_of::<CreateStakeBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = CreateStakeBody::default();
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_duration() {
        let instance = CreateStakeBody::default();
        let _: Option<f64> = instance.duration;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateStakeBody::default();
        let _: CreateStakeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = CreateStakeBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = CreateStakeBody::default();
        let _: Provider = instance.provider;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateStakeBody::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createstakebodykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeBodyKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeBodyKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeBodyKind::default();
        let b = CreateStakeBodyKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeBodyKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeBodyKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeBodyKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeBodyKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeBodyKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeBodyKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeBodyKind>();
        let align = std::mem::align_of::<CreateStakeBodyKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeBodyKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeBodyKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeBodyKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeBodyKind>>();
        let type_size = std::mem::size_of::<CreateStakeBodyKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeBodyKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeBodyKind),
            type_size
        );
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
mod test_provider {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Provider::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Provider::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Provider::default();
        let b = Provider::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Provider::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Provider = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Provider::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Provider =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Provider::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Provider =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Provider::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Provider>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Provider>();
        let align = std::mem::align_of::<Provider>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Provider));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Provider::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Provider>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Provider>>();
        let type_size = std::mem::size_of::<Provider>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(Provider),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(Provider), type_size);
    }
}
#[cfg(test)]
mod test_createstakeresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeResponse::default();
        let b = CreateStakeResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeResponse>();
        let align = std::mem::align_of::<CreateStakeResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeResponse>>();
        let type_size = std::mem::size_of::<CreateStakeResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeResponse),
            type_size
        );
    }
    #[test]
    fn test_field_stake() {
        let instance = CreateStakeResponse::default();
        let _: CreateStakeResponseStake = instance.stake;
    }
    #[test]
    fn test_field_stake_action() {
        let instance = CreateStakeResponse::default();
        let _: CreateStakeResponseStakeAction = instance.stake_action;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createstakeresponsestake {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeResponseStake::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeResponseStake::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeResponseStake::default();
        let b = CreateStakeResponseStake::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeResponseStake::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeResponseStake = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeResponseStake::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeResponseStake =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeResponseStake::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeResponseStake =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeResponseStake::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeResponseStake>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeResponseStake>();
        let align = std::mem::align_of::<CreateStakeResponseStake>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeResponseStake));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeResponseStake::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeResponseStake>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeResponseStake>>();
        let type_size = std::mem::size_of::<CreateStakeResponseStake>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeResponseStake),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeResponseStake),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateStakeResponseStake::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateStakeResponseStake::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_protocol() {
        let instance = CreateStakeResponseStake::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = CreateStakeResponseStake::default();
        let _: Provider = instance.provider;
    }
    #[test]
    fn test_field_provider_stake_id() {
        let instance = CreateStakeResponseStake::default();
        let _: String = instance.provider_stake_id;
    }
    #[test]
    fn test_field_request_body() {
        let instance = CreateStakeResponseStake::default();
        let _: TentacledRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = CreateStakeResponseStake::default();
        let _: PurpleRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = CreateStakeResponseStake::default();
        let _: Status = instance.status;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateStakeResponseStake::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeResponseStake::default();
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
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_duration() {
        let instance = TentacledRequestBody::default();
        let _: Option<f64> = instance.duration;
    }
    #[test]
    fn test_field_kind() {
        let instance = TentacledRequestBody::default();
        let _: CreateStakeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = TentacledRequestBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = TentacledRequestBody::default();
        let _: Provider = instance.provider;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = TentacledRequestBody::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = TentacledRequestBody::default();
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
mod test_status {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Status::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Status::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Status::default();
        let b = Status::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Status::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Status = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Status::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Status =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Status::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Status =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Status::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Status>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Status>();
        let align = std::mem::align_of::<Status>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Status));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Status::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Status>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Status>>();
        let type_size = std::mem::size_of::<Status>();
        println!("Option<{}> size: {} bytes", stringify!(Status), option_size);
        println!("Raw {} size: {} bytes", stringify!(Status), type_size);
    }
}
#[cfg(test)]
mod test_createstakeresponsestakeaction {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeResponseStakeAction::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeResponseStakeAction::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeResponseStakeAction::default();
        let b = CreateStakeResponseStakeAction::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeResponseStakeAction::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeResponseStakeAction =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeResponseStakeAction::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeResponseStakeAction =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeResponseStakeAction::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeResponseStakeAction =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeResponseStakeAction::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeResponseStakeAction>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeResponseStakeAction>();
        let align = std::mem::align_of::<CreateStakeResponseStakeAction>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateStakeResponseStakeAction)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeResponseStakeAction::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeResponseStakeAction>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeResponseStakeAction>>();
        let type_size = std::mem::size_of::<CreateStakeResponseStakeAction>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeResponseStakeAction),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeResponseStakeAction),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateStakeResponseStakeAction::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateStakeResponseStakeAction::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateStakeResponseStakeAction::default();
        let _: StakeActionKind = instance.kind;
    }
    #[test]
    fn test_field_request_body() {
        let instance = CreateStakeResponseStakeAction::default();
        let _: PurpleRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = CreateStakeResponseStakeAction::default();
        let _: FluffyRequester = instance.requester;
    }
    #[test]
    fn test_field_stake_id() {
        let instance = CreateStakeResponseStakeAction::default();
        let _: String = instance.stake_id;
    }
    #[test]
    fn test_field_transaction_id() {
        let instance = CreateStakeResponseStakeAction::default();
        let _: Option<String> = instance.transaction_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeResponseStakeAction::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_stakeactionkind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = StakeActionKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = StakeActionKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = StakeActionKind::default();
        let b = StakeActionKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = StakeActionKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: StakeActionKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = StakeActionKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: StakeActionKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = StakeActionKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: StakeActionKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = StakeActionKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<StakeActionKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<StakeActionKind>();
        let align = std::mem::align_of::<StakeActionKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(StakeActionKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = StakeActionKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<StakeActionKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<StakeActionKind>>();
        let type_size = std::mem::size_of::<StakeActionKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(StakeActionKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(StakeActionKind),
            type_size
        );
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
    fn test_field_amount() {
        let instance = PurpleRequestBody::default();
        let _: Option<String> = instance.amount;
    }
    #[test]
    fn test_field_duration() {
        let instance = PurpleRequestBody::default();
        let _: Option<f64> = instance.duration;
    }
    #[test]
    fn test_field_kind() {
        let instance = PurpleRequestBody::default();
        let _: PurpleKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = PurpleRequestBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = PurpleRequestBody::default();
        let _: Option<Provider> = instance.provider;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = PurpleRequestBody::default();
        let _: Option<String> = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PurpleRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_purplekind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PurpleKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PurpleKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PurpleKind::default();
        let b = PurpleKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PurpleKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PurpleKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PurpleKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PurpleKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PurpleKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PurpleKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PurpleKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PurpleKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PurpleKind>();
        let align = std::mem::align_of::<PurpleKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PurpleKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PurpleKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PurpleKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PurpleKind>>();
        let type_size = std::mem::size_of::<PurpleKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PurpleKind),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(PurpleKind), type_size);
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
mod test_createstakerequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeRequest::default();
        let b = CreateStakeRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeRequest>();
        let align = std::mem::align_of::<CreateStakeRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeRequest>>();
        let type_size = std::mem::size_of::<CreateStakeRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = CreateStakeRequest::default();
        let _: Body = instance.body;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeRequest::default();
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
    fn test_field_amount() {
        let instance = Body::default();
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_duration() {
        let instance = Body::default();
        let _: Option<f64> = instance.duration;
    }
    #[test]
    fn test_field_kind() {
        let instance = Body::default();
        let _: CreateStakeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = Body::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = Body::default();
        let _: Provider = instance.provider;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = Body::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Body::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createstakeactionparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeActionParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeActionParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeActionParams::default();
        let b = CreateStakeActionParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeActionParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeActionParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeActionParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeActionParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeActionParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeActionParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeActionParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeActionParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeActionParams>();
        let align = std::mem::align_of::<CreateStakeActionParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeActionParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeActionParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeActionParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeActionParams>>();
        let type_size = std::mem::size_of::<CreateStakeActionParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeActionParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeActionParams),
            type_size
        );
    }
    #[test]
    fn test_field_stake_id() {
        let instance = CreateStakeActionParams::default();
        let _: String = instance.stake_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeActionParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createstakeactionresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeActionResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeActionResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeActionResponse::default();
        let b = CreateStakeActionResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeActionResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeActionResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeActionResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeActionResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeActionResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeActionResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeActionResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeActionResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeActionResponse>();
        let align = std::mem::align_of::<CreateStakeActionResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeActionResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeActionResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeActionResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeActionResponse>>();
        let type_size = std::mem::size_of::<CreateStakeActionResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeActionResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeActionResponse),
            type_size
        );
    }
    #[test]
    fn test_field_stake() {
        let instance = CreateStakeActionResponse::default();
        let _: CreateStakeActionResponseStake = instance.stake;
    }
    #[test]
    fn test_field_stake_action() {
        let instance = CreateStakeActionResponse::default();
        let _: CreateStakeActionResponseStakeAction = instance.stake_action;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeActionResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createstakeactionresponsestake {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeActionResponseStake::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeActionResponseStake::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeActionResponseStake::default();
        let b = CreateStakeActionResponseStake::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeActionResponseStake::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeActionResponseStake =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeActionResponseStake::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeActionResponseStake =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeActionResponseStake::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeActionResponseStake =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeActionResponseStake::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeActionResponseStake>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeActionResponseStake>();
        let align = std::mem::align_of::<CreateStakeActionResponseStake>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateStakeActionResponseStake)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeActionResponseStake::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeActionResponseStake>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeActionResponseStake>>();
        let type_size = std::mem::size_of::<CreateStakeActionResponseStake>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeActionResponseStake),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeActionResponseStake),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateStakeActionResponseStake::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateStakeActionResponseStake::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_protocol() {
        let instance = CreateStakeActionResponseStake::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = CreateStakeActionResponseStake::default();
        let _: Provider = instance.provider;
    }
    #[test]
    fn test_field_provider_stake_id() {
        let instance = CreateStakeActionResponseStake::default();
        let _: String = instance.provider_stake_id;
    }
    #[test]
    fn test_field_request_body() {
        let instance = CreateStakeActionResponseStake::default();
        let _: StickyRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = CreateStakeActionResponseStake::default();
        let _: TentacledRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = CreateStakeActionResponseStake::default();
        let _: Status = instance.status;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = CreateStakeActionResponseStake::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeActionResponseStake::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_stickyrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = StickyRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = StickyRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = StickyRequestBody::default();
        let b = StickyRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = StickyRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: StickyRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = StickyRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: StickyRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = StickyRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: StickyRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = StickyRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<StickyRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<StickyRequestBody>();
        let align = std::mem::align_of::<StickyRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(StickyRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = StickyRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<StickyRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<StickyRequestBody>>();
        let type_size = std::mem::size_of::<StickyRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(StickyRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(StickyRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = StickyRequestBody::default();
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_duration() {
        let instance = StickyRequestBody::default();
        let _: Option<f64> = instance.duration;
    }
    #[test]
    fn test_field_kind() {
        let instance = StickyRequestBody::default();
        let _: CreateStakeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = StickyRequestBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = StickyRequestBody::default();
        let _: Provider = instance.provider;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = StickyRequestBody::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = StickyRequestBody::default();
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
mod test_createstakeactionresponsestakeaction {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeActionResponseStakeAction::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeActionResponseStakeAction::default();
        let b = CreateStakeActionResponseStakeAction::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeActionResponseStakeAction =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeActionResponseStakeAction::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeActionResponseStakeAction =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeActionResponseStakeAction::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeActionResponseStakeAction =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeActionResponseStakeAction::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeActionResponseStakeAction>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeActionResponseStakeAction>();
        let align = std::mem::align_of::<CreateStakeActionResponseStakeAction>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(CreateStakeActionResponseStakeAction)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeActionResponseStakeAction>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeActionResponseStakeAction>>();
        let type_size = std::mem::size_of::<CreateStakeActionResponseStakeAction>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeActionResponseStakeAction),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeActionResponseStakeAction),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let _: StakeActionKind = instance.kind;
    }
    #[test]
    fn test_field_request_body() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let _: FluffyRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let _: StickyRequester = instance.requester;
    }
    #[test]
    fn test_field_stake_id() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let _: String = instance.stake_id;
    }
    #[test]
    fn test_field_transaction_id() {
        let instance = CreateStakeActionResponseStakeAction::default();
        let _: Option<String> = instance.transaction_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeActionResponseStakeAction::default();
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
    fn test_field_amount() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.amount;
    }
    #[test]
    fn test_field_duration() {
        let instance = FluffyRequestBody::default();
        let _: Option<f64> = instance.duration;
    }
    #[test]
    fn test_field_kind() {
        let instance = FluffyRequestBody::default();
        let _: PurpleKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = FluffyRequestBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = FluffyRequestBody::default();
        let _: Option<Provider> = instance.provider;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = FluffyRequestBody::default();
        let _: Option<String> = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = FluffyRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_stickyrequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = StickyRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = StickyRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = StickyRequester::default();
        let b = StickyRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = StickyRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: StickyRequester = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = StickyRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: StickyRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = StickyRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: StickyRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = StickyRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<StickyRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<StickyRequester>();
        let align = std::mem::align_of::<StickyRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(StickyRequester));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = StickyRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<StickyRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<StickyRequester>>();
        let type_size = std::mem::size_of::<StickyRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(StickyRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(StickyRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = StickyRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = StickyRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = StickyRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = StickyRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createstakeactionrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeActionRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeActionRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeActionRequest::default();
        let b = CreateStakeActionRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeActionRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeActionRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeActionRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeActionRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeActionRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeActionRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeActionRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeActionRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeActionRequest>();
        let align = std::mem::align_of::<CreateStakeActionRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeActionRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeActionRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeActionRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeActionRequest>>();
        let type_size = std::mem::size_of::<CreateStakeActionRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeActionRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeActionRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = CreateStakeActionRequest::default();
        let _: CreateStakeActionBody = instance.body;
    }
    #[test]
    fn test_field_stake_id() {
        let instance = CreateStakeActionRequest::default();
        let _: String = instance.stake_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeActionRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createstakeactionbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeActionBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeActionBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeActionBody::default();
        let b = CreateStakeActionBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeActionBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeActionBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeActionBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeActionBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeActionBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeActionBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeActionBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeActionBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeActionBody>();
        let align = std::mem::align_of::<CreateStakeActionBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeActionBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeActionBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeActionBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeActionBody>>();
        let type_size = std::mem::size_of::<CreateStakeActionBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeActionBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeActionBody),
            type_size
        );
    }
    #[test]
    fn test_field_kind() {
        let instance = CreateStakeActionBody::default();
        let _: CreateStakeActionBodyKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = CreateStakeActionBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateStakeActionBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createstakeactionbodykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateStakeActionBodyKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateStakeActionBodyKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateStakeActionBodyKind::default();
        let b = CreateStakeActionBodyKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateStakeActionBodyKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateStakeActionBodyKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateStakeActionBodyKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateStakeActionBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateStakeActionBodyKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateStakeActionBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateStakeActionBodyKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateStakeActionBodyKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateStakeActionBodyKind>();
        let align = std::mem::align_of::<CreateStakeActionBodyKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateStakeActionBodyKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateStakeActionBodyKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateStakeActionBodyKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateStakeActionBodyKind>>();
        let type_size = std::mem::size_of::<CreateStakeActionBodyKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateStakeActionBodyKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateStakeActionBodyKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_getstakerewardsparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetStakeRewardsParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetStakeRewardsParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetStakeRewardsParams::default();
        let b = GetStakeRewardsParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetStakeRewardsParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetStakeRewardsParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetStakeRewardsParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetStakeRewardsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetStakeRewardsParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetStakeRewardsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetStakeRewardsParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetStakeRewardsParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetStakeRewardsParams>();
        let align = std::mem::align_of::<GetStakeRewardsParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetStakeRewardsParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetStakeRewardsParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetStakeRewardsParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetStakeRewardsParams>>();
        let type_size = std::mem::size_of::<GetStakeRewardsParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetStakeRewardsParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetStakeRewardsParams),
            type_size
        );
    }
    #[test]
    fn test_field_stake_id() {
        let instance = GetStakeRewardsParams::default();
        let _: String = instance.stake_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetStakeRewardsParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getstakerewardsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetStakeRewardsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetStakeRewardsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetStakeRewardsResponse::default();
        let b = GetStakeRewardsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetStakeRewardsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetStakeRewardsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetStakeRewardsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetStakeRewardsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetStakeRewardsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetStakeRewardsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetStakeRewardsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetStakeRewardsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetStakeRewardsResponse>();
        let align = std::mem::align_of::<GetStakeRewardsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetStakeRewardsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetStakeRewardsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetStakeRewardsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetStakeRewardsResponse>>();
        let type_size = std::mem::size_of::<GetStakeRewardsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetStakeRewardsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetStakeRewardsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_balance() {
        let instance = GetStakeRewardsResponse::default();
        let _: String = instance.balance;
    }
    #[test]
    fn test_field_symbol() {
        let instance = GetStakeRewardsResponse::default();
        let _: String = instance.symbol;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetStakeRewardsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getstakerewardsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetStakeRewardsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetStakeRewardsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetStakeRewardsRequest::default();
        let b = GetStakeRewardsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetStakeRewardsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetStakeRewardsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetStakeRewardsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetStakeRewardsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetStakeRewardsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetStakeRewardsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetStakeRewardsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetStakeRewardsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetStakeRewardsRequest>();
        let align = std::mem::align_of::<GetStakeRewardsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetStakeRewardsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetStakeRewardsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetStakeRewardsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetStakeRewardsRequest>>();
        let type_size = std::mem::size_of::<GetStakeRewardsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetStakeRewardsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetStakeRewardsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_stake_id() {
        let instance = GetStakeRewardsRequest::default();
        let _: String = instance.stake_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetStakeRewardsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakeactionsquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakeActionsQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakeActionsQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakeActionsQuery::default();
        let b = ListStakeActionsQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakeActionsQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakeActionsQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakeActionsQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakeActionsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakeActionsQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakeActionsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakeActionsQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakeActionsQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakeActionsQuery>();
        let align = std::mem::align_of::<ListStakeActionsQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakeActionsQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakeActionsQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakeActionsQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakeActionsQuery>>();
        let type_size = std::mem::size_of::<ListStakeActionsQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakeActionsQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakeActionsQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListStakeActionsQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListStakeActionsQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakeActionsQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakeactionsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakeActionsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakeActionsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakeActionsResponse::default();
        let b = ListStakeActionsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakeActionsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakeActionsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakeActionsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakeActionsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakeActionsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakeActionsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakeActionsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakeActionsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakeActionsResponse>();
        let align = std::mem::align_of::<ListStakeActionsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakeActionsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakeActionsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakeActionsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakeActionsResponse>>();
        let type_size = std::mem::size_of::<ListStakeActionsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakeActionsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakeActionsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListStakeActionsResponse::default();
        let _: Vec<ListStakeActionsResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListStakeActionsResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakeActionsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakeactionsresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakeActionsResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakeActionsResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakeActionsResponseItem::default();
        let b = ListStakeActionsResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakeActionsResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakeActionsResponseItem =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakeActionsResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakeActionsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakeActionsResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakeActionsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakeActionsResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakeActionsResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakeActionsResponseItem>();
        let align = std::mem::align_of::<ListStakeActionsResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakeActionsResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakeActionsResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakeActionsResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakeActionsResponseItem>>();
        let type_size = std::mem::size_of::<ListStakeActionsResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakeActionsResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakeActionsResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = ListStakeActionsResponseItem::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = ListStakeActionsResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = ListStakeActionsResponseItem::default();
        let _: StakeActionKind = instance.kind;
    }
    #[test]
    fn test_field_request_body() {
        let instance = ListStakeActionsResponseItem::default();
        let _: ItemRequestBody = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = ListStakeActionsResponseItem::default();
        let _: IndigoRequester = instance.requester;
    }
    #[test]
    fn test_field_stake_id() {
        let instance = ListStakeActionsResponseItem::default();
        let _: String = instance.stake_id;
    }
    #[test]
    fn test_field_transaction_id() {
        let instance = ListStakeActionsResponseItem::default();
        let _: Option<String> = instance.transaction_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakeActionsResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_itemrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ItemRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ItemRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ItemRequestBody::default();
        let b = ItemRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ItemRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ItemRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ItemRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ItemRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ItemRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ItemRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ItemRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ItemRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ItemRequestBody>();
        let align = std::mem::align_of::<ItemRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ItemRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ItemRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ItemRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ItemRequestBody>>();
        let type_size = std::mem::size_of::<ItemRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ItemRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ItemRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = ItemRequestBody::default();
        let _: Option<String> = instance.amount;
    }
    #[test]
    fn test_field_duration() {
        let instance = ItemRequestBody::default();
        let _: Option<f64> = instance.duration;
    }
    #[test]
    fn test_field_kind() {
        let instance = ItemRequestBody::default();
        let _: PurpleKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = ItemRequestBody::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = ItemRequestBody::default();
        let _: Option<Provider> = instance.provider;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ItemRequestBody::default();
        let _: Option<String> = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ItemRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_indigorequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = IndigoRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = IndigoRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = IndigoRequester::default();
        let b = IndigoRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = IndigoRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: IndigoRequester = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = IndigoRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: IndigoRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = IndigoRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: IndigoRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = IndigoRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<IndigoRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<IndigoRequester>();
        let align = std::mem::align_of::<IndigoRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(IndigoRequester));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = IndigoRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<IndigoRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<IndigoRequester>>();
        let type_size = std::mem::size_of::<IndigoRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(IndigoRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(IndigoRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = IndigoRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = IndigoRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = IndigoRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = IndigoRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakeactionsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakeActionsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakeActionsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakeActionsRequest::default();
        let b = ListStakeActionsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakeActionsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakeActionsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakeActionsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakeActionsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakeActionsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakeActionsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakeActionsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakeActionsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakeActionsRequest>();
        let align = std::mem::align_of::<ListStakeActionsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakeActionsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakeActionsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakeActionsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakeActionsRequest>>();
        let type_size = std::mem::size_of::<ListStakeActionsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakeActionsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakeActionsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListStakeActionsRequest::default();
        let _: Option<ListStakeActionsRequestQuery> = instance.query;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakeActionsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakeactionsrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakeActionsRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakeActionsRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakeActionsRequestQuery::default();
        let b = ListStakeActionsRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakeActionsRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakeActionsRequestQuery =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakeActionsRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakeActionsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakeActionsRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakeActionsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakeActionsRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakeActionsRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakeActionsRequestQuery>();
        let align = std::mem::align_of::<ListStakeActionsRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakeActionsRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakeActionsRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakeActionsRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakeActionsRequestQuery>>();
        let type_size = std::mem::size_of::<ListStakeActionsRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakeActionsRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakeActionsRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListStakeActionsRequestQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListStakeActionsRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakeActionsRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakesquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakesQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakesQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakesQuery::default();
        let b = ListStakesQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakesQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakesQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakesQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakesQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakesQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakesQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakesQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakesQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakesQuery>();
        let align = std::mem::align_of::<ListStakesQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakesQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakesQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakesQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakesQuery>>();
        let type_size = std::mem::size_of::<ListStakesQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakesQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakesQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListStakesQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListStakesQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakesQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakesresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakesResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakesResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakesResponse::default();
        let b = ListStakesResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakesResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakesResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakesResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakesResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakesResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakesResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakesResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakesResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakesResponse>();
        let align = std::mem::align_of::<ListStakesResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakesResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakesResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakesResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakesResponse>>();
        let type_size = std::mem::size_of::<ListStakesResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakesResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakesResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListStakesResponse::default();
        let _: Vec<ListStakesResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListStakesResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakesResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakesresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakesResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakesResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakesResponseItem::default();
        let b = ListStakesResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakesResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakesResponseItem = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakesResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakesResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakesResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakesResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakesResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakesResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakesResponseItem>();
        let align = std::mem::align_of::<ListStakesResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakesResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakesResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakesResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakesResponseItem>>();
        let type_size = std::mem::size_of::<ListStakesResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakesResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakesResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = ListStakesResponseItem::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_id() {
        let instance = ListStakesResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_protocol() {
        let instance = ListStakesResponseItem::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = ListStakesResponseItem::default();
        let _: Provider = instance.provider;
    }
    #[test]
    fn test_field_provider_stake_id() {
        let instance = ListStakesResponseItem::default();
        let _: String = instance.provider_stake_id;
    }
    #[test]
    fn test_field_request_body() {
        let instance = ListStakesResponseItem::default();
        let _: ItemRequestBodyClass = instance.request_body;
    }
    #[test]
    fn test_field_requester() {
        let instance = ListStakesResponseItem::default();
        let _: IndecentRequester = instance.requester;
    }
    #[test]
    fn test_field_status() {
        let instance = ListStakesResponseItem::default();
        let _: Status = instance.status;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ListStakesResponseItem::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakesResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_itemrequestbodyclass {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ItemRequestBodyClass::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ItemRequestBodyClass::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ItemRequestBodyClass::default();
        let b = ItemRequestBodyClass::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ItemRequestBodyClass::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ItemRequestBodyClass = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ItemRequestBodyClass::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ItemRequestBodyClass =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ItemRequestBodyClass::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ItemRequestBodyClass =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ItemRequestBodyClass::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ItemRequestBodyClass>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ItemRequestBodyClass>();
        let align = std::mem::align_of::<ItemRequestBodyClass>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ItemRequestBodyClass));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ItemRequestBodyClass::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ItemRequestBodyClass>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ItemRequestBodyClass>>();
        let type_size = std::mem::size_of::<ItemRequestBodyClass>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ItemRequestBodyClass),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ItemRequestBodyClass),
            type_size
        );
    }
    #[test]
    fn test_field_amount() {
        let instance = ItemRequestBodyClass::default();
        let _: String = instance.amount;
    }
    #[test]
    fn test_field_duration() {
        let instance = ItemRequestBodyClass::default();
        let _: Option<f64> = instance.duration;
    }
    #[test]
    fn test_field_kind() {
        let instance = ItemRequestBodyClass::default();
        let _: CreateStakeBodyKind = instance.kind;
    }
    #[test]
    fn test_field_protocol() {
        let instance = ItemRequestBodyClass::default();
        let _: Protocol = instance.protocol;
    }
    #[test]
    fn test_field_provider() {
        let instance = ItemRequestBodyClass::default();
        let _: Provider = instance.provider;
    }
    #[test]
    fn test_field_wallet_id() {
        let instance = ItemRequestBodyClass::default();
        let _: String = instance.wallet_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ItemRequestBodyClass::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_indecentrequester {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = IndecentRequester::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = IndecentRequester::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = IndecentRequester::default();
        let b = IndecentRequester::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = IndecentRequester::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: IndecentRequester = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = IndecentRequester::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: IndecentRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = IndecentRequester::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: IndecentRequester =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = IndecentRequester::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<IndecentRequester>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<IndecentRequester>();
        let align = std::mem::align_of::<IndecentRequester>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(IndecentRequester));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = IndecentRequester::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<IndecentRequester>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<IndecentRequester>>();
        let type_size = std::mem::size_of::<IndecentRequester>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(IndecentRequester),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(IndecentRequester),
            type_size
        );
    }
    #[test]
    fn test_field_app_id() {
        let instance = IndecentRequester::default();
        let _: Option<String> = instance.app_id;
    }
    #[test]
    fn test_field_token_id() {
        let instance = IndecentRequester::default();
        let _: Option<String> = instance.token_id;
    }
    #[test]
    fn test_field_user_id() {
        let instance = IndecentRequester::default();
        let _: String = instance.user_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = IndecentRequester::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakesrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakesRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakesRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakesRequest::default();
        let b = ListStakesRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakesRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakesRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakesRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakesRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakesRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakesRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakesRequest>();
        let align = std::mem::align_of::<ListStakesRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakesRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakesRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakesRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakesRequest>>();
        let type_size = std::mem::size_of::<ListStakesRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakesRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakesRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListStakesRequest::default();
        let _: Option<ListStakesRequestQuery> = instance.query;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakesRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_liststakesrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListStakesRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListStakesRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListStakesRequestQuery::default();
        let b = ListStakesRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListStakesRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListStakesRequestQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListStakesRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListStakesRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListStakesRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListStakesRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListStakesRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListStakesRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListStakesRequestQuery>();
        let align = std::mem::align_of::<ListStakesRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListStakesRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListStakesRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListStakesRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListStakesRequestQuery>>();
        let type_size = std::mem::size_of::<ListStakesRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListStakesRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListStakesRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListStakesRequestQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListStakesRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListStakesRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
