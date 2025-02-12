// @dfns-sdk-rs/src/lib.rs

// @title: Dfns SDK Rust
// @author: deltartificial 
// @version: 0.1.0
// @description: Rust SDK for the Dfns API

pub mod api;
pub mod client;
pub mod error;
pub mod models;
pub mod signer;
pub mod store;
pub mod utils;

pub use client::{
    api_client::DfnsApiClient, authenticator::DfnsAuthenticator, base_auth_api::BaseAuthApi,
    delegated_api_client::DfnsDelegatedApiClient,
};

pub use error::DfnsError;
pub use signer::{CredentialSigner, UserActionChallenge};
pub use store::{Challenge, CredentialStore};

pub use models::generic::{DfnsApiClientOptions, DfnsBaseApiOptions};
