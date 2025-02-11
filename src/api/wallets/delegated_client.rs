// @dfns-sdk-rs/src/api/wallets/delegated_client.rs

use super::types::*;
use crate::{
    client::{
        base_auth_api::{
            BaseAuthApi, CreateUserActionChallengeRequest, SignUserActionChallengeRequest,
        },
        delegated_api_client::DfnsDelegatedApiClientOptions,
    },
    error::DfnsError,
    signer::UserActionChallenge,
    utils::{
        fetch::{simple_fetch, FetchOptions, HttpMethod},
        url::{build_path_and_query, PathAndQueryParams},
    },
};
use serde_json::json;
use std::collections::HashMap;

pub struct DelegatedWalletsClient {
    api_options: DfnsDelegatedApiClientOptions,
}

impl DelegatedWalletsClient {
    pub fn new(api_options: DfnsDelegatedApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn broadcast_transaction_init(
        &self,
        request: BroadcastTransactionRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/transactions",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn broadcast_transaction_complete(
        &self,
        request: BroadcastTransactionRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<BroadcastTransactionResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/transactions",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn create_wallet_init(
        &self,
        request: CreateWalletRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn create_wallet_complete(
        &self,
        request: CreateWalletRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<CreateWalletResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn delegate_wallet_init(
        &self,
        request: DelegateWalletRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/delegate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn delegate_wallet_complete(
        &self,
        request: DelegateWalletRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<DelegateWalletResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/delegate",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn export_wallet_init(
        &self,
        request: ExportWalletRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/export",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn export_wallet_complete(
        &self,
        request: ExportWalletRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ExportWalletResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/export",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_signature(
        &self,
        request: GetSignatureRequest,
    ) -> Result<GetSignatureResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/signatures/:signatureId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map.insert("signatureId".to_string(), request.signature_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_transaction(
        &self,
        request: GetTransactionRequest,
    ) -> Result<GetTransactionResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/transactions/:transactionId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map.insert("transactionId".to_string(), request.transaction_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_transfer(
        &self,
        request: GetTransferRequest,
    ) -> Result<GetTransferResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/transfers/:transferId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map.insert("transferId".to_string(), request.transfer_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_wallet(
        &self,
        request: GetWalletRequest,
    ) -> Result<GetWalletResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_wallet_assets(
        &self,
        request: GetWalletAssetsRequest,
    ) -> Result<GetWalletAssetsResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/assets",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(net_worth) = q.net_worth {
                            map.insert("netWorth".to_string(), net_worth.to_string());
                        }
                        map
                    })
                    .unwrap_or_default(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_wallet_history(
        &self,
        request: GetWalletHistoryRequest,
    ) -> Result<GetWalletHistoryResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/history",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(contract) = q.contract {
                            map.insert("contract".to_string(), contract);
                        }
                        if let Some(direction) = q.direction {
                            map.insert("direction".to_string(), direction.to_string());
                        }
                        if let Some(kind) = q.kind {
                            map.insert("kind".to_string(), kind.to_string());
                        }
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit);
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
                        map
                    })
                    .unwrap_or_default(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn get_wallet_nfts(
        &self,
        request: GetWalletNftsRequest,
    ) -> Result<GetWalletNftsResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/nfts",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn import_wallet_init(
        &self,
        request: ImportWalletRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/import",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn import_wallet_complete(
        &self,
        request: ImportWalletRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<ImportWalletResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/import",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn list_signatures(
        &self,
        request: ListSignaturesRequest,
    ) -> Result<ListSignaturesResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/signatures",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit);
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
                        map
                    })
                    .unwrap_or_default(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn list_transactions(
        &self,
        request: ListTransactionsRequest,
    ) -> Result<ListTransactionsResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/transactions",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit);
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
                        map
                    })
                    .unwrap_or_default(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn list_transfers(
        &self,
        request: ListTransfersRequest,
    ) -> Result<ListTransfersResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/transfers",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: request
                    .query
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit);
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
                        map
                    })
                    .unwrap_or_default(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn list_wallets(
        &self,
        request: Option<ListWalletsRequest>,
    ) -> Result<ListWalletsResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: request
                    .and_then(|r| r.query)
                    .map(|q| {
                        let mut map = HashMap::new();
                        if let Some(limit) = q.limit {
                            map.insert("limit".to_string(), limit);
                        }
                        if let Some(owner_id) = q.owner_id {
                            map.insert("ownerId".to_string(), owner_id);
                        }
                        if let Some(owner_username) = q.owner_username {
                            map.insert("ownerUsername".to_string(), owner_username);
                        }
                        if let Some(token) = q.pagination_token {
                            map.insert("paginationToken".to_string(), token);
                        }
                        map
                    })
                    .unwrap_or_default(),
            },
        );

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn tag_wallet_init(
        &self,
        request: TagWalletRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/tags",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn tag_wallet_complete(
        &self,
        request: TagWalletRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<TagWalletResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/tags",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn transfer_asset_init(
        &self,
        request: TransferAssetRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/transfers",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn transfer_asset_complete(
        &self,
        request: TransferAssetRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<TransferAssetResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/transfers",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn untag_wallet_init(
        &self,
        request: UntagWalletRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/tags",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::DELETE,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn untag_wallet_complete(
        &self,
        request: UntagWalletRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<UntagWalletResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/tags",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::DELETE,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn update_wallet_init(
        &self,
        request: UpdateWalletRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::PUT,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn update_wallet_complete(
        &self,
        request: UpdateWalletRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<UpdateWalletResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::PUT,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn generate_signature_init(
        &self,
        request: GenerateSignatureRequest,
    ) -> Result<UserActionChallenge, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/signatures",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        BaseAuthApi::create_user_action_challenge(
            CreateUserActionChallengeRequest {
                user_action_http_method: HttpMethod::POST,
                user_action_http_path: path,
                user_action_payload: serde_json::to_string(&request.body)?,
                user_action_server_kind: "Api".to_string(),
            },
            self.api_options.base.clone(),
        )
        .await
    }

    pub async fn generate_signature_complete(
        &self,
        request: GenerateSignatureRequest,
        signed_challenge: SignUserActionChallengeRequest,
    ) -> Result<GenerateSignatureResponse, DfnsError> {
        let path = build_path_and_query(
            "/wallets/:walletId/signatures",
            &PathAndQueryParams {
                path: {
                    let mut map = HashMap::new();
                    map.insert("walletId".to_string(), request.wallet_id.clone());
                    map
                },
                query: HashMap::new(),
            },
        );

        let user_action = BaseAuthApi::sign_user_action_challenge(
            signed_challenge,
            self.api_options.base.clone(),
        )
        .await?
        .user_action;

        let mut headers = HashMap::new();
        headers.insert("x-dfns-useraction".to_string(), user_action);

        simple_fetch(
            &path,
            FetchOptions {
                method: HttpMethod::POST,
                headers: Some(headers),
                body: Some(json!(request.body)),
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }
}
