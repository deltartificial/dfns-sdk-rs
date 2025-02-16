/// @dfns-sdk-rs/tests/unit/api/networks/types_test.rs

#[path = "../../../src/api/networks/types.rs"]
mod parent;
use parent::{
    Body, Fast, GetFeesQuery, GetFeesQueryNetwork, GetFeesRequest, GetFeesResponse,
    GetFeesResponseKind, GetFeesResponseNetwork, Query, ReadContractBody, ReadContractBodyKind,
    ReadContractBodyNetwork, ReadContractRequest, ReadContractResponse, Slow, Standard,
};
use serde_json;
use std::mem;
#[cfg(test)]
mod test_getfeesquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetFeesQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetFeesQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetFeesQuery::default();
        let b = GetFeesQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetFeesQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetFeesQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetFeesQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetFeesQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetFeesQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetFeesQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetFeesQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetFeesQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetFeesQuery>();
        let align = std::mem::align_of::<GetFeesQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetFeesQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetFeesQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetFeesQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetFeesQuery>>();
        let type_size = std::mem::size_of::<GetFeesQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetFeesQuery),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(GetFeesQuery), type_size);
    }
    #[test]
    fn test_field_network() {
        let instance = GetFeesQuery::default();
        let _: GetFeesQueryNetwork = instance.network;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetFeesQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getfeesquerynetwork {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetFeesQueryNetwork::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetFeesQueryNetwork::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetFeesQueryNetwork::default();
        let b = GetFeesQueryNetwork::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetFeesQueryNetwork::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetFeesQueryNetwork = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetFeesQueryNetwork::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetFeesQueryNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetFeesQueryNetwork::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetFeesQueryNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetFeesQueryNetwork::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetFeesQueryNetwork>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetFeesQueryNetwork>();
        let align = std::mem::align_of::<GetFeesQueryNetwork>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetFeesQueryNetwork));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetFeesQueryNetwork::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetFeesQueryNetwork>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetFeesQueryNetwork>>();
        let type_size = std::mem::size_of::<GetFeesQueryNetwork>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetFeesQueryNetwork),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetFeesQueryNetwork),
            type_size
        );
    }
}
#[cfg(test)]
mod test_getfeesresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetFeesResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetFeesResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetFeesResponse::default();
        let b = GetFeesResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetFeesResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetFeesResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetFeesResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetFeesResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetFeesResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetFeesResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetFeesResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetFeesResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetFeesResponse>();
        let align = std::mem::align_of::<GetFeesResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetFeesResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetFeesResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetFeesResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetFeesResponse>>();
        let type_size = std::mem::size_of::<GetFeesResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetFeesResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetFeesResponse),
            type_size
        );
    }
    #[test]
    fn test_field_block_number() {
        let instance = GetFeesResponse::default();
        let _: f64 = instance.block_number;
    }
    #[test]
    fn test_field_fast() {
        let instance = GetFeesResponse::default();
        let _: Fast = instance.fast;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetFeesResponse::default();
        let _: GetFeesResponseKind = instance.kind;
    }
    #[test]
    fn test_field_network() {
        let instance = GetFeesResponse::default();
        let _: GetFeesResponseNetwork = instance.network;
    }
    #[test]
    fn test_field_slow() {
        let instance = GetFeesResponse::default();
        let _: Slow = instance.slow;
    }
    #[test]
    fn test_field_standard() {
        let instance = GetFeesResponse::default();
        let _: Standard = instance.standard;
    }
    #[test]
    fn test_field_estimated_base_fee() {
        let instance = GetFeesResponse::default();
        let _: Option<f64> = instance.estimated_base_fee;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetFeesResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_fast {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Fast::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Fast::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Fast::default();
        let b = Fast::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Fast::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Fast = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Fast::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Fast = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Fast::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Fast =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Fast::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Fast>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Fast>();
        let align = std::mem::align_of::<Fast>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Fast));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Fast::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Fast>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Fast>>();
        let type_size = std::mem::size_of::<Fast>();
        println!("Option<{}> size: {} bytes", stringify!(Fast), option_size);
        println!("Raw {} size: {} bytes", stringify!(Fast), type_size);
    }
    #[test]
    fn test_field_block_horizon() {
        let instance = Fast::default();
        let _: Option<f64> = instance.block_horizon;
    }
    #[test]
    fn test_field_fee_rate() {
        let instance = Fast::default();
        let _: Option<String> = instance.fee_rate;
    }
    #[test]
    fn test_field_max_fee_per_gas() {
        let instance = Fast::default();
        let _: Option<String> = instance.max_fee_per_gas;
    }
    #[test]
    fn test_field_max_priority_fee_per_gas() {
        let instance = Fast::default();
        let _: Option<String> = instance.max_priority_fee_per_gas;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Fast::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getfeesresponsekind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetFeesResponseKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetFeesResponseKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetFeesResponseKind::default();
        let b = GetFeesResponseKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetFeesResponseKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetFeesResponseKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetFeesResponseKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetFeesResponseKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetFeesResponseKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetFeesResponseKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetFeesResponseKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetFeesResponseKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetFeesResponseKind>();
        let align = std::mem::align_of::<GetFeesResponseKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetFeesResponseKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetFeesResponseKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetFeesResponseKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetFeesResponseKind>>();
        let type_size = std::mem::size_of::<GetFeesResponseKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetFeesResponseKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetFeesResponseKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_getfeesresponsenetwork {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetFeesResponseNetwork::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetFeesResponseNetwork::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetFeesResponseNetwork::default();
        let b = GetFeesResponseNetwork::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetFeesResponseNetwork::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetFeesResponseNetwork = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetFeesResponseNetwork::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetFeesResponseNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetFeesResponseNetwork::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetFeesResponseNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetFeesResponseNetwork::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetFeesResponseNetwork>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetFeesResponseNetwork>();
        let align = std::mem::align_of::<GetFeesResponseNetwork>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetFeesResponseNetwork));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetFeesResponseNetwork::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetFeesResponseNetwork>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetFeesResponseNetwork>>();
        let type_size = std::mem::size_of::<GetFeesResponseNetwork>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetFeesResponseNetwork),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetFeesResponseNetwork),
            type_size
        );
    }
}
#[cfg(test)]
mod test_slow {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Slow::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Slow::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Slow::default();
        let b = Slow::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Slow::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Slow = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Slow::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Slow = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Slow::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Slow =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Slow::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Slow>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Slow>();
        let align = std::mem::align_of::<Slow>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Slow));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Slow::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Slow>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Slow>>();
        let type_size = std::mem::size_of::<Slow>();
        println!("Option<{}> size: {} bytes", stringify!(Slow), option_size);
        println!("Raw {} size: {} bytes", stringify!(Slow), type_size);
    }
    #[test]
    fn test_field_block_horizon() {
        let instance = Slow::default();
        let _: Option<f64> = instance.block_horizon;
    }
    #[test]
    fn test_field_fee_rate() {
        let instance = Slow::default();
        let _: Option<String> = instance.fee_rate;
    }
    #[test]
    fn test_field_max_fee_per_gas() {
        let instance = Slow::default();
        let _: Option<String> = instance.max_fee_per_gas;
    }
    #[test]
    fn test_field_max_priority_fee_per_gas() {
        let instance = Slow::default();
        let _: Option<String> = instance.max_priority_fee_per_gas;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Slow::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_standard {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Standard::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Standard::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Standard::default();
        let b = Standard::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Standard::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Standard = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Standard::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Standard =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Standard::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Standard =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Standard::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Standard>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Standard>();
        let align = std::mem::align_of::<Standard>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Standard));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Standard::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Standard>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Standard>>();
        let type_size = std::mem::size_of::<Standard>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(Standard),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(Standard), type_size);
    }
    #[test]
    fn test_field_block_horizon() {
        let instance = Standard::default();
        let _: Option<f64> = instance.block_horizon;
    }
    #[test]
    fn test_field_fee_rate() {
        let instance = Standard::default();
        let _: Option<String> = instance.fee_rate;
    }
    #[test]
    fn test_field_max_fee_per_gas() {
        let instance = Standard::default();
        let _: Option<String> = instance.max_fee_per_gas;
    }
    #[test]
    fn test_field_max_priority_fee_per_gas() {
        let instance = Standard::default();
        let _: Option<String> = instance.max_priority_fee_per_gas;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Standard::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getfeesrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetFeesRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetFeesRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetFeesRequest::default();
        let b = GetFeesRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetFeesRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetFeesRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetFeesRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetFeesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetFeesRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetFeesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetFeesRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetFeesRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetFeesRequest>();
        let align = std::mem::align_of::<GetFeesRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetFeesRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetFeesRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetFeesRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetFeesRequest>>();
        let type_size = std::mem::size_of::<GetFeesRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetFeesRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetFeesRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = GetFeesRequest::default();
        let _: Option<Query> = instance.query;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetFeesRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_query {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Query::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Query::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Query::default();
        let b = Query::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Query::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Query = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Query::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Query = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Query::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Query =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Query::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Query>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Query>();
        let align = std::mem::align_of::<Query>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Query));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Query::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Query>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Query>>();
        let type_size = std::mem::size_of::<Query>();
        println!("Option<{}> size: {} bytes", stringify!(Query), option_size);
        println!("Raw {} size: {} bytes", stringify!(Query), type_size);
    }
    #[test]
    fn test_field_network() {
        let instance = Query::default();
        let _: GetFeesQueryNetwork = instance.network;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Query::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_readcontractbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ReadContractBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ReadContractBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ReadContractBody::default();
        let b = ReadContractBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ReadContractBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ReadContractBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ReadContractBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ReadContractBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ReadContractBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ReadContractBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ReadContractBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ReadContractBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ReadContractBody>();
        let align = std::mem::align_of::<ReadContractBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ReadContractBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ReadContractBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ReadContractBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ReadContractBody>>();
        let type_size = std::mem::size_of::<ReadContractBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ReadContractBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ReadContractBody),
            type_size
        );
    }
    #[test]
    fn test_field_contract() {
        let instance = ReadContractBody::default();
        let _: String = instance.contract;
    }
    #[test]
    fn test_field_data() {
        let instance = ReadContractBody::default();
        let _: String = instance.data;
    }
    #[test]
    fn test_field_kind() {
        let instance = ReadContractBody::default();
        let _: ReadContractBodyKind = instance.kind;
    }
    #[test]
    fn test_field_network() {
        let instance = ReadContractBody::default();
        let _: ReadContractBodyNetwork = instance.network;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ReadContractBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_readcontractbodykind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ReadContractBodyKind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ReadContractBodyKind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ReadContractBodyKind::default();
        let b = ReadContractBodyKind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ReadContractBodyKind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ReadContractBodyKind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ReadContractBodyKind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ReadContractBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ReadContractBodyKind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ReadContractBodyKind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ReadContractBodyKind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ReadContractBodyKind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ReadContractBodyKind>();
        let align = std::mem::align_of::<ReadContractBodyKind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ReadContractBodyKind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ReadContractBodyKind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ReadContractBodyKind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ReadContractBodyKind>>();
        let type_size = std::mem::size_of::<ReadContractBodyKind>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ReadContractBodyKind),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ReadContractBodyKind),
            type_size
        );
    }
}
#[cfg(test)]
mod test_readcontractbodynetwork {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ReadContractBodyNetwork::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ReadContractBodyNetwork::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ReadContractBodyNetwork::default();
        let b = ReadContractBodyNetwork::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ReadContractBodyNetwork::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ReadContractBodyNetwork = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ReadContractBodyNetwork::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ReadContractBodyNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ReadContractBodyNetwork::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ReadContractBodyNetwork =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ReadContractBodyNetwork::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ReadContractBodyNetwork>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ReadContractBodyNetwork>();
        let align = std::mem::align_of::<ReadContractBodyNetwork>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ReadContractBodyNetwork));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ReadContractBodyNetwork::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ReadContractBodyNetwork>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ReadContractBodyNetwork>>();
        let type_size = std::mem::size_of::<ReadContractBodyNetwork>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ReadContractBodyNetwork),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ReadContractBodyNetwork),
            type_size
        );
    }
}
#[cfg(test)]
mod test_readcontractresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ReadContractResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ReadContractResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ReadContractResponse::default();
        let b = ReadContractResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ReadContractResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ReadContractResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ReadContractResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ReadContractResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ReadContractResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ReadContractResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ReadContractResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ReadContractResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ReadContractResponse>();
        let align = std::mem::align_of::<ReadContractResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ReadContractResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ReadContractResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ReadContractResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ReadContractResponse>>();
        let type_size = std::mem::size_of::<ReadContractResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ReadContractResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ReadContractResponse),
            type_size
        );
    }
    #[test]
    fn test_field_data() {
        let instance = ReadContractResponse::default();
        let _: String = instance.data;
    }
    #[test]
    fn test_field_kind() {
        let instance = ReadContractResponse::default();
        let _: ReadContractBodyKind = instance.kind;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ReadContractResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_readcontractrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ReadContractRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ReadContractRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ReadContractRequest::default();
        let b = ReadContractRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ReadContractRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ReadContractRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ReadContractRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ReadContractRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ReadContractRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ReadContractRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ReadContractRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ReadContractRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ReadContractRequest>();
        let align = std::mem::align_of::<ReadContractRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ReadContractRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ReadContractRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ReadContractRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ReadContractRequest>>();
        let type_size = std::mem::size_of::<ReadContractRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ReadContractRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ReadContractRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = ReadContractRequest::default();
        let _: Body = instance.body;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ReadContractRequest::default();
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
    fn test_field_contract() {
        let instance = Body::default();
        let _: String = instance.contract;
    }
    #[test]
    fn test_field_data() {
        let instance = Body::default();
        let _: String = instance.data;
    }
    #[test]
    fn test_field_kind() {
        let instance = Body::default();
        let _: ReadContractBodyKind = instance.kind;
    }
    #[test]
    fn test_field_network() {
        let instance = Body::default();
        let _: ReadContractBodyNetwork = instance.network;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Body::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
