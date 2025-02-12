# Dfns SDK Rust

![Build status](https://img.shields.io/badge/build-passing-brightgreen?style=flat) ![License](https://img.shields.io/badge/license-MIT-blue) ![Implementation](https://img.shields.io/badge/implemented-100%25-green)

**Modular, extensible, and easy-to-use Rust SDK for the Dfns API.**

![](./assets/sdk-rs.png)

> **Warning**: This SDK is currently under development and not ready for production use. The API is subject to change. This SDK is not fully implemented and missing tests.

- [Dfns Website](https://www.dfns.co)
- [Dfns API Docs](https://docs.dfns.co)

## Installation

```bash
cargo add dfns-sdk-rs
```

## Usage

Here's a simple example of how to use the SDK:

```rust
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, CredentialSigner,
    models::wallets::CreateWalletRequest,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize your signer (implementation depends on your use case)
    let signer = Arc::new(YourCredentialSigner::new());

    // Create API client
    let client = DfnsApiClient::new(
        DfnsBaseApiOptions {
            base_url: "https://api.dfns.io".to_string(),
            app_id: "ap-2ng9jv-80cfc-983pop0iauf2sv8r".to_string(),
            auth_token: "your-auth-token".to_string(),
        },
        Some(signer),
    );

    // Create a wallet
    let wallet = client
        .wallets()
        .create_wallet(CreateWalletRequest {
            network: "EthereumSepolia".to_string(),
        })
        .await?;

    // Get wallet assets
    let assets = client
        .wallets()
        .get_wallet_assets(wallet.id)
        .await?;

    println!("Wallet assets: {:?}", assets);
    Ok(())
}
```

The example above demonstrates:

- Creating an API client with authentication
- Creating a new wallet
- Retrieving wallet assets

## Documentation

### Generating Documentation

To generate the documentation locally, you'll need the nightly Rust toolchain. Here's how to set it up:

```bash
# Install and switch to nightly Rust
rustup install nightly
rustup default nightly

# Generate the documentation
RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo doc --no-deps --document-private-items --target-dir ./docs
```

### Viewing Documentation

To view the documentation in your browser:

```bash
# Start a local server (Python required)
python3 -m http.server 8000 --directory ./docs/doc
```

Then open [http://localhost:8000/dfns_sdk_rs/](http://localhost:8000/dfns_sdk_rs/) in your browser.

For more examples and detailed API documentation, check the [documentation section](#documentation).
