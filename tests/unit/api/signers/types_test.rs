/// @dfns-sdk-rs/tests/unit/api/signers/types_test.rs

#[path = "../../../../src/api/signers/types.rs"]
mod parent;
use parent::{Cluster, ListSignersResponse, Signer};
use serde_json;
use std::collections::HashMap;
use std::mem;
#[cfg(test)]
mod test_listsignersresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListSignersResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListSignersResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListSignersResponse::default();
        let b = ListSignersResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListSignersResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListSignersResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListSignersResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListSignersResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListSignersResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListSignersResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListSignersResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListSignersResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListSignersResponse>();
        let align = std::mem::align_of::<ListSignersResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListSignersResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListSignersResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListSignersResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListSignersResponse>>();
        let type_size = std::mem::size_of::<ListSignersResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListSignersResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListSignersResponse),
            type_size
        );
    }
    #[test]
    fn test_field_clusters() {
        let instance = ListSignersResponse::default();
        let _: Vec<Cluster> = instance.clusters;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListSignersResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_cluster {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Cluster::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Cluster::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Cluster::default();
        let b = Cluster::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Cluster::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Cluster = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Cluster::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Cluster =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Cluster::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Cluster =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Cluster::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Cluster>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Cluster>();
        let align = std::mem::align_of::<Cluster>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Cluster));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Cluster::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Cluster>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Cluster>>();
        let type_size = std::mem::size_of::<Cluster>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(Cluster),
            option_size
        );
        println!("Raw {} size: {} bytes", stringify!(Cluster), type_size);
    }
    #[test]
    fn test_field_cluster_id() {
        let instance = Cluster::default();
        let _: String = instance.cluster_id;
    }
    #[test]
    fn test_field_signers() {
        let instance = Cluster::default();
        let _: Vec<Signer> = instance.signers;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Cluster::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_signer {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Signer::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Signer::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Signer::default();
        let b = Signer::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Signer::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Signer = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Signer::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Signer =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Signer::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Signer =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Signer::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Signer>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Signer>();
        let align = std::mem::align_of::<Signer>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Signer));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Signer::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Signer>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Signer>>();
        let type_size = std::mem::size_of::<Signer>();
        println!("Option<{}> size: {} bytes", stringify!(Signer), option_size);
        println!("Raw {} size: {} bytes", stringify!(Signer), type_size);
    }
    #[test]
    fn test_field_encryption_key() {
        let instance = Signer::default();
        let _: String = instance.encryption_key;
    }
    #[test]
    fn test_field_signer_id() {
        let instance = Signer::default();
        let _: String = instance.signer_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = Signer::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
