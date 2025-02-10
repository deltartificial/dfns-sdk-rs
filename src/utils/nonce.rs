// @dfns-sdk-rs/src/utils/nonce.rs

use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;
use crate::utils::base64;

#[derive(Serialize)]
struct NonceData {
    uuid: String,
    date: String,
}

pub fn generate_nonce() -> String {
    let nonce_data = NonceData {
        uuid: Uuid::new_v4().to_string(),
        date: Utc::now().to_rfc3339(),
    };

    let json = serde_json::to_string(&nonce_data)
        .expect("Failed to serialize nonce data");
        
    base64::to_base64_url(json)
}