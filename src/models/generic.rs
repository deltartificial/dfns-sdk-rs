// @dfns-sdk-rs/src/models/generic.rs

use crate::signer::CredentialSigner;
use serde::{Deserialize, Serialize};

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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DfnsApiClientOptions {
    pub base: DfnsBaseApiOptions,
    #[serde(skip)]
    pub signer: Option<Box<dyn CredentialSigner>>,
}

impl std::fmt::Debug for DfnsApiClientOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DfnsApiClientOptions")
            .field("base", &self.base)
            .field("signer", &"<dyn CredentialSigner>")
            .finish()
    }
}

impl Clone for DfnsApiClientOptions {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            signer: None,
        }
    }
}

impl PartialEq for DfnsApiClientOptions {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl DfnsApiClientOptions {
    pub fn new(base: DfnsBaseApiOptions) -> Self {
        Self { base, signer: None }
    }

    pub fn with_signer(mut self, signer: Box<dyn CredentialSigner>) -> Self {
        self.signer = Some(signer);
        self
    }
}
