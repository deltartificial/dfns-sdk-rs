# Dfns SDK Rust

> **Warning**: This SDK is currently under development and not ready for production use. The API is subject to change.

## Structure

```md
├── api
│   ├── auth
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── exchanges
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── mod.rs ✅
│   ├── networks
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── permissions
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── policies
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── signers
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── staking
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── wallets
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   └── webhooks
│   │   ├── client.rs
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
├── client
│   ├── api_client.rs
│   ├── authenticator.rs
│   ├── base_auth_api.rs
│   ├── delegated_api_client.rs
│   └── mod.rs
├── error.rs ✅
├── lib.rs
├── main.rs
├── models
│   ├── auth.rs ✅
│   ├── exchanges.rs ✅
│   ├── generic.rs ✅
│   ├── mod.rs ✅
│   ├── networks.rs ✅
│   ├── permissions.rs ✅
│   ├── policies.rs ✅
│   ├── signers.rs ✅
│   ├── staking.rs ✅
│   ├── wallets.rs ✅
│   └── webhooks.rs ✅
├── signer.rs ✅
├── store.rs ✅
└── utils
│   ├── base64.rs ✅
│   ├── bigint.rs ✅
│   ├── crypto.rs ✅
│   ├── fetch.rs
│   ├── mod.rs ✅
│   ├── nonce.rs ✅
│   ├── string.rs ✅
│   ├── url.rs ✅
│   └── user_action_fetch.rs

14 directories, 66 files
```
