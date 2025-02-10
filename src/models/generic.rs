// @dfns-sdk-rs/src/models/generic.rs

use serde::{Deserialize, Serialize};
use crate::signer::CredentialSigner;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DfnsBaseApiOptions {
    pub app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DfnsApiClientOptions {
    #[serde(flatten)]
    pub base: DfnsBaseApiOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<Box<dyn CredentialSigner>>,
}

impl DfnsApiClientOptions {
    pub fn new(base: DfnsBaseApiOptions) -> Self {
        Self {
            base,
            signer: None,
        }
    }

    pub fn with_signer(mut self, signer: Box<dyn CredentialSigner>) -> Self {
        self.signer = Some(signer);
        self
    }
}
