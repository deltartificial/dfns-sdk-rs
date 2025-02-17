// @dfns-sdk-rs/src/api/signers/types.rs

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListSignersResponse {
    pub clusters: Vec<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub cluster_id: String,

    pub signers: Vec<Signer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signer {
    pub encryption_key: String,

    pub signer_id: String,
}
