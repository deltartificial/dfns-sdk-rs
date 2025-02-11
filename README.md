# Dfns SDK Rust

![Build status](https://img.shields.io/badge/build-passing-brightgreen?style=flat) ![License](https://img.shields.io/badge/license-MIT-blue) ![Implementation](https://img.shields.io/badge/implemented-82%25-blue)



**Modular, extensible, and easy-to-use Rust SDK for the Dfns API.**

![](./assets/sdk-rs.png)

> **Warning**: This SDK is currently under development and not ready for production use. The API is subject to change. Not fully implemented.

## Structure

55/67 files are implemented = 82%

```md
├── api
│   ├── auth
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── exchanges
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── mod.rs ✅
│   ├── networks
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── permissions
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── policies
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── signers
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── staking
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   ├── wallets
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
│   └── webhooks
│   │   ├── client.rs ✅
│   │   ├── delegated_client.rs
│   │   ├── mod.rs ✅
│   │   └── types.rs ✅
├── client
│   ├── api_client.rs ✅
│   ├── authenticator.rs ✅
│   ├── base_auth_api.rs ✅
│   ├── delegated_api_client.rs
│   └── mod.rs ✅
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
│   ├── fetch.rs ✅
│   ├── mod.rs ✅
│   ├── nonce.rs ✅
│   ├── string.rs ✅
│   ├── url.rs ✅
│   └── user_action_fetch.rs ✅

14 directories, 67 files
```
