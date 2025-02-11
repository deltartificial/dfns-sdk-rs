// @dfns-sdk-rs/src/error.rs

use serde::Serialize;
use std::error::Error;
use std::fmt;
use reqwest::Error as ReqwestError;
use url::ParseError;
use reqwest::header::InvalidHeaderValue;

#[derive(Debug, Serialize)]
pub struct DfnsError {
    pub http_status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

impl DfnsError {
    pub fn new(
        http_status: u16,
        message: impl Into<String>,
        context: Option<serde_json::Value>,
    ) -> Self {
        Self {
            http_status,
            message: message.into(),
            context,
        }
    }
}

impl fmt::Display for DfnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_json = serde_json::json!({
            "httpStatus": self.http_status,
            "message": self.message,
            "context": self.context,
        });
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&error_json).unwrap_or_default()
        )
    }
}

impl Error for DfnsError {}

#[derive(Debug, Serialize)]
pub struct PolicyPendingError {
    pub http_status: u16,
    pub message: String,
    pub context: Option<serde_json::Value>,
}

impl PolicyPendingError {
    pub const HTTP_ACCEPTED: u16 = 202;

    pub fn new(context: Option<serde_json::Value>) -> Self {
        Self {
            http_status: Self::HTTP_ACCEPTED,
            message: "Operation triggered a policy pending approval".to_string(),
            context,
        }
    }
}

impl fmt::Display for PolicyPendingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_json = serde_json::json!({
            "httpStatus": self.http_status,
            "message": self.message,
            "context": self.context,
        });
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&error_json).unwrap_or_default()
        )
    }
}

impl Error for PolicyPendingError {}

impl From<PolicyPendingError> for DfnsError {
    fn from(err: PolicyPendingError) -> Self {
        Self::new(err.http_status, err.message, err.context)
    }
}

impl From<ReqwestError> for DfnsError {
    fn from(err: ReqwestError) -> Self {
        Self::new(
            err.status()
                .map(|s| s.as_u16())
                .unwrap_or(500),
            err.to_string(),
            None,
        )
    }
}

impl From<ParseError> for DfnsError {
    fn from(err: ParseError) -> Self {
        Self::new(400, format!("URL parsing error: {}", err), None)
    }
}

impl From<InvalidHeaderValue> for DfnsError {
    fn from(err: InvalidHeaderValue) -> Self {
        Self::new(400, format!("Invalid header value: {}", err), None)
    }
}

impl From<serde_json::Error> for DfnsError {
    fn from(err: serde_json::Error) -> Self {
        Self::new(400, format!("JSON serialization error: {}", err), None)
    }
}
