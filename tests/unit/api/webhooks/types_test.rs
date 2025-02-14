/// @dfns-sdk-rs/tests/unit/api/webhooks/types_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::api::webhooks::types::{
        CreateWebhookBody, CreateWebhookRequest, CreateWebhookResponse, DeliveryFailed, Event, Kind,
        ListWebhookEventsResponse, ListWebhookEventsResponseItem, Status, UpdateWebhookBody,
    };
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_event_serialization() {
        let events = vec![
            (Event::Empty, "*"),
            (Event::PolicyApprovalPending, "policy.approval.pending"),
            (Event::WalletCreated, "wallet.created"),
            (Event::WalletSignatureSigned, "wallet.signature.signed"),
        ];

        for (event, expected) in events {
            let serialized = serde_json::to_string(&event).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: Event = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, event);
        }
    }

    #[test]
    fn test_status_serialization() {
        let statuses = vec![
            (Status::Enabled, "Enabled"),
            (Status::Disabled, "Disabled"),
        ];

        for (status, expected) in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected));

            let deserialized: Status = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, status);
        }
    }

    #[test]
    fn test_create_webhook_body() {
        let body = CreateWebhookBody {
            description: Some("Test webhook".to_string()),
            events: vec![Event::WalletCreated, Event::WalletSignatureSigned],
            status: Some(Status::Enabled),
            url: "https://example.com/webhook".to_string(),
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateWebhookBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_create_webhook_response() {
        let response = CreateWebhookResponse {
            date_created: "2024-01-01T00:00:00Z".to_string(),
            date_updated: "2024-01-01T00:00:00Z".to_string(),
            description: Some("Test webhook".to_string()),
            events: vec![Event::WalletCreated],
            id: "webhook-123".to_string(),
            secret: "secret-123".to_string(),
            status: Status::Enabled,
            url: "https://example.com/webhook".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: CreateWebhookResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_delivery_failed_display() {
        assert_eq!(DeliveryFailed::True.to_string(), "true");
        assert_eq!(DeliveryFailed::False.to_string(), "false");
    }

    #[test]
    fn test_kind_display() {
        let kinds = vec![
            (Kind::PolicyApprovalPending, "policy.approval.pending"),
            (Kind::WalletCreated, "wallet.created"),
            (Kind::WalletSignatureSigned, "wallet.signature.signed"),
        ];

        for (kind, expected) in kinds {
            assert_eq!(kind.to_string(), expected);
        }
    }

    #[test]
    fn test_list_webhook_events_response() {
        let mut data = HashMap::new();
        data.insert("key1".to_string(), Some(json!("value1")));
        data.insert("key2".to_string(), None);

        let item = ListWebhookEventsResponseItem {
            data: data.clone(),
            date: "2024-01-01T00:00:00Z".to_string(),
            error: None,
            id: "event-123".to_string(),
            kind: Kind::WalletCreated,
            status: "200".to_string(),
            timestamp_sent: 1704067200.0,
        };

        let response = ListWebhookEventsResponse {
            items: vec![item],
            next_page_token: Some("next-page-token".to_string()),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ListWebhookEventsResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, response);
    }

    #[test]
    fn test_update_webhook_body() {
        let body = UpdateWebhookBody {
            description: Some("Updated webhook".to_string()),
            events: Some(vec![Event::WalletCreated]),
            status: Some(Status::Disabled),
            url: Some("https://example.com/updated-webhook".to_string()),
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: UpdateWebhookBody = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, body);
    }

    #[test]
    fn test_create_webhook_request() {
        let request = CreateWebhookRequest {
            body: CreateWebhookBody {
                description: Some("Test webhook".to_string()),
                events: vec![Event::WalletCreated],
                status: Some(Status::Enabled),
                url: "https://example.com/webhook".to_string(),
            },
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateWebhookRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, request);
    }
}
