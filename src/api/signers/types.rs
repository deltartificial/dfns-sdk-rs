// @dfns-sdk-rs/src/api/signers/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signer {
    pub signer_id: String,
    pub encryption_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignerCluster {
    pub cluster_id: String,
    pub signers: Vec<Signer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSignersResponse {
    pub clusters: Vec<SignerCluster>,
}