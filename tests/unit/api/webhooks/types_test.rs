/// @dfns-sdk-rs/tests/unit/api/webhooks/types_test.rs

#[path = "../../../../src/api/webhooks/types.rs"]
mod parent;
use parent::{
    CreateWebhookBody, CreateWebhookRequest, CreateWebhookRequestBody, CreateWebhookResponse,
    DeleteWebhookParams, DeleteWebhookRequest, DeleteWebhookResponse, DeliveryFailed, Event,
    GetWebhookEventParams, GetWebhookEventRequest, GetWebhookEventResponse, GetWebhookParams,
    GetWebhookRequest, GetWebhookResponse, Kind, ListWebhookEventsParams, ListWebhookEventsQuery,
    ListWebhookEventsRequest, ListWebhookEventsRequestQuery, ListWebhookEventsResponse,
    ListWebhookEventsResponseItem, ListWebhooksQuery, ListWebhooksRequest,
    ListWebhooksRequestQuery, ListWebhooksResponse, ListWebhooksResponseItem, PingWebhookParams,
    PingWebhookRequest, PingWebhookResponse, Status, UpdateWebhookBody, UpdateWebhookParams,
    UpdateWebhookRequest, UpdateWebhookRequestBody, UpdateWebhookResponse,
};
use serde_json;
use std::mem;
#[cfg(test)]
mod test_createwebhookbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWebhookBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWebhookBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWebhookBody::default();
        let b = CreateWebhookBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWebhookBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWebhookBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWebhookBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWebhookBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWebhookBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWebhookBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWebhookBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWebhookBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWebhookBody>();
        let align = std::mem::align_of::<CreateWebhookBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWebhookBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWebhookBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWebhookBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWebhookBody>>();
        let type_size = std::mem::size_of::<CreateWebhookBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWebhookBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWebhookBody),
            type_size
        );
    }
    #[test]
    fn test_field_description() {
        let instance = CreateWebhookBody::default();
        let _: Option<String> = instance.description;
    }
    #[test]
    fn test_field_events() {
        let instance = CreateWebhookBody::default();
        let _: Vec<Event> = instance.events;
    }
    #[test]
    fn test_field_status() {
        let instance = CreateWebhookBody::default();
        let _: Option<Status> = instance.status;
    }
    #[test]
    fn test_field_url() {
        let instance = CreateWebhookBody::default();
        let _: String = instance.url;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWebhookBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_event {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Event::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Event::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Event::default();
        let b = Event::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Event::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Event = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Event::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Event = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Event::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Event =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Event::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Event>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Event>();
        let align = std::mem::align_of::<Event>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Event));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Event::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Event>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Event>>();
        let type_size = std::mem::size_of::<Event>();
        println!("Option<{}> size: {} bytes", stringify!(Event), option_size);
        println!("Raw {} size: {} bytes", stringify!(Event), type_size);
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
mod test_createwebhookresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWebhookResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWebhookResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWebhookResponse::default();
        let b = CreateWebhookResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWebhookResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWebhookResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWebhookResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWebhookResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWebhookResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWebhookResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWebhookResponse>();
        let align = std::mem::align_of::<CreateWebhookResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWebhookResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWebhookResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWebhookResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWebhookResponse>>();
        let type_size = std::mem::size_of::<CreateWebhookResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWebhookResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWebhookResponse),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = CreateWebhookResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_updated() {
        let instance = CreateWebhookResponse::default();
        let _: String = instance.date_updated;
    }
    #[test]
    fn test_field_description() {
        let instance = CreateWebhookResponse::default();
        let _: Option<String> = instance.description;
    }
    #[test]
    fn test_field_events() {
        let instance = CreateWebhookResponse::default();
        let _: Vec<Event> = instance.events;
    }
    #[test]
    fn test_field_id() {
        let instance = CreateWebhookResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_secret() {
        let instance = CreateWebhookResponse::default();
        let _: String = instance.secret;
    }
    #[test]
    fn test_field_status() {
        let instance = CreateWebhookResponse::default();
        let _: Status = instance.status;
    }
    #[test]
    fn test_field_url() {
        let instance = CreateWebhookResponse::default();
        let _: String = instance.url;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWebhookResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwebhookrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWebhookRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWebhookRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWebhookRequest::default();
        let b = CreateWebhookRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWebhookRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWebhookRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWebhookRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWebhookRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWebhookRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWebhookRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWebhookRequest>();
        let align = std::mem::align_of::<CreateWebhookRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWebhookRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWebhookRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWebhookRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWebhookRequest>>();
        let type_size = std::mem::size_of::<CreateWebhookRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWebhookRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWebhookRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = CreateWebhookRequest::default();
        let _: CreateWebhookRequestBody = instance.body;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWebhookRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_createwebhookrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = CreateWebhookRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = CreateWebhookRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = CreateWebhookRequestBody::default();
        let b = CreateWebhookRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = CreateWebhookRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: CreateWebhookRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = CreateWebhookRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: CreateWebhookRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = CreateWebhookRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: CreateWebhookRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = CreateWebhookRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<CreateWebhookRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<CreateWebhookRequestBody>();
        let align = std::mem::align_of::<CreateWebhookRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(CreateWebhookRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = CreateWebhookRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<CreateWebhookRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<CreateWebhookRequestBody>>();
        let type_size = std::mem::size_of::<CreateWebhookRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(CreateWebhookRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(CreateWebhookRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_description() {
        let instance = CreateWebhookRequestBody::default();
        let _: Option<String> = instance.description;
    }
    #[test]
    fn test_field_events() {
        let instance = CreateWebhookRequestBody::default();
        let _: Vec<Event> = instance.events;
    }
    #[test]
    fn test_field_status() {
        let instance = CreateWebhookRequestBody::default();
        let _: Option<Status> = instance.status;
    }
    #[test]
    fn test_field_url() {
        let instance = CreateWebhookRequestBody::default();
        let _: String = instance.url;
    }
    #[test]
    fn check_field_attributes() {
        let instance = CreateWebhookRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_deletewebhookparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DeleteWebhookParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DeleteWebhookParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DeleteWebhookParams::default();
        let b = DeleteWebhookParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DeleteWebhookParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DeleteWebhookParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DeleteWebhookParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DeleteWebhookParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DeleteWebhookParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DeleteWebhookParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DeleteWebhookParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DeleteWebhookParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DeleteWebhookParams>();
        let align = std::mem::align_of::<DeleteWebhookParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DeleteWebhookParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DeleteWebhookParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DeleteWebhookParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DeleteWebhookParams>>();
        let type_size = std::mem::size_of::<DeleteWebhookParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DeleteWebhookParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DeleteWebhookParams),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = DeleteWebhookParams::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DeleteWebhookParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_deletewebhookresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DeleteWebhookResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DeleteWebhookResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DeleteWebhookResponse::default();
        let b = DeleteWebhookResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DeleteWebhookResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DeleteWebhookResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DeleteWebhookResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DeleteWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DeleteWebhookResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DeleteWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DeleteWebhookResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DeleteWebhookResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DeleteWebhookResponse>();
        let align = std::mem::align_of::<DeleteWebhookResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DeleteWebhookResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DeleteWebhookResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DeleteWebhookResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DeleteWebhookResponse>>();
        let type_size = std::mem::size_of::<DeleteWebhookResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DeleteWebhookResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DeleteWebhookResponse),
            type_size
        );
    }
    #[test]
    fn test_field_deleted() {
        let instance = DeleteWebhookResponse::default();
        let _: bool = instance.deleted;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DeleteWebhookResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_deletewebhookrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DeleteWebhookRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DeleteWebhookRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DeleteWebhookRequest::default();
        let b = DeleteWebhookRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DeleteWebhookRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DeleteWebhookRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DeleteWebhookRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DeleteWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DeleteWebhookRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DeleteWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DeleteWebhookRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DeleteWebhookRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DeleteWebhookRequest>();
        let align = std::mem::align_of::<DeleteWebhookRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DeleteWebhookRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DeleteWebhookRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DeleteWebhookRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DeleteWebhookRequest>>();
        let type_size = std::mem::size_of::<DeleteWebhookRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DeleteWebhookRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DeleteWebhookRequest),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = DeleteWebhookRequest::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = DeleteWebhookRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwebhookparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWebhookParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWebhookParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWebhookParams::default();
        let b = GetWebhookParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWebhookParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWebhookParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWebhookParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWebhookParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWebhookParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWebhookParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWebhookParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWebhookParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWebhookParams>();
        let align = std::mem::align_of::<GetWebhookParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWebhookParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWebhookParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWebhookParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWebhookParams>>();
        let type_size = std::mem::size_of::<GetWebhookParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWebhookParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWebhookParams),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = GetWebhookParams::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWebhookParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwebhookresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWebhookResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWebhookResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWebhookResponse::default();
        let b = GetWebhookResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWebhookResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWebhookResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWebhookResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWebhookResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWebhookResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWebhookResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWebhookResponse>();
        let align = std::mem::align_of::<GetWebhookResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWebhookResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWebhookResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWebhookResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWebhookResponse>>();
        let type_size = std::mem::size_of::<GetWebhookResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWebhookResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWebhookResponse),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = GetWebhookResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_updated() {
        let instance = GetWebhookResponse::default();
        let _: String = instance.date_updated;
    }
    #[test]
    fn test_field_description() {
        let instance = GetWebhookResponse::default();
        let _: Option<String> = instance.description;
    }
    #[test]
    fn test_field_events() {
        let instance = GetWebhookResponse::default();
        let _: Vec<Event> = instance.events;
    }
    #[test]
    fn test_field_id() {
        let instance = GetWebhookResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_status() {
        let instance = GetWebhookResponse::default();
        let _: Status = instance.status;
    }
    #[test]
    fn test_field_url() {
        let instance = GetWebhookResponse::default();
        let _: String = instance.url;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWebhookResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwebhookrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWebhookRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWebhookRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWebhookRequest::default();
        let b = GetWebhookRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWebhookRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWebhookRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWebhookRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWebhookRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWebhookRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWebhookRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWebhookRequest>();
        let align = std::mem::align_of::<GetWebhookRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWebhookRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWebhookRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWebhookRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWebhookRequest>>();
        let type_size = std::mem::size_of::<GetWebhookRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWebhookRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWebhookRequest),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = GetWebhookRequest::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWebhookRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwebhookeventparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWebhookEventParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWebhookEventParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWebhookEventParams::default();
        let b = GetWebhookEventParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWebhookEventParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWebhookEventParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWebhookEventParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWebhookEventParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWebhookEventParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWebhookEventParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWebhookEventParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWebhookEventParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWebhookEventParams>();
        let align = std::mem::align_of::<GetWebhookEventParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWebhookEventParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWebhookEventParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWebhookEventParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWebhookEventParams>>();
        let type_size = std::mem::size_of::<GetWebhookEventParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWebhookEventParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWebhookEventParams),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_event_id() {
        let instance = GetWebhookEventParams::default();
        let _: String = instance.webhook_event_id;
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = GetWebhookEventParams::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWebhookEventParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_getwebhookeventresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWebhookEventResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWebhookEventResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWebhookEventResponse::default();
        let b = GetWebhookEventResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWebhookEventResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWebhookEventResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWebhookEventResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWebhookEventResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWebhookEventResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWebhookEventResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWebhookEventResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWebhookEventResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWebhookEventResponse>();
        let align = std::mem::align_of::<GetWebhookEventResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWebhookEventResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWebhookEventResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWebhookEventResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWebhookEventResponse>>();
        let type_size = std::mem::size_of::<GetWebhookEventResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWebhookEventResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWebhookEventResponse),
            type_size
        );
    }
    #[test]
    fn test_field_data() {
        let instance = GetWebhookEventResponse::default();
        let _: HashMap<String, Option<serde_json::Value>> = instance.data;
    }
    #[test]
    fn test_field_date() {
        let instance = GetWebhookEventResponse::default();
        let _: String = instance.date;
    }
    #[test]
    fn test_field_error() {
        let instance = GetWebhookEventResponse::default();
        let _: Option<String> = instance.error;
    }
    #[test]
    fn test_field_id() {
        let instance = GetWebhookEventResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = GetWebhookEventResponse::default();
        let _: Kind = instance.kind;
    }
    #[test]
    fn test_field_status() {
        let instance = GetWebhookEventResponse::default();
        let _: String = instance.status;
    }
    #[test]
    fn test_field_timestamp_sent() {
        let instance = GetWebhookEventResponse::default();
        let _: f64 = instance.timestamp_sent;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWebhookEventResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_kind {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = Kind::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = Kind::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = Kind::default();
        let b = Kind::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = Kind::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: Kind = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = Kind::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Kind = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = Kind::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: Kind =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = Kind::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<Kind>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<Kind>();
        let align = std::mem::align_of::<Kind>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(Kind));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = Kind::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<Kind>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<Kind>>();
        let type_size = std::mem::size_of::<Kind>();
        println!("Option<{}> size: {} bytes", stringify!(Kind), option_size);
        println!("Raw {} size: {} bytes", stringify!(Kind), type_size);
    }
}
#[cfg(test)]
mod test_getwebhookeventrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = GetWebhookEventRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = GetWebhookEventRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = GetWebhookEventRequest::default();
        let b = GetWebhookEventRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = GetWebhookEventRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: GetWebhookEventRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = GetWebhookEventRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: GetWebhookEventRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = GetWebhookEventRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: GetWebhookEventRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = GetWebhookEventRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<GetWebhookEventRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<GetWebhookEventRequest>();
        let align = std::mem::align_of::<GetWebhookEventRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(GetWebhookEventRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = GetWebhookEventRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<GetWebhookEventRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<GetWebhookEventRequest>>();
        let type_size = std::mem::size_of::<GetWebhookEventRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(GetWebhookEventRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(GetWebhookEventRequest),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_event_id() {
        let instance = GetWebhookEventRequest::default();
        let _: String = instance.webhook_event_id;
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = GetWebhookEventRequest::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = GetWebhookEventRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhookeventsparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhookEventsParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhookEventsParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhookEventsParams::default();
        let b = ListWebhookEventsParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhookEventsParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhookEventsParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhookEventsParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhookEventsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhookEventsParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhookEventsParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhookEventsParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhookEventsParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhookEventsParams>();
        let align = std::mem::align_of::<ListWebhookEventsParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhookEventsParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhookEventsParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhookEventsParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhookEventsParams>>();
        let type_size = std::mem::size_of::<ListWebhookEventsParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhookEventsParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhookEventsParams),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = ListWebhookEventsParams::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhookEventsParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhookeventsquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhookEventsQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhookEventsQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhookEventsQuery::default();
        let b = ListWebhookEventsQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhookEventsQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhookEventsQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhookEventsQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhookEventsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhookEventsQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhookEventsQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhookEventsQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhookEventsQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhookEventsQuery>();
        let align = std::mem::align_of::<ListWebhookEventsQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhookEventsQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhookEventsQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhookEventsQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhookEventsQuery>>();
        let type_size = std::mem::size_of::<ListWebhookEventsQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhookEventsQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhookEventsQuery),
            type_size
        );
    }
    #[test]
    fn test_field_delivery_failed() {
        let instance = ListWebhookEventsQuery::default();
        let _: Option<DeliveryFailed> = instance.delivery_failed;
    }
    #[test]
    fn test_field_kind() {
        let instance = ListWebhookEventsQuery::default();
        let _: Option<Kind> = instance.kind;
    }
    #[test]
    fn test_field_limit() {
        let instance = ListWebhookEventsQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListWebhookEventsQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhookEventsQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_deliveryfailed {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = DeliveryFailed::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = DeliveryFailed::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = DeliveryFailed::default();
        let b = DeliveryFailed::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = DeliveryFailed::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: DeliveryFailed = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = DeliveryFailed::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: DeliveryFailed =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = DeliveryFailed::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: DeliveryFailed =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = DeliveryFailed::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<DeliveryFailed>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<DeliveryFailed>();
        let align = std::mem::align_of::<DeliveryFailed>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(DeliveryFailed));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = DeliveryFailed::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<DeliveryFailed>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<DeliveryFailed>>();
        let type_size = std::mem::size_of::<DeliveryFailed>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(DeliveryFailed),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(DeliveryFailed),
            type_size
        );
    }
}
#[cfg(test)]
mod test_listwebhookeventsresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhookEventsResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhookEventsResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhookEventsResponse::default();
        let b = ListWebhookEventsResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhookEventsResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhookEventsResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhookEventsResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhookEventsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhookEventsResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhookEventsResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhookEventsResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhookEventsResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhookEventsResponse>();
        let align = std::mem::align_of::<ListWebhookEventsResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhookEventsResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhookEventsResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhookEventsResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhookEventsResponse>>();
        let type_size = std::mem::size_of::<ListWebhookEventsResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhookEventsResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhookEventsResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListWebhookEventsResponse::default();
        let _: Vec<ListWebhookEventsResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListWebhookEventsResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhookEventsResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhookeventsresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhookEventsResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhookEventsResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhookEventsResponseItem::default();
        let b = ListWebhookEventsResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhookEventsResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhookEventsResponseItem =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhookEventsResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhookEventsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhookEventsResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhookEventsResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhookEventsResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhookEventsResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhookEventsResponseItem>();
        let align = std::mem::align_of::<ListWebhookEventsResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ListWebhookEventsResponseItem)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhookEventsResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhookEventsResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhookEventsResponseItem>>();
        let type_size = std::mem::size_of::<ListWebhookEventsResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhookEventsResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhookEventsResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_data() {
        let instance = ListWebhookEventsResponseItem::default();
        let _: HashMap<String, Option<serde_json::Value>> = instance.data;
    }
    #[test]
    fn test_field_date() {
        let instance = ListWebhookEventsResponseItem::default();
        let _: String = instance.date;
    }
    #[test]
    fn test_field_error() {
        let instance = ListWebhookEventsResponseItem::default();
        let _: Option<String> = instance.error;
    }
    #[test]
    fn test_field_id() {
        let instance = ListWebhookEventsResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_kind() {
        let instance = ListWebhookEventsResponseItem::default();
        let _: Kind = instance.kind;
    }
    #[test]
    fn test_field_status() {
        let instance = ListWebhookEventsResponseItem::default();
        let _: String = instance.status;
    }
    #[test]
    fn test_field_timestamp_sent() {
        let instance = ListWebhookEventsResponseItem::default();
        let _: f64 = instance.timestamp_sent;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhookEventsResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhookeventsrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhookEventsRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhookEventsRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhookEventsRequest::default();
        let b = ListWebhookEventsRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhookEventsRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhookEventsRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhookEventsRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhookEventsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhookEventsRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhookEventsRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhookEventsRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhookEventsRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhookEventsRequest>();
        let align = std::mem::align_of::<ListWebhookEventsRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhookEventsRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhookEventsRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhookEventsRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhookEventsRequest>>();
        let type_size = std::mem::size_of::<ListWebhookEventsRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhookEventsRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhookEventsRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListWebhookEventsRequest::default();
        let _: Option<ListWebhookEventsRequestQuery> = instance.query;
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = ListWebhookEventsRequest::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhookEventsRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhookeventsrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhookEventsRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhookEventsRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhookEventsRequestQuery::default();
        let b = ListWebhookEventsRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhookEventsRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhookEventsRequestQuery =
            serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhookEventsRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhookEventsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhookEventsRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhookEventsRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhookEventsRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhookEventsRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhookEventsRequestQuery>();
        let align = std::mem::align_of::<ListWebhookEventsRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!(
            "Type {} metrics:",
            stringify!(ListWebhookEventsRequestQuery)
        );
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhookEventsRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhookEventsRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhookEventsRequestQuery>>();
        let type_size = std::mem::size_of::<ListWebhookEventsRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhookEventsRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhookEventsRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_delivery_failed() {
        let instance = ListWebhookEventsRequestQuery::default();
        let _: Option<DeliveryFailed> = instance.delivery_failed;
    }
    #[test]
    fn test_field_kind() {
        let instance = ListWebhookEventsRequestQuery::default();
        let _: Option<Kind> = instance.kind;
    }
    #[test]
    fn test_field_limit() {
        let instance = ListWebhookEventsRequestQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListWebhookEventsRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhookEventsRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhooksquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhooksQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhooksQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhooksQuery::default();
        let b = ListWebhooksQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhooksQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhooksQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhooksQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhooksQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhooksQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhooksQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhooksQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhooksQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhooksQuery>();
        let align = std::mem::align_of::<ListWebhooksQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhooksQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhooksQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhooksQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhooksQuery>>();
        let type_size = std::mem::size_of::<ListWebhooksQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhooksQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhooksQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListWebhooksQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListWebhooksQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhooksQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhooksresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhooksResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhooksResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhooksResponse::default();
        let b = ListWebhooksResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhooksResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhooksResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhooksResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhooksResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhooksResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhooksResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhooksResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhooksResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhooksResponse>();
        let align = std::mem::align_of::<ListWebhooksResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhooksResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhooksResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhooksResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhooksResponse>>();
        let type_size = std::mem::size_of::<ListWebhooksResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhooksResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhooksResponse),
            type_size
        );
    }
    #[test]
    fn test_field_items() {
        let instance = ListWebhooksResponse::default();
        let _: Vec<ListWebhooksResponseItem> = instance.items;
    }
    #[test]
    fn test_field_next_page_token() {
        let instance = ListWebhooksResponse::default();
        let _: Option<String> = instance.next_page_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhooksResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhooksresponseitem {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhooksResponseItem::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhooksResponseItem::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhooksResponseItem::default();
        let b = ListWebhooksResponseItem::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhooksResponseItem::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhooksResponseItem = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhooksResponseItem::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhooksResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhooksResponseItem::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhooksResponseItem =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhooksResponseItem::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhooksResponseItem>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhooksResponseItem>();
        let align = std::mem::align_of::<ListWebhooksResponseItem>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhooksResponseItem));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhooksResponseItem::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhooksResponseItem>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhooksResponseItem>>();
        let type_size = std::mem::size_of::<ListWebhooksResponseItem>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhooksResponseItem),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhooksResponseItem),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = ListWebhooksResponseItem::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_updated() {
        let instance = ListWebhooksResponseItem::default();
        let _: String = instance.date_updated;
    }
    #[test]
    fn test_field_description() {
        let instance = ListWebhooksResponseItem::default();
        let _: Option<String> = instance.description;
    }
    #[test]
    fn test_field_events() {
        let instance = ListWebhooksResponseItem::default();
        let _: Vec<Event> = instance.events;
    }
    #[test]
    fn test_field_id() {
        let instance = ListWebhooksResponseItem::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_status() {
        let instance = ListWebhooksResponseItem::default();
        let _: Status = instance.status;
    }
    #[test]
    fn test_field_url() {
        let instance = ListWebhooksResponseItem::default();
        let _: String = instance.url;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhooksResponseItem::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhooksrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhooksRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhooksRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhooksRequest::default();
        let b = ListWebhooksRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhooksRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhooksRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhooksRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhooksRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhooksRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhooksRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhooksRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhooksRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhooksRequest>();
        let align = std::mem::align_of::<ListWebhooksRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhooksRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhooksRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhooksRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhooksRequest>>();
        let type_size = std::mem::size_of::<ListWebhooksRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhooksRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhooksRequest),
            type_size
        );
    }
    #[test]
    fn test_field_query() {
        let instance = ListWebhooksRequest::default();
        let _: Option<ListWebhooksRequestQuery> = instance.query;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhooksRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_listwebhooksrequestquery {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = ListWebhooksRequestQuery::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = ListWebhooksRequestQuery::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = ListWebhooksRequestQuery::default();
        let b = ListWebhooksRequestQuery::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = ListWebhooksRequestQuery::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: ListWebhooksRequestQuery = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = ListWebhooksRequestQuery::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: ListWebhooksRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = ListWebhooksRequestQuery::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: ListWebhooksRequestQuery =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = ListWebhooksRequestQuery::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<ListWebhooksRequestQuery>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<ListWebhooksRequestQuery>();
        let align = std::mem::align_of::<ListWebhooksRequestQuery>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(ListWebhooksRequestQuery));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = ListWebhooksRequestQuery::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<ListWebhooksRequestQuery>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<ListWebhooksRequestQuery>>();
        let type_size = std::mem::size_of::<ListWebhooksRequestQuery>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(ListWebhooksRequestQuery),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(ListWebhooksRequestQuery),
            type_size
        );
    }
    #[test]
    fn test_field_limit() {
        let instance = ListWebhooksRequestQuery::default();
        let _: Option<f64> = instance.limit;
    }
    #[test]
    fn test_field_pagination_token() {
        let instance = ListWebhooksRequestQuery::default();
        let _: Option<String> = instance.pagination_token;
    }
    #[test]
    fn check_field_attributes() {
        let instance = ListWebhooksRequestQuery::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_pingwebhookparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PingWebhookParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PingWebhookParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PingWebhookParams::default();
        let b = PingWebhookParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PingWebhookParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PingWebhookParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PingWebhookParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PingWebhookParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PingWebhookParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PingWebhookParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PingWebhookParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PingWebhookParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PingWebhookParams>();
        let align = std::mem::align_of::<PingWebhookParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PingWebhookParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PingWebhookParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PingWebhookParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PingWebhookParams>>();
        let type_size = std::mem::size_of::<PingWebhookParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PingWebhookParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(PingWebhookParams),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = PingWebhookParams::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PingWebhookParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_pingwebhookresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PingWebhookResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PingWebhookResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PingWebhookResponse::default();
        let b = PingWebhookResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PingWebhookResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PingWebhookResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PingWebhookResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PingWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PingWebhookResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PingWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PingWebhookResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PingWebhookResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PingWebhookResponse>();
        let align = std::mem::align_of::<PingWebhookResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PingWebhookResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PingWebhookResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PingWebhookResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PingWebhookResponse>>();
        let type_size = std::mem::size_of::<PingWebhookResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PingWebhookResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(PingWebhookResponse),
            type_size
        );
    }
    #[test]
    fn test_field_error() {
        let instance = PingWebhookResponse::default();
        let _: Option<String> = instance.error;
    }
    #[test]
    fn test_field_status() {
        let instance = PingWebhookResponse::default();
        let _: String = instance.status;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PingWebhookResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_pingwebhookrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = PingWebhookRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = PingWebhookRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = PingWebhookRequest::default();
        let b = PingWebhookRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = PingWebhookRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: PingWebhookRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = PingWebhookRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PingWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = PingWebhookRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: PingWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = PingWebhookRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<PingWebhookRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<PingWebhookRequest>();
        let align = std::mem::align_of::<PingWebhookRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(PingWebhookRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = PingWebhookRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<PingWebhookRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<PingWebhookRequest>>();
        let type_size = std::mem::size_of::<PingWebhookRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(PingWebhookRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(PingWebhookRequest),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = PingWebhookRequest::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = PingWebhookRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewebhookbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWebhookBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWebhookBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWebhookBody::default();
        let b = UpdateWebhookBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWebhookBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWebhookBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWebhookBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWebhookBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWebhookBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWebhookBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWebhookBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWebhookBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWebhookBody>();
        let align = std::mem::align_of::<UpdateWebhookBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWebhookBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWebhookBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWebhookBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWebhookBody>>();
        let type_size = std::mem::size_of::<UpdateWebhookBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWebhookBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWebhookBody),
            type_size
        );
    }
    #[test]
    fn test_field_description() {
        let instance = UpdateWebhookBody::default();
        let _: Option<String> = instance.description;
    }
    #[test]
    fn test_field_events() {
        let instance = UpdateWebhookBody::default();
        let _: Option<Vec<Event>> = instance.events;
    }
    #[test]
    fn test_field_status() {
        let instance = UpdateWebhookBody::default();
        let _: Option<Status> = instance.status;
    }
    #[test]
    fn test_field_url() {
        let instance = UpdateWebhookBody::default();
        let _: Option<String> = instance.url;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWebhookBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewebhookparams {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWebhookParams::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWebhookParams::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWebhookParams::default();
        let b = UpdateWebhookParams::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWebhookParams::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWebhookParams = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWebhookParams::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWebhookParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWebhookParams::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWebhookParams =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWebhookParams::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWebhookParams>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWebhookParams>();
        let align = std::mem::align_of::<UpdateWebhookParams>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWebhookParams));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWebhookParams::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWebhookParams>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWebhookParams>>();
        let type_size = std::mem::size_of::<UpdateWebhookParams>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWebhookParams),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWebhookParams),
            type_size
        );
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = UpdateWebhookParams::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWebhookParams::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewebhookresponse {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWebhookResponse::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWebhookResponse::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWebhookResponse::default();
        let b = UpdateWebhookResponse::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWebhookResponse::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWebhookResponse = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWebhookResponse::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWebhookResponse::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWebhookResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWebhookResponse::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWebhookResponse>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWebhookResponse>();
        let align = std::mem::align_of::<UpdateWebhookResponse>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWebhookResponse));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWebhookResponse::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWebhookResponse>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWebhookResponse>>();
        let type_size = std::mem::size_of::<UpdateWebhookResponse>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWebhookResponse),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWebhookResponse),
            type_size
        );
    }
    #[test]
    fn test_field_date_created() {
        let instance = UpdateWebhookResponse::default();
        let _: String = instance.date_created;
    }
    #[test]
    fn test_field_date_updated() {
        let instance = UpdateWebhookResponse::default();
        let _: String = instance.date_updated;
    }
    #[test]
    fn test_field_description() {
        let instance = UpdateWebhookResponse::default();
        let _: Option<String> = instance.description;
    }
    #[test]
    fn test_field_events() {
        let instance = UpdateWebhookResponse::default();
        let _: Vec<Event> = instance.events;
    }
    #[test]
    fn test_field_id() {
        let instance = UpdateWebhookResponse::default();
        let _: String = instance.id;
    }
    #[test]
    fn test_field_status() {
        let instance = UpdateWebhookResponse::default();
        let _: Status = instance.status;
    }
    #[test]
    fn test_field_url() {
        let instance = UpdateWebhookResponse::default();
        let _: String = instance.url;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWebhookResponse::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewebhookrequest {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWebhookRequest::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWebhookRequest::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWebhookRequest::default();
        let b = UpdateWebhookRequest::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWebhookRequest::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWebhookRequest = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWebhookRequest::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWebhookRequest::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWebhookRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWebhookRequest::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWebhookRequest>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWebhookRequest>();
        let align = std::mem::align_of::<UpdateWebhookRequest>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWebhookRequest));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWebhookRequest::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWebhookRequest>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWebhookRequest>>();
        let type_size = std::mem::size_of::<UpdateWebhookRequest>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWebhookRequest),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWebhookRequest),
            type_size
        );
    }
    #[test]
    fn test_field_body() {
        let instance = UpdateWebhookRequest::default();
        let _: UpdateWebhookRequestBody = instance.body;
    }
    #[test]
    fn test_field_webhook_id() {
        let instance = UpdateWebhookRequest::default();
        let _: String = instance.webhook_id;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWebhookRequest::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
#[cfg(test)]
mod test_updatewebhookrequestbody {
    use super::*;
    #[test]
    fn implements_debug() {
        let instance = UpdateWebhookRequestBody::default();
        let _result = format!("{:?}", instance);
    }
    #[test]
    fn implements_clone() {
        let original = UpdateWebhookRequestBody::default();
        let cloned = original.clone();
        assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
    }
    #[test]
    fn implements_partialeq() {
        let a = UpdateWebhookRequestBody::default();
        let b = UpdateWebhookRequestBody::default();
        assert_eq!(a, b);
    }
    #[test]
    fn implements_serde_traits() {
        let instance = UpdateWebhookRequestBody::default();
        let serialized = serde_json::to_string(&instance).unwrap();
        let _deserialized: UpdateWebhookRequestBody = serde_json::from_str(&serialized).unwrap();
    }
    #[test]
    fn serialization_roundtrip() {
        let original = UpdateWebhookRequestBody::default();
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UpdateWebhookRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
    #[test]
    fn serialization_pretty_format() {
        let value = UpdateWebhookRequestBody::default();
        let serialized =
            serde_json::to_string_pretty(&value).expect("Failed to serialize with pretty format");
        let deserialized: UpdateWebhookRequestBody =
            serde_json::from_str(&serialized).expect("Failed to deserialize from pretty format");
        assert_eq!(value, deserialized);
    }
    #[test]
    fn serialization_json_schema() {
        let value = UpdateWebhookRequestBody::default();
        let serialized = serde_json::to_value(&value).expect("Failed to convert to JSON value");
        assert!(
            serialized.is_object() || serialized.is_array(),
            "Serialized value must be a JSON object or array"
        );
    }
    #[test]
    fn serialization_error_handling() {
        let invalid_json = r#"{"invalid": json"#;
        let result = serde_json::from_str::<UpdateWebhookRequestBody>(invalid_json);
        assert!(result.is_err(), "Should fail with invalid JSON");
    }
    #[test]
    fn check_size_and_alignment() {
        let size = std::mem::size_of::<UpdateWebhookRequestBody>();
        let align = std::mem::align_of::<UpdateWebhookRequestBody>();
        assert!(size > 0, "Type should have non-zero size");
        assert!(align > 0, "Type should have non-zero alignment");
        assert!(size % align == 0, "Size should be a multiple of alignment");
        println!("Type {} metrics:", stringify!(UpdateWebhookRequestBody));
        println!("  - Size: {} bytes", size);
        println!("  - Alignment: {} bytes", align);
    }
    #[test]
    fn check_default_instance_size() {
        let instance = UpdateWebhookRequestBody::default();
        let instance_size = std::mem::size_of_val(&instance);
        let type_size = std::mem::size_of::<UpdateWebhookRequestBody>();
        assert_eq!(
            instance_size, type_size,
            "Instance size should match type size"
        );
    }
    #[test]
    fn check_option_size() {
        let option_size = std::mem::size_of::<Option<UpdateWebhookRequestBody>>();
        let type_size = std::mem::size_of::<UpdateWebhookRequestBody>();
        println!(
            "Option<{}> size: {} bytes",
            stringify!(UpdateWebhookRequestBody),
            option_size
        );
        println!(
            "Raw {} size: {} bytes",
            stringify!(UpdateWebhookRequestBody),
            type_size
        );
    }
    #[test]
    fn test_field_description() {
        let instance = UpdateWebhookRequestBody::default();
        let _: Option<String> = instance.description;
    }
    #[test]
    fn test_field_events() {
        let instance = UpdateWebhookRequestBody::default();
        let _: Option<Vec<Event>> = instance.events;
    }
    #[test]
    fn test_field_status() {
        let instance = UpdateWebhookRequestBody::default();
        let _: Option<Status> = instance.status;
    }
    #[test]
    fn test_field_url() {
        let instance = UpdateWebhookRequestBody::default();
        let _: Option<String> = instance.url;
    }
    #[test]
    fn check_field_attributes() {
        let instance = UpdateWebhookRequestBody::default();
        let fields = std::mem::size_of_val(&instance);
        assert!(fields > 0, "Type should have valid fields");
    }
}
