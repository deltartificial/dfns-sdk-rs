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

## Examples

Check out the list of all examples in [this repository](https://github.com/deltartificial/dfns-sdk-rs-examples).

##### Wallets

- [x] [List Wallets](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/list_wallets.rs) - Demonstrates how to retrieve a list of wallets
- [x] [Create Wallet](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/create_wallet.rs) - Shows how to create a new wallet
- [x] [Get Wallet](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/get_wallet.rs) - Retrieves details of a specific wallet
- [x] [Update Wallet](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/update_wallet.rs) - Updates wallet information
- [x] [Tag Wallet](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/tag_wallet.rs) - Adds tags to a wallet
- [x] [Untag Wallet](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/untag_wallet.rs) - Removes tags from a wallet
- [x] [Get Wallet Assets](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/get_wallet_assets.rs) - Lists assets in a wallet
- [x] [Get Wallet History](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/get_wallet_history.rs) - Shows transaction history
- [x] [Get Wallet NFTs](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/get_wallet_nfts.rs) - Lists NFTs in a wallet
- [x] [Transfer Asset](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/transfer_asset.rs) - Transfers assets between wallets
- [x] [Broadcast Transaction](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/broadcast_transaction.rs) - Broadcasts a transaction
- [x] [Generate Signature](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/generate_signature.rs) - Generates a signature
- [x] [Get Signature](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/get_signature.rs) - Retrieves a specific signature
- [x] [List Signatures](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/list_signatures.rs) - Lists all signatures
- [x] [Get Transaction](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/get_transaction.rs) - Gets transaction details
- [x] [List Transactions](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/list_transactions.rs) - Lists all transactions
- [x] [Get Transfer](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/get_transfer.rs) - Gets transfer details
- [x] [List Transfers](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/list_transfers.rs) - Lists all transfers
- [x] [Export Wallet](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/export_wallet.rs) - Exports a wallet
- [x] [Import Wallet](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/import_wallet.rs) - Imports a wallet
- [x] [Delegate Wallet](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/wallets/delegate_wallet.rs) - Delegates wallet control

##### Webhooks

- [x] [Create Webhook](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/webhooks/create_webhook.rs) - Creates a new webhook
- [x] [Get Webhook](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/webhooks/get_webhook.rs) - Gets webhook details
- [x] [Update Webhook](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/webhooks/update_webhook.rs) - Updates webhook configuration
- [x] [Delete Webhook](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/webhooks/delete_webhook.rs) - Deletes a webhook
- [x] [List Webhooks](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/webhooks/list_webhooks.rs) - Lists all webhooks
- [x] [Ping Webhook](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/webhooks/ping_webhook.rs) - Tests webhook connectivity
- [x] [Get Webhook Event](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/webhooks/get_webhook_event.rs) - Gets webhook event details
- [x] [List Webhook Events](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/webhooks/list_webhook_events.rs) - Lists webhook events

##### Staking

- [x] [Create Stake](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/staking/create_stake.rs) - Creates a new stake
- [x] [Create Stake Action](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/staking/create_stake_action.rs) - Creates a stake action
- [x] [Get Stake Rewards](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/staking/get_stake_rewards.rs) - Gets stake rewards
- [x] [List Stake Actions](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/staking/list_stake_actions.rs) - Lists stake actions
- [x] [List Stakes](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/staking/list_stakes.rs) - Lists all stakes

##### Signers

- [x] [List Signers](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/signers/list_signers.rs) - Lists all signers in clusters

##### Policies

- [x] [Archive Policy](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/policies/archive_policy.rs) - Archives a policy
- [x] [Create Approval Decision](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/policies/create_approval_decision.rs) - Creates an approval decision
- [x] [Create Policy](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/policies/create_policy.rs) - Creates a new policy
- [x] [Get Approval](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/policies/get_approval.rs) - Gets approval details
- [x] [Get Policy](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/policies/get_policy.rs) - Gets policy details
- [x] [List Approvals](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/policies/list_approvals.rs) - Lists all approvals
- [x] [List Policies](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/policies/list_policies.rs) - Lists all policies
- [x] [Update Policy](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/policies/update_policy.rs) - Updates policy configuration

##### Permissions

- [x] [Archive Permission](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/permissions/archive_permission.rs) - Archives a permission
- [x] [Create Assignment](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/permissions/create_assignment.rs) - Creates a permission assignment
- [x] [Create Permission](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/permissions/create_permission.rs) - Creates a new permission
- [x] [Delete Assignment](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/permissions/delete_assignment.rs) - Deletes a permission assignment
- [x] [Get Permission](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/permissions/get_permission.rs) - Gets permission details
- [x] [List Assignments](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/permissions/list_assignments.rs) - Lists all assignments
- [x] [List Permissions](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/permissions/list_permissions.rs) - Lists all permissions
- [x] [Update Permission](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/permissions/update_permission.rs) - Updates permission configuration

##### Networks

- [x] [Get Fees](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/networks/get_fees.rs) - Gets network fees for various blockchains
- [x] [Read Contract](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/networks/read_contract.rs) - Reads data from a smart contract

##### Exchanges

- [x] [Create Deposit](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/create_deposit.rs) - Creates a new deposit
- [x] [Create Exchange](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/create_exchange.rs) - Creates a new exchange
- [x] [Create Withdrawal](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/create_withdrawal.rs) - Creates a new withdrawal
- [x] [Delete Exchange](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/delete_exchange.rs) - Deletes an exchange
- [x] [Get Exchange](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/get_exchange.rs) - Gets exchange details
- [x] [List Account Assets](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/list_account_assets.rs) - Lists assets in an account
- [x] [List Accounts](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/list_accounts.rs) - Lists all accounts
- [x] [List Asset Withdrawal Networks](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/list_asset_withdrawal_networks.rs) - Lists withdrawal networks for an asset
- [x] [List Exchanges](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/exchanges/list_exchanges.rs) - Lists all exchanges

##### Authentication 

- [x] [Activate Application](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/activate_application.rs) - Activates an application
- [x] [Activate Credential](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/activate_credential.rs) - Activates a credential
- [x] [Activate Personal Access Token](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/activate_personal_access_token.rs) - Activates a personal access token
- [x] [Activate Service Account](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/activate_service_account.rs) - Activates a service account
- [x] [Activate User](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/activate_user.rs) - Activates a user
- [x] [Deactivate Application](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/deactivate_application.rs) - Deactivates an application
- [x] [Deactivate Credential](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/deactivate_credential.rs) - Deactivates a credential
- [x] [Deactivate Personal Access Token](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/deactivate_personal_access_token.rs) - Deactivates a personal access token
- [x] [Deactivate Service Account](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/deactivate_service_account.rs) - Deactivates a service account
- [x] [Deactivate User](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/deactivate_user.rs) - Deactivates a user
- [x] [Archive Application](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/archive_application.rs) - Archives an application
- [x] [Archive Personal Access Token](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/archive_personal_access_token.rs) - Archives a personal access token
- [x] [Archive Service Account](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/archive_service_account.rs) - Archives a service account
- [x] [Archive User](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/archive_user.rs) - Archives a user
- [x] [Create Credential Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_credential_challenge.rs) - Creates a credential challenge
- [x] [Create Credential Code](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_credential_code.rs) - Creates a credential code
- [x] [Create Credential With Code](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_credential_with_code.rs) - Creates a credential using a code
- [x] [Create Delegated Recovery Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_delegated_recovery_challenge.rs) - Creates a delegated recovery challenge
- [x] [Create Delegated Registration Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_delegated_registration_challenge.rs) - Creates a delegated registration challenge
- [x] [Create Login Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_login_challenge.rs) - Creates a login challenge
- [x] [Create Personal Access Token](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_personal_access_token.rs) - Creates a personal access token
- [x] [Create Recovery Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_recovery_challenge.rs) - Creates a recovery challenge
- [x] [Create Registration Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_registration_challenge.rs) - Creates a registration challenge
- [x] [Create Service Account](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_service_account.rs) - Creates a service account
- [x] [Create User](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_user.rs) - Creates a new user
- [x] [Create User Action Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_user_action_challenge.rs) - Creates a user action challenge
- [x] [Create User Action Signature](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/create_user_action_signature.rs) - Creates a user action signature
- [x] [Get Application](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/get_application.rs) - Gets application details
- [x] [Get Personal Access Token](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/get_personal_access_token.rs) - Gets personal access token details
- [x] [Get Service Account](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/get_service_account.rs) - Gets service account details
- [x] [Get User](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/get_user.rs) - Gets user details
- [x] [List Applications](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/list_applications.rs) - Lists all applications
- [x] [List Credentials](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/list_credentials.rs) - Lists all credentials
- [x] [List Personal Access Tokens](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/list_personal_access_tokens.rs) - Lists all personal access tokens
- [x] [Login](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/login.rs) - Performs a login
- [x] [Recover](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/recover.rs) - Recovers an account
- [x] [Recreate Delegated Registration Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/recreate_delegated_registration_challenge.rs) - Recreates a delegated registration challenge
- [x] [Register](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/register.rs) - Registers a new user
- [x] [Register End User](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/register_end_user.rs) - Registers an end user
- [x] [Register With Recovery](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/register_with_recovery.rs) - Registers using recovery
- [x] [Update Application](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/update_application.rs) - Updates an application
- [x] [Update Credential](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/update_credential.rs) - Updates a credential
- [x] [Update Personal Access Token](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/update_personal_access_token.rs) - Updates a personal access token
- [x] [Update Service Account](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/update_service_account.rs) - Updates a service account
- [x] [Verify Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/verify_challenge.rs) - Verifies a challenge
- [x] [Verify Recovery Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/verify_recovery_challenge.rs) - Verifies a recovery challenge
- [x] [Verify Registration Challenge](https://github.com/deltartificial/dfns-sdk-rs-examples/blob/master/examples/auth/verify_registration_challenge.rs) - Verifies a registration challenge

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
