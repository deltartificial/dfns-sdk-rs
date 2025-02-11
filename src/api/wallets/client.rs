// @dfns-sdk-rs/src/api/wallets/client.rs

use super::types::*;
use crate::{
    models::generic::DfnsApiClientOptions,
    utils::{
        fetch::simple_fetch,
        url::{build_path_and_query, PathAndQueryParams},
        user_action_fetch::user_action_fetch,
    },
};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct WalletsClient {
    api_options: DfnsApiClientOptions,
}

impl WalletsClient {
    pub fn new(api_options: DfnsApiClientOptions) -> Self {
        Self { api_options }
    }

    pub async fn broadcast_transaction(
        &self,
        request: BroadcastTransactionRequest,
    ) -> Result<BroadcastTransactionResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/transactions",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn create_wallet(
        &self,
        request: CreateWalletRequest,
    ) -> Result<CreateWalletResponse, crate::error::DfnsError> {
        let path = build_path_and_query(
            "/wallets",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn delegate_wallet(
        &self,
        request: DelegateWalletRequest,
    ) -> Result<DelegateWalletResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/delegate",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn export_wallet(
        &self,
        request: ExportWalletRequest,
    ) -> Result<ExportWalletResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/export",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn generate_signature(
        &self,
        request: GenerateSignatureRequest,
    ) -> Result<GenerateSignatureResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/signatures",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn get_signature(
        &self,
        request: GetSignatureRequest,
    ) -> Result<GetSignatureResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());
        path_params.insert("signatureId".to_string(), request.signature_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/signatures/:signatureId",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<GetTransactionResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());
        path_params.insert("transactionId".to_string(), request.transaction_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/transactions/:transactionId",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<GetTransferResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());
        path_params.insert("transferId".to_string(), request.transfer_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/transfers/:transferId",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<GetWalletResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<GetWalletAssetsResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let mut query_params = HashMap::new();
        if let Some(query) = request.query {
            if let Some(net_worth) = query.net_worth {
                query_params.insert("netWorth".to_string(), net_worth.to_string());
            }
        }

        let path = build_path_and_query(
            "/wallets/:walletId/assets",
            &PathAndQueryParams {
                path: path_params,
                query: query_params,
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<GetWalletHistoryResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let mut query_params = HashMap::new();
        if let Some(query) = request.query {
            if let Some(contract) = query.contract {
                query_params.insert("contract".to_string(), contract);
            }
            if let Some(direction) = query.direction {
                query_params.insert("direction".to_string(), direction.to_string());
            }
            if let Some(kind) = query.kind {
                query_params.insert("kind".to_string(), kind.to_string());
            }
            if let Some(limit) = query.limit {
                query_params.insert("limit".to_string(), limit);
            }
            if let Some(token) = query.pagination_token {
                query_params.insert("paginationToken".to_string(), token);
            }
        }

        let path = build_path_and_query(
            "/wallets/:walletId/history",
            &PathAndQueryParams {
                path: path_params,
                query: query_params,
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<GetWalletNftsResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/nfts",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn import_wallet(
        &self,
        request: ImportWalletRequest,
    ) -> Result<ImportWalletResponse, crate::error::DfnsError> {
        let path = build_path_and_query(
            "/wallets/import",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn list_signatures(
        &self,
        request: ListSignaturesRequest,
    ) -> Result<ListSignaturesResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let mut query_params = HashMap::new();
        if let Some(query) = request.query {
            if let Some(limit) = query.limit {
                query_params.insert("limit".to_string(), limit);
            }
            if let Some(token) = query.pagination_token {
                query_params.insert("paginationToken".to_string(), token);
            }
        }

        let path = build_path_and_query(
            "/wallets/:walletId/signatures",
            &PathAndQueryParams {
                path: path_params,
                query: query_params,
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<ListTransactionsResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let mut query_params = HashMap::new();
        if let Some(query) = request.query {
            if let Some(limit) = query.limit {
                query_params.insert("limit".to_string(), limit);
            }
            if let Some(token) = query.pagination_token {
                query_params.insert("paginationToken".to_string(), token);
            }
        }

        let path = build_path_and_query(
            "/wallets/:walletId/transactions",
            &PathAndQueryParams {
                path: path_params,
                query: query_params,
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<ListTransfersResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let mut query_params = HashMap::new();
        if let Some(query) = request.query {
            if let Some(limit) = query.limit {
                query_params.insert("limit".to_string(), limit);
            }
            if let Some(token) = query.pagination_token {
                query_params.insert("paginationToken".to_string(), token);
            }
        }

        let path = build_path_and_query(
            "/wallets/:walletId/transfers",
            &PathAndQueryParams {
                path: path_params,
                query: query_params,
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
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
    ) -> Result<ListWalletsResponse, crate::error::DfnsError> {
        let mut query_params = HashMap::new();
        if let Some(req) = request {
            if let Some(query) = req.query {
                if let Some(limit) = query.limit {
                    query_params.insert("limit".to_string(), limit);
                }
                if let Some(owner_id) = query.owner_id {
                    query_params.insert("ownerId".to_string(), owner_id);
                }
                if let Some(owner_username) = query.owner_username {
                    query_params.insert("ownerUsername".to_string(), owner_username);
                }
                if let Some(token) = query.pagination_token {
                    query_params.insert("paginationToken".to_string(), token);
                }
            }
        }

        let path = build_path_and_query(
            "/wallets",
            &PathAndQueryParams {
                path: HashMap::new(),
                query: query_params,
            },
        );

        simple_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::GET,
                headers: None,
                body: None,
                api_options: self.api_options.base.clone(),
            },
        )
        .await
    }

    pub async fn tag_wallet(
        &self,
        request: TagWalletRequest,
    ) -> Result<TagWalletResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/tags",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::PUT,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn transfer_asset(
        &self,
        request: TransferAssetRequest,
    ) -> Result<TransferAssetResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/transfers",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::POST,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn untag_wallet(
        &self,
        request: UntagWalletRequest,
    ) -> Result<UntagWalletResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId/tags",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::DELETE,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }

    pub async fn update_wallet(
        &self,
        request: UpdateWalletRequest,
    ) -> Result<UpdateWalletResponse, crate::error::DfnsError> {
        let mut path_params = HashMap::new();
        path_params.insert("walletId".to_string(), request.wallet_id.clone());

        let path = build_path_and_query(
            "/wallets/:walletId",
            &PathAndQueryParams {
                path: path_params,
                query: HashMap::new(),
            },
        );

        user_action_fetch(
            &path,
            crate::utils::fetch::FetchOptions {
                method: crate::utils::fetch::HttpMethod::PUT,
                headers: None,
                body: Some(json!(request.body)),
                api_options: self.api_options.clone(),
            },
        )
        .await
    }
}
