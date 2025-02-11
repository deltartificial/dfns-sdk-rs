// @dfns-sdk-rs/src/api/policies/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePolicyParams {
    pub policy_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePolicyResponse {
    pub action: ArchivePolicyResponseAction,

    pub activity_kind: ActivityKindEnum,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<ArchivePolicyResponseFilters>,

    pub id: String,

    pub name: String,

    pub rule: ArchivePolicyResponseRule,

    pub status: ArchivePolicyResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePolicyResponseAction {
    pub approval_groups: Option<Vec<PurpleApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleApprovalGroup {
    pub approvers: PurpleApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleApprovers {
    pub user_id: Option<PurpleUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionKind {
    Block,

    #[serde(rename = "NoAction")]
    NoAction,

    #[serde(rename = "RequestApproval")]
    RequestApproval,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivityKindEnum {
    #[serde(rename = "Permissions:Assign")]
    PermissionsAssign,

    #[serde(rename = "Permissions:Modify")]
    PermissionsModify,

    #[serde(rename = "Policies:Modify")]
    PoliciesModify,

    #[serde(rename = "Wallets:IncomingTransaction")]
    WalletsIncomingTransaction,

    #[serde(rename = "Wallets:Sign")]
    WalletsSign,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePolicyResponseFilters {
    pub permission_id: Option<PurplePermissionId>,

    pub policy_id: Option<PurplePolicyId>,

    pub wallet_id: Option<PurpleWalletId>,

    pub wallet_tags: Option<PurpleWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurplePermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurplePolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchivePolicyResponseRule {
    pub configuration: Option<PurpleConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleConfiguration {
    pub addresses: Option<BraggadociousAddresses>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<PurpleAlerts>,

    pub exposures: Option<PurpleExposures>,

    pub fallback_behaviours: Option<PurpleFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BraggadociousAddresses {
    PurpleAddresses(PurpleAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertLevel {
    #[serde(rename = "HIGH")]
    High,

    #[serde(rename = "LOW")]
    Low,

    #[serde(rename = "MEDIUM")]
    Medium,

    #[serde(rename = "SEVERE")]
    Severe,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleExposures {
    pub direct: PurpleDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuleKind {
    #[serde(rename = "AlwaysTrigger")]
    AlwaysTrigger,

    #[serde(rename = "ChainalysisTransactionPrescreening")]
    ChainalysisTransactionPrescreening,

    #[serde(rename = "ChainalysisTransactionScreening")]
    ChainalysisTransactionScreening,

    #[serde(rename = "TransactionAmountLimit")]
    TransactionAmountLimit,

    #[serde(rename = "TransactionAmountVelocity")]
    TransactionAmountVelocity,

    #[serde(rename = "TransactionCountVelocity")]
    TransactionCountVelocity,

    #[serde(rename = "TransactionRecipientWhitelist")]
    TransactionRecipientWhitelist,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArchivePolicyResponseStatus {
    Active,

    Archived,
}

impl std::fmt::Display for ArchivePolicyResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePolicyRequest {
    pub policy_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApprovalDecisionBody {
    pub reason: Option<String>,

    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Approved,

    Denied,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalDecisionParams {
    pub approval_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalDecisionResponse {
    pub activity: CreateApprovalDecisionResponseActivity,

    pub date_created: Option<String>,

    pub date_resolved: Option<String>,

    pub date_updated: String,

    pub decisions: Vec<CreateApprovalDecisionResponseDecision>,

    pub expiration_date: Option<String>,

    pub id: String,

    pub initiator_id: String,

    pub policy_evaluations: Vec<CreateApprovalDecisionResponsePolicyEvaluation>,

    pub status: CreateApprovalDecisionResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalDecisionResponseActivity {
    pub kind: ActivityKindEnum,

    pub signature_request: Option<PurpleSignatureRequest>,

    pub transaction_request: Option<PurpleTransactionRequest>,

    pub transfer_request: Option<PurpleTransferRequest>,

    pub blockchain_event: Option<PurpleBlockchainEvent>,

    pub change_request: Option<PurpleChangeRequest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleBlockchainEvent {
    pub balance_id: Option<String>,

    pub block_number: f64,

    pub decimals: Option<f64>,

    pub direction: Direction,

    pub fee: Option<String>,

    pub from: Option<String>,

    pub index: Option<String>,

    pub kind: BlockchainEventKind,

    pub liquidity_pool: Option<String>,

    pub memo: Option<String>,

    pub metadata: PurpleMetadata,

    pub network: BlockchainEventNetwork,

    pub symbol: Option<String>,

    pub timestamp: String,

    pub to: Option<String>,

    pub tx_hash: String,

    pub value: Option<String>,

    pub verified: Option<bool>,

    pub wallet_id: String,

    pub metadata_address: Option<String>,

    pub asset_id: Option<String>,

    pub clawback: Option<bool>,

    pub opt_in: Option<bool>,

    pub opt_out: Option<bool>,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,

    pub froms: Option<Vec<String>>,

    pub tos: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    In,

    Out,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockchainEventKind {
    #[serde(rename = "Aip21Transfer")]
    Aip21Transfer,

    #[serde(rename = "AsaTransfer")]
    AsaTransfer,

    #[serde(rename = "Erc20Transfer")]
    Erc20Transfer,

    #[serde(rename = "Erc721Transfer")]
    Erc721Transfer,

    #[serde(rename = "NativeTransfer")]
    NativeTransfer,

    #[serde(rename = "Sep41Transfer")]
    Sep41Transfer,

    #[serde(rename = "Spl2022Transfer")]
    Spl2022Transfer,

    #[serde(rename = "SplTransfer")]
    SplTransfer,

    #[serde(rename = "Tep74Transfer")]
    Tep74Transfer,

    #[serde(rename = "Trc10Transfer")]
    Trc10Transfer,

    #[serde(rename = "Trc20Transfer")]
    Trc20Transfer,

    #[serde(rename = "Trc721Transfer")]
    Trc721Transfer,

    #[serde(rename = "UtxoTransfer")]
    UtxoTransfer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleMetadata {
    pub asset: PurpleAsset,

    pub fee: Option<PurpleFee>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleFee {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockchainEventNetwork {
    Algorand,

    #[serde(rename = "AlgorandTestnet")]
    AlgorandTestnet,

    Aptos,

    #[serde(rename = "AptosTestnet")]
    AptosTestnet,

    #[serde(rename = "ArbitrumGoerli")]
    ArbitrumGoerli,

    #[serde(rename = "ArbitrumOne")]
    ArbitrumOne,

    #[serde(rename = "ArbitrumSepolia")]
    ArbitrumSepolia,

    #[serde(rename = "AvalancheC")]
    AvalancheC,

    #[serde(rename = "AvalancheCFuji")]
    AvalancheCFuji,

    Base,

    #[serde(rename = "BaseGoerli")]
    BaseGoerli,

    #[serde(rename = "BaseSepolia")]
    BaseSepolia,

    Berachain,

    #[serde(rename = "BerachainBArtio")]
    BerachainBArtio,

    Bitcoin,

    #[serde(rename = "BitcoinSignet")]
    BitcoinSignet,

    #[serde(rename = "BitcoinTestnet3")]
    BitcoinTestnet3,

    Bsc,

    #[serde(rename = "BscTestnet")]
    BscTestnet,

    Cardano,

    #[serde(rename = "CardanoPreprod")]
    CardanoPreprod,

    Celo,

    #[serde(rename = "CeloAlfajores")]
    CeloAlfajores,

    Dogecoin,

    #[serde(rename = "DogecoinTestnet")]
    DogecoinTestnet,

    Ethereum,

    #[serde(rename = "EthereumGoerli")]
    EthereumGoerli,

    #[serde(rename = "EthereumHolesky")]
    EthereumHolesky,

    #[serde(rename = "EthereumSepolia")]
    EthereumSepolia,

    #[serde(rename = "FantomOpera")]
    FantomOpera,

    #[serde(rename = "FantomTestnet")]
    FantomTestnet,

    #[serde(rename = "InternetComputer")]
    InternetComputer,

    Ion,

    #[serde(rename = "IonTestnet")]
    IonTestnet,

    Iota,

    #[serde(rename = "IotaTestnet")]
    IotaTestnet,

    Kaspa,

    #[serde(rename = "KaspaTestnet11")]
    KaspaTestnet11,

    Kusama,

    Litecoin,

    #[serde(rename = "LitecoinTestnet")]
    LitecoinTestnet,

    Optimism,

    #[serde(rename = "OptimismGoerli")]
    OptimismGoerli,

    #[serde(rename = "OptimismSepolia")]
    OptimismSepolia,

    Origyn,

    Polkadot,

    Polygon,

    #[serde(rename = "PolygonAmoy")]
    PolygonAmoy,

    #[serde(rename = "PolygonMumbai")]
    PolygonMumbai,

    Race,

    #[serde(rename = "RaceSepolia")]
    RaceSepolia,

    #[serde(rename = "SeiAtlantic2")]
    SeiAtlantic2,

    #[serde(rename = "SeiPacific1")]
    SeiPacific1,

    Solana,

    #[serde(rename = "SolanaDevnet")]
    SolanaDevnet,

    Stellar,

    #[serde(rename = "StellarTestnet")]
    StellarTestnet,

    Tezos,

    #[serde(rename = "TezosGhostnet")]
    TezosGhostnet,

    Ton,

    #[serde(rename = "TonTestnet")]
    TonTestnet,

    Tron,

    #[serde(rename = "TronNile")]
    TronNile,

    Westend,

    #[serde(rename = "XrpLedger")]
    XrpLedger,

    #[serde(rename = "XrpLedgerTestnet")]
    XrpLedgerTestnet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleChangeRequest {
    pub approval_id: Option<String>,

    pub body: PurpleBody,

    pub date_created: String,

    pub date_resolved: Option<String>,

    pub entity_id: String,

    pub id: String,

    pub kind: ChangeRequestKind,

    pub operation_kind: ChangeRequestOperationKind,

    pub requester: PurpleRequester,

    pub status: ChangeRequestStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleBody {
    pub action: Option<PurpleAction>,

    pub activity_kind: Option<ActivityKindEnum>,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<PurpleFilters>,

    pub id: String,

    pub name: Option<String>,

    pub rule: Option<PurpleRule>,

    pub status: Option<ArchivePolicyResponseStatus>,

    pub is_archived: Option<bool>,

    pub is_immutable: Option<bool>,

    pub operations: Option<Vec<String>>,

    pub identity_id: Option<String>,

    pub permission_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleAction {
    pub approval_groups: Option<Vec<FluffyApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyApprovalGroup {
    pub approvers: FluffyApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyApprovers {
    pub user_id: Option<FluffyUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleFilters {
    pub permission_id: Option<FluffyPermissionId>,

    pub policy_id: Option<FluffyPolicyId>,

    pub wallet_id: Option<FluffyWalletId>,

    pub wallet_tags: Option<FluffyWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleRule {
    pub configuration: Option<FluffyConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyConfiguration {
    pub addresses: Option<Addresses1>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<FluffyAlerts>,

    pub exposures: Option<FluffyExposures>,

    pub fallback_behaviours: Option<FluffyFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses1 {
    FluffyAddresses(FluffyAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyExposures {
    pub direct: FluffyDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeRequestKind {
    Assignment,

    Permission,

    Policy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeRequestOperationKind {
    Create,

    Delete,

    Update,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeRequestStatus {
    Applied,

    Failed,

    Pending,

    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleSignatureRequest {
    pub approval_id: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub date_signed: Option<String>,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: SignatureRequestNetwork,

    pub reason: Option<String>,

    pub request_body: PurpleRequestBody,

    pub requester: FluffyRequester,

    pub signature: Option<PurpleSignature>,

    pub signatures: Option<Vec<FluffySignature>>,

    pub signed_data: Option<String>,

    pub status: SignatureRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignatureRequestNetwork {
    Algorand,

    #[serde(rename = "AlgorandTestnet")]
    AlgorandTestnet,

    Aptos,

    #[serde(rename = "AptosTestnet")]
    AptosTestnet,

    #[serde(rename = "ArbitrumGoerli")]
    ArbitrumGoerli,

    #[serde(rename = "ArbitrumOne")]
    ArbitrumOne,

    #[serde(rename = "ArbitrumSepolia")]
    ArbitrumSepolia,

    #[serde(rename = "AvalancheC")]
    AvalancheC,

    #[serde(rename = "AvalancheCFuji")]
    AvalancheCFuji,

    Base,

    #[serde(rename = "BaseGoerli")]
    BaseGoerli,

    #[serde(rename = "BaseSepolia")]
    BaseSepolia,

    Berachain,

    #[serde(rename = "BerachainBArtio")]
    BerachainBArtio,

    Bitcoin,

    #[serde(rename = "BitcoinSignet")]
    BitcoinSignet,

    #[serde(rename = "BitcoinTestnet3")]
    BitcoinTestnet3,

    Bsc,

    #[serde(rename = "BscTestnet")]
    BscTestnet,

    Cardano,

    #[serde(rename = "CardanoPreprod")]
    CardanoPreprod,

    Celo,

    #[serde(rename = "CeloAlfajores")]
    CeloAlfajores,

    Dogecoin,

    #[serde(rename = "DogecoinTestnet")]
    DogecoinTestnet,

    Ethereum,

    #[serde(rename = "EthereumGoerli")]
    EthereumGoerli,

    #[serde(rename = "EthereumHolesky")]
    EthereumHolesky,

    #[serde(rename = "EthereumSepolia")]
    EthereumSepolia,

    #[serde(rename = "FantomOpera")]
    FantomOpera,

    #[serde(rename = "FantomTestnet")]
    FantomTestnet,

    #[serde(rename = "InternetComputer")]
    InternetComputer,

    Ion,

    #[serde(rename = "IonTestnet")]
    IonTestnet,

    Iota,

    #[serde(rename = "IotaTestnet")]
    IotaTestnet,

    Kaspa,

    #[serde(rename = "KaspaTestnet11")]
    KaspaTestnet11,

    #[serde(rename = "KeyECDSA")]
    KeyEcdsa,

    #[serde(rename = "KeyECDSAStark")]
    KeyEcdsaStark,

    #[serde(rename = "KeyEdDSA")]
    KeyEdDsa,

    Kusama,

    Litecoin,

    #[serde(rename = "LitecoinTestnet")]
    LitecoinTestnet,

    Optimism,

    #[serde(rename = "OptimismGoerli")]
    OptimismGoerli,

    #[serde(rename = "OptimismSepolia")]
    OptimismSepolia,

    Origyn,

    Polkadot,

    Polygon,

    #[serde(rename = "PolygonAmoy")]
    PolygonAmoy,

    #[serde(rename = "PolygonMumbai")]
    PolygonMumbai,

    Race,

    #[serde(rename = "RaceSepolia")]
    RaceSepolia,

    #[serde(rename = "SeiAtlantic2")]
    SeiAtlantic2,

    #[serde(rename = "SeiPacific1")]
    SeiPacific1,

    Solana,

    #[serde(rename = "SolanaDevnet")]
    SolanaDevnet,

    Stellar,

    #[serde(rename = "StellarTestnet")]
    StellarTestnet,

    Tezos,

    #[serde(rename = "TezosGhostnet")]
    TezosGhostnet,

    Ton,

    #[serde(rename = "TonTestnet")]
    TonTestnet,

    Tron,

    #[serde(rename = "TronNile")]
    TronNile,

    Westend,

    #[serde(rename = "XrpLedger")]
    XrpLedger,

    #[serde(rename = "XrpLedgerTestnet")]
    XrpLedgerTestnet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRequestBody {
    pub external_id: Option<String>,

    pub kind: PurpleKind,

    pub sign_doc: Option<String>,

    pub hash: Option<String>,

    pub taproot_merkle_root: Option<String>,

    pub message: Option<Message>,

    pub transaction: Option<String>,

    pub domain: Option<PurpleDomain>,

    pub types: Option<HashMap<String, Vec<PurpleType>>>,

    pub psbt: Option<String>,

    pub format: Option<Format>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleDomain {
    pub chain_id: Option<ChainId>,

    pub name: Option<String>,

    pub salt: Option<String>,

    pub verifying_contract: Option<String>,

    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChainId {
    Double(f64),

    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Format {
    Full,

    Simple,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PurpleKind {
    Bip322,

    Eip712,

    Hash,

    Message,

    Psbt,

    #[serde(rename = "SignDocDirect")]
    SignDocDirect,

    Transaction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),

    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleType {
    pub name: String,

    #[serde(rename = "type")]
    pub type_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleSignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffySignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignatureRequestStatus {
    Confirmed,

    Executing,

    Failed,

    Pending,

    Rejected,

    Signed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleTransactionRequest {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: BlockchainEventNetwork,

    pub reason: Option<String>,

    pub request_body: FluffyRequestBody,

    pub requester: TentacledRequester,

    pub status: TransactionRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRequestBody {
    pub external_id: Option<String>,

    pub kind: FluffyKind,

    pub transaction: Option<String>,

    pub data: Option<String>,

    pub gas_limit: Option<String>,

    pub nonce: Option<ChainId>,

    pub to: Option<String>,

    pub value: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,

    pub gas_price: Option<String>,

    pub psbt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FluffyKind {
    Eip1559,

    Evm,

    #[serde(rename = "EvmLegacy")]
    EvmLegacy,

    Psbt,

    Transaction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionRequestStatus {
    Broadcasted,

    Confirmed,

    Executing,

    Failed,

    Pending,

    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleTransferRequest {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub metadata: FluffyMetadata,

    pub network: BlockchainEventNetwork,

    pub reason: Option<String>,

    pub request_body: TentacledRequestBody,

    pub requester: StickyRequester,

    pub status: TransactionRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyMetadata {
    pub asset: FluffyAsset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledRequestBody {
    pub amount: Option<String>,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: TentacledKind,

    pub memo: Option<String>,

    pub priority: Option<Priority>,

    pub to: String,

    pub asset_id: Option<String>,

    pub metadata: Option<String>,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TentacledKind {
    Aip21,

    Asa,

    Erc20,

    Erc721,

    Native,

    Sep41,

    Spl,

    Spl2022,

    Tep74,

    Trc10,

    Trc20,

    Trc721,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Fast,

    Slow,

    Standard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalDecisionResponseDecision {
    pub date: String,

    pub reason: Option<String>,

    pub user_id: String,

    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalDecisionResponsePolicyEvaluation {
    pub policy_id: String,

    pub reason: String,

    pub triggered: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateApprovalDecisionResponseStatus {
    Approved,

    Denied,

    Expired,

    Pending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalDecisionRequest {
    pub approval_id: String,

    pub body: CreateApprovalDecisionRequestBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApprovalDecisionRequestBody {
    pub reason: Option<String>,

    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyResponse {
    pub action: CreatePolicyResponseAction,

    pub activity_kind: ActivityKindEnum,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<CreatePolicyResponseFilters>,

    pub id: String,

    pub name: String,

    pub rule: CreatePolicyResponseRule,

    pub status: ArchivePolicyResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyResponseAction {
    pub approval_groups: Option<Vec<StickyApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyApprovalGroup {
    pub approvers: StickyApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyApprovers {
    pub user_id: Option<StickyUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyResponseFilters {
    pub permission_id: Option<StickyPermissionId>,

    pub policy_id: Option<StickyPolicyId>,

    pub wallet_id: Option<StickyWalletId>,

    pub wallet_tags: Option<StickyWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePolicyResponseRule {
    pub configuration: Option<StickyConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyConfiguration {
    pub addresses: Option<Addresses3>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<StickyAlerts>,

    pub exposures: Option<StickyExposures>,

    pub fallback_behaviours: Option<StickyFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses3 {
    StickyAddresses(StickyAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyExposures {
    pub direct: StickyDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePolicyRequest {
    pub body: CreatePolicyBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyBody {
    pub action: CreatePolicyBodyAction,

    pub activity_kind: ActivityKindEnum,

    pub filters: Option<CreatePolicyBodyFilters>,

    pub name: String,

    pub rule: CreatePolicyBodyRule,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyBodyAction {
    pub approval_groups: Option<Vec<TentacledApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledApprovalGroup {
    pub approvers: TentacledApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledApprovers {
    pub user_id: Option<TentacledUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyBodyFilters {
    pub permission_id: Option<TentacledPermissionId>,

    pub policy_id: Option<TentacledPolicyId>,

    pub wallet_id: Option<TentacledWalletId>,

    pub wallet_tags: Option<TentacledWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePolicyBodyRule {
    pub configuration: Option<TentacledConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledConfiguration {
    pub addresses: Option<Addresses2>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<TentacledAlerts>,

    pub exposures: Option<TentacledExposures>,

    pub fallback_behaviours: Option<TentacledFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses2 {
    StringArray(Vec<String>),

    TentacledAddresses(TentacledAddresses),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledExposures {
    pub direct: TentacledDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApprovalParams {
    pub approval_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApprovalResponse {
    pub activity: GetApprovalResponseActivity,

    pub date_created: Option<String>,

    pub date_resolved: Option<String>,

    pub date_updated: String,

    pub decisions: Vec<GetApprovalResponseDecision>,

    pub expiration_date: Option<String>,

    pub id: String,

    pub initiator_id: String,

    pub policy_evaluations: Vec<GetApprovalResponsePolicyEvaluation>,

    pub status: CreateApprovalDecisionResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApprovalResponseActivity {
    pub kind: ActivityKindEnum,

    pub signature_request: Option<FluffySignatureRequest>,

    pub transaction_request: Option<FluffyTransactionRequest>,

    pub transfer_request: Option<FluffyTransferRequest>,

    pub blockchain_event: Option<FluffyBlockchainEvent>,

    pub change_request: Option<FluffyChangeRequest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyBlockchainEvent {
    pub balance_id: Option<String>,

    pub block_number: f64,

    pub decimals: Option<f64>,

    pub direction: Direction,

    pub fee: Option<String>,

    pub from: Option<String>,

    pub index: Option<String>,

    pub kind: BlockchainEventKind,

    pub liquidity_pool: Option<String>,

    pub memo: Option<String>,

    pub metadata: TentacledMetadata,

    pub network: BlockchainEventNetwork,

    pub symbol: Option<String>,

    pub timestamp: String,

    pub to: Option<String>,

    pub tx_hash: String,

    pub value: Option<String>,

    pub verified: Option<bool>,

    pub wallet_id: String,

    pub metadata_address: Option<String>,

    pub asset_id: Option<String>,

    pub clawback: Option<bool>,

    pub opt_in: Option<bool>,

    pub opt_out: Option<bool>,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,

    pub froms: Option<Vec<String>>,

    pub tos: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledMetadata {
    pub asset: TentacledAsset,

    pub fee: Option<FluffyFee>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyFee {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyChangeRequest {
    pub approval_id: Option<String>,

    pub body: FluffyBody,

    pub date_created: String,

    pub date_resolved: Option<String>,

    pub entity_id: String,

    pub id: String,

    pub kind: ChangeRequestKind,

    pub operation_kind: ChangeRequestOperationKind,

    pub requester: IndigoRequester,

    pub status: ChangeRequestStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyBody {
    pub action: Option<FluffyAction>,

    pub activity_kind: Option<ActivityKindEnum>,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<FluffyFilters>,

    pub id: String,

    pub name: Option<String>,

    pub rule: Option<FluffyRule>,

    pub status: Option<ArchivePolicyResponseStatus>,

    pub is_archived: Option<bool>,

    pub is_immutable: Option<bool>,

    pub operations: Option<Vec<String>>,

    pub identity_id: Option<String>,

    pub permission_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyAction {
    pub approval_groups: Option<Vec<IndigoApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoApprovalGroup {
    pub approvers: IndigoApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoApprovers {
    pub user_id: Option<IndigoUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyFilters {
    pub permission_id: Option<IndigoPermissionId>,

    pub policy_id: Option<IndigoPolicyId>,

    pub wallet_id: Option<IndigoWalletId>,

    pub wallet_tags: Option<IndigoWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyRule {
    pub configuration: Option<IndigoConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoConfiguration {
    pub addresses: Option<Addresses4>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<IndigoAlerts>,

    pub exposures: Option<IndigoExposures>,

    pub fallback_behaviours: Option<IndigoFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses4 {
    IndigoAddresses(IndigoAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoExposures {
    pub direct: IndigoDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffySignatureRequest {
    pub approval_id: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub date_signed: Option<String>,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: SignatureRequestNetwork,

    pub reason: Option<String>,

    pub request_body: StickyRequestBody,

    pub requester: IndecentRequester,

    pub signature: Option<TentacledSignature>,

    pub signatures: Option<Vec<StickySignature>>,

    pub signed_data: Option<String>,

    pub status: SignatureRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyRequestBody {
    pub external_id: Option<String>,

    pub kind: PurpleKind,

    pub sign_doc: Option<String>,

    pub hash: Option<String>,

    pub taproot_merkle_root: Option<String>,

    pub message: Option<Message>,

    pub transaction: Option<String>,

    pub domain: Option<FluffyDomain>,

    pub types: Option<HashMap<String, Vec<FluffyType>>>,

    pub psbt: Option<String>,

    pub format: Option<Format>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyDomain {
    pub chain_id: Option<ChainId>,

    pub name: Option<String>,

    pub salt: Option<String>,

    pub verifying_contract: Option<String>,

    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyType {
    pub name: String,

    #[serde(rename = "type")]
    pub type_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledSignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickySignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyTransactionRequest {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: BlockchainEventNetwork,

    pub reason: Option<String>,

    pub request_body: IndigoRequestBody,

    pub requester: HilariousRequester,

    pub status: TransactionRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoRequestBody {
    pub external_id: Option<String>,

    pub kind: FluffyKind,

    pub transaction: Option<String>,

    pub data: Option<String>,

    pub gas_limit: Option<String>,

    pub nonce: Option<ChainId>,

    pub to: Option<String>,

    pub value: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,

    pub gas_price: Option<String>,

    pub psbt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyTransferRequest {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub metadata: StickyMetadata,

    pub network: BlockchainEventNetwork,

    pub reason: Option<String>,

    pub request_body: IndecentRequestBody,

    pub requester: AmbitiousRequester,

    pub status: TransactionRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyMetadata {
    pub asset: StickyAsset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentRequestBody {
    pub amount: Option<String>,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: TentacledKind,

    pub memo: Option<String>,

    pub priority: Option<Priority>,

    pub to: String,

    pub asset_id: Option<String>,

    pub metadata: Option<String>,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApprovalResponseDecision {
    pub date: String,

    pub reason: Option<String>,

    pub user_id: String,

    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApprovalResponsePolicyEvaluation {
    pub policy_id: String,

    pub reason: String,

    pub triggered: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApprovalRequest {
    pub approval_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyParams {
    pub policy_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyResponse {
    pub action: GetPolicyResponseAction,

    pub activity_kind: ActivityKindEnum,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<GetPolicyResponseFilters>,

    pub id: String,

    pub name: String,

    pub pending_change_request: Option<GetPolicyResponsePendingChangeRequest>,

    pub rule: GetPolicyResponseRule,

    pub status: ArchivePolicyResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyResponseAction {
    pub approval_groups: Option<Vec<IndecentApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentApprovalGroup {
    pub approvers: IndecentApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentApprovers {
    pub user_id: Option<IndecentUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyResponseFilters {
    pub permission_id: Option<IndecentPermissionId>,

    pub policy_id: Option<IndecentPolicyId>,

    pub wallet_id: Option<IndecentWalletId>,

    pub wallet_tags: Option<IndecentWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyResponsePendingChangeRequest {
    pub approval_id: Option<String>,

    pub body: TentacledBody,

    pub date_created: String,

    pub date_resolved: Option<String>,

    pub entity_id: String,

    pub id: String,

    pub kind: PendingChangeRequestKind,

    pub operation_kind: PendingChangeRequestOperationKind,

    pub requester: CunningRequester,

    pub status: ChangeRequestStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledBody {
    pub action: TentacledAction,

    pub activity_kind: ActivityKindEnum,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<TentacledFilters>,

    pub id: String,

    pub name: String,

    pub rule: TentacledRule,

    pub status: ArchivePolicyResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledAction {
    pub approval_groups: Option<Vec<HilariousApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousApprovalGroup {
    pub approvers: HilariousApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousApprovers {
    pub user_id: Option<HilariousUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledFilters {
    pub permission_id: Option<HilariousPermissionId>,

    pub policy_id: Option<HilariousPolicyId>,

    pub wallet_id: Option<HilariousWalletId>,

    pub wallet_tags: Option<HilariousWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledRule {
    pub configuration: Option<IndecentConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentConfiguration {
    pub addresses: Option<Addresses5>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<IndecentAlerts>,

    pub exposures: Option<IndecentExposures>,

    pub fallback_behaviours: Option<IndecentFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses5 {
    IndecentAddresses(IndecentAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentExposures {
    pub direct: IndecentDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PendingChangeRequestKind {
    Policy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PendingChangeRequestOperationKind {
    Update,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPolicyResponseRule {
    pub configuration: Option<HilariousConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousConfiguration {
    pub addresses: Option<Addresses6>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<HilariousAlerts>,

    pub exposures: Option<HilariousExposures>,

    pub fallback_behaviours: Option<HilariousFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses6 {
    HilariousAddresses(HilariousAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousExposures {
    pub direct: HilariousDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyRequest {
    pub policy_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListApprovalsQuery {
    pub approver_id: Option<String>,

    pub initiator_id: Option<String>,

    pub limit: Option<String>,

    pub pagination_token: Option<String>,

    pub status: Option<ListApprovalsQueryStatus>,
}

impl std::fmt::Display for ListApprovalsQueryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Approved => write!(f, "Approved"),
            Self::AutoApproved => write!(f, "AutoApproved"),
            Self::AutoDenied => write!(f, "AutoDenied"),
            Self::Denied => write!(f, "Denied"),
            Self::Expired => write!(f, "Expired"),
            Self::Pending => write!(f, "Pending"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListApprovalsQueryStatus {
    Approved,

    #[serde(rename = "AutoApproved")]
    AutoApproved,

    #[serde(rename = "AutoDenied")]
    AutoDenied,

    Denied,

    Expired,

    Pending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListApprovalsResponse {
    pub items: Vec<ListApprovalsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListApprovalsResponseItem {
    pub activity: ItemActivity,

    pub date_created: Option<String>,

    pub date_resolved: Option<String>,

    pub date_updated: String,

    pub decisions: Vec<ItemDecision>,

    pub expiration_date: Option<String>,

    pub id: String,

    pub initiator_id: String,

    pub policy_evaluations: Vec<ItemPolicyEvaluation>,

    pub status: CreateApprovalDecisionResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemActivity {
    pub kind: ActivityKindEnum,

    pub signature_request: Option<TentacledSignatureRequest>,

    pub transaction_request: Option<TentacledTransactionRequest>,

    pub transfer_request: Option<TentacledTransferRequest>,

    pub blockchain_event: Option<TentacledBlockchainEvent>,

    pub change_request: Option<TentacledChangeRequest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledBlockchainEvent {
    pub balance_id: Option<String>,

    pub block_number: f64,

    pub decimals: Option<f64>,

    pub direction: Direction,

    pub fee: Option<String>,

    pub from: Option<String>,

    pub index: Option<String>,

    pub kind: BlockchainEventKind,

    pub liquidity_pool: Option<String>,

    pub memo: Option<String>,

    pub metadata: IndigoMetadata,

    pub network: BlockchainEventNetwork,

    pub symbol: Option<String>,

    pub timestamp: String,

    pub to: Option<String>,

    pub tx_hash: String,

    pub value: Option<String>,

    pub verified: Option<bool>,

    pub wallet_id: String,

    pub metadata_address: Option<String>,

    pub asset_id: Option<String>,

    pub clawback: Option<bool>,

    pub opt_in: Option<bool>,

    pub opt_out: Option<bool>,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,

    pub froms: Option<Vec<String>>,

    pub tos: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoMetadata {
    pub asset: IndigoAsset,

    pub fee: Option<TentacledFee>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledFee {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledChangeRequest {
    pub approval_id: Option<String>,

    pub body: StickyBody,

    pub date_created: String,

    pub date_resolved: Option<String>,

    pub entity_id: String,

    pub id: String,

    pub kind: ChangeRequestKind,

    pub operation_kind: ChangeRequestOperationKind,

    pub requester: MagentaRequester,

    pub status: ChangeRequestStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyBody {
    pub action: Option<StickyAction>,

    pub activity_kind: Option<ActivityKindEnum>,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<StickyFilters>,

    pub id: String,

    pub name: Option<String>,

    pub rule: Option<StickyRule>,

    pub status: Option<ArchivePolicyResponseStatus>,

    pub is_archived: Option<bool>,

    pub is_immutable: Option<bool>,

    pub operations: Option<Vec<String>>,

    pub identity_id: Option<String>,

    pub permission_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyAction {
    pub approval_groups: Option<Vec<AmbitiousApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbitiousApprovalGroup {
    pub approvers: AmbitiousApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousApprovers {
    pub user_id: Option<AmbitiousUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbitiousUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyFilters {
    pub permission_id: Option<AmbitiousPermissionId>,

    pub policy_id: Option<AmbitiousPolicyId>,

    pub wallet_id: Option<AmbitiousWalletId>,

    pub wallet_tags: Option<AmbitiousWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbitiousPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbitiousPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbitiousWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyRule {
    pub configuration: Option<AmbitiousConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousConfiguration {
    pub addresses: Option<Addresses7>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<AmbitiousAlerts>,

    pub exposures: Option<AmbitiousExposures>,

    pub fallback_behaviours: Option<AmbitiousFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses7 {
    AmbitiousAddresses(AmbitiousAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbitiousExposures {
    pub direct: AmbitiousDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledSignatureRequest {
    pub approval_id: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub date_signed: Option<String>,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: SignatureRequestNetwork,

    pub reason: Option<String>,

    pub request_body: HilariousRequestBody,

    pub requester: FriskyRequester,

    pub signature: Option<IndigoSignature>,

    pub signatures: Option<Vec<IndecentSignature>>,

    pub signed_data: Option<String>,

    pub status: SignatureRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousRequestBody {
    pub external_id: Option<String>,

    pub kind: PurpleKind,

    pub sign_doc: Option<String>,

    pub hash: Option<String>,

    pub taproot_merkle_root: Option<String>,

    pub message: Option<Message>,

    pub transaction: Option<String>,

    pub domain: Option<TentacledDomain>,

    pub types: Option<HashMap<String, Vec<TentacledType>>>,

    pub psbt: Option<String>,

    pub format: Option<Format>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledDomain {
    pub chain_id: Option<ChainId>,

    pub name: Option<String>,

    pub salt: Option<String>,

    pub verifying_contract: Option<String>,

    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledType {
    pub name: String,

    #[serde(rename = "type")]
    pub type_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoSignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentSignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledTransactionRequest {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: BlockchainEventNetwork,

    pub reason: Option<String>,

    pub request_body: AmbitiousRequestBody,

    pub requester: MischievousRequester,

    pub status: TransactionRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousRequestBody {
    pub external_id: Option<String>,

    pub kind: FluffyKind,

    pub transaction: Option<String>,

    pub data: Option<String>,

    pub gas_limit: Option<String>,

    pub nonce: Option<ChainId>,

    pub to: Option<String>,

    pub value: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,

    pub gas_price: Option<String>,

    pub psbt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledTransferRequest {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub metadata: IndecentMetadata,

    pub network: BlockchainEventNetwork,

    pub reason: Option<String>,

    pub request_body: CunningRequestBody,

    pub requester: BraggadociousRequester,

    pub status: TransactionRequestStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentMetadata {
    pub asset: IndecentAsset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningRequestBody {
    pub amount: Option<String>,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: TentacledKind,

    pub memo: Option<String>,

    pub priority: Option<Priority>,

    pub to: String,

    pub asset_id: Option<String>,

    pub metadata: Option<String>,

    pub contract: Option<String>,

    pub token_id: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BraggadociousRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDecision {
    pub date: String,

    pub reason: Option<String>,

    pub user_id: String,

    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPolicyEvaluation {
    pub policy_id: String,

    pub reason: String,

    pub triggered: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListApprovalsRequest {
    pub query: Option<ListApprovalsRequestQuery>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListApprovalsRequestQuery {
    pub approver_id: Option<String>,

    pub initiator_id: Option<String>,

    pub limit: Option<String>,

    pub pagination_token: Option<String>,

    pub status: Option<ListApprovalsQueryStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPoliciesQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,

    pub status: Option<ArchivePolicyResponseStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPoliciesResponse {
    pub items: Vec<ListPoliciesResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPoliciesResponseItem {
    pub action: ItemAction,

    pub activity_kind: ActivityKindEnum,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<ItemFilters>,

    pub id: String,

    pub name: String,

    pub pending_change_request: Option<ItemPendingChangeRequest>,

    pub rule: ItemRule,

    pub status: ArchivePolicyResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemAction {
    pub approval_groups: Option<Vec<CunningApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CunningApprovalGroup {
    pub approvers: CunningApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningApprovers {
    pub user_id: Option<CunningUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CunningUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemFilters {
    pub permission_id: Option<CunningPermissionId>,

    pub policy_id: Option<CunningPolicyId>,

    pub wallet_id: Option<CunningWalletId>,

    pub wallet_tags: Option<CunningWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CunningPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CunningPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CunningWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPendingChangeRequest {
    pub approval_id: Option<String>,

    pub body: IndigoBody,

    pub date_created: String,

    pub date_resolved: Option<String>,

    pub entity_id: String,

    pub id: String,

    pub kind: PendingChangeRequestKind,

    pub operation_kind: PendingChangeRequestOperationKind,

    pub requester: Requester1,

    pub status: ChangeRequestStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoBody {
    pub action: IndigoAction,

    pub activity_kind: ActivityKindEnum,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<IndigoFilters>,

    pub id: String,

    pub name: String,

    pub rule: IndigoRule,

    pub status: ArchivePolicyResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoAction {
    pub approval_groups: Option<Vec<MagentaApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagentaApprovalGroup {
    pub approvers: MagentaApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaApprovers {
    pub user_id: Option<MagentaUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagentaUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoFilters {
    pub permission_id: Option<MagentaPermissionId>,

    pub policy_id: Option<MagentaPolicyId>,

    pub wallet_id: Option<MagentaWalletId>,

    pub wallet_tags: Option<MagentaWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagentaPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagentaPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagentaWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoRule {
    pub configuration: Option<CunningConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningConfiguration {
    pub addresses: Option<Addresses8>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<CunningAlerts>,

    pub exposures: Option<CunningExposures>,

    pub fallback_behaviours: Option<CunningFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses8 {
    CunningAddresses(CunningAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CunningExposures {
    pub direct: CunningDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requester1 {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemRule {
    pub configuration: Option<MagentaConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaConfiguration {
    pub addresses: Option<Addresses9>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<MagentaAlerts>,

    pub exposures: Option<MagentaExposures>,

    pub fallback_behaviours: Option<MagentaFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses9 {
    MagentaAddresses(MagentaAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagentaExposures {
    pub direct: MagentaDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListPoliciesRequest {
    pub query: Option<ListPoliciesRequestQuery>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPoliciesRequestQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,

    pub status: Option<ArchivePolicyResponseStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyParams {
    pub policy_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyResponse {
    pub action: UpdatePolicyResponseAction,

    pub activity_kind: ActivityKindEnum,

    pub date_created: Option<String>,

    pub date_updated: Option<String>,

    pub filters: Option<UpdatePolicyResponseFilters>,

    pub id: String,

    pub name: String,

    pub rule: UpdatePolicyResponseRule,

    pub status: ArchivePolicyResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyResponseAction {
    pub approval_groups: Option<Vec<MischievousApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MischievousApprovalGroup {
    pub approvers: MischievousApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousApprovers {
    pub user_id: Option<MischievousUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MischievousUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyResponseFilters {
    pub permission_id: Option<MischievousPermissionId>,

    pub policy_id: Option<MischievousPolicyId>,

    pub wallet_id: Option<MischievousWalletId>,

    pub wallet_tags: Option<MischievousWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MischievousPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MischievousPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MischievousWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePolicyResponseRule {
    pub configuration: Option<MischievousConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousConfiguration {
    pub addresses: Option<Addresses11>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<MischievousAlerts>,

    pub exposures: Option<MischievousExposures>,

    pub fallback_behaviours: Option<MischievousFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses11 {
    MischievousAddresses(MischievousAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MischievousExposures {
    pub direct: MischievousDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MischievousFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyRequest {
    pub body: UpdatePolicyBody,

    pub policy_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyBody {
    pub action: UpdatePolicyBodyAction,

    pub activity_kind: ActivityKindEnum,

    pub filters: Option<UpdatePolicyBodyFilters>,

    pub name: String,

    pub rule: UpdatePolicyBodyRule,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyBodyAction {
    pub approval_groups: Option<Vec<FriskyApprovalGroup>>,

    pub auto_reject_timeout: Option<f64>,

    pub kind: ActionKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriskyApprovalGroup {
    pub approvers: FriskyApprovers,

    pub name: Option<String>,

    pub quorum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyApprovers {
    pub user_id: Option<FriskyUserId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriskyUserId {
    #[serde(rename = "in")]
    pub user_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyBodyFilters {
    pub permission_id: Option<FriskyPermissionId>,

    pub policy_id: Option<FriskyPolicyId>,

    pub wallet_id: Option<FriskyWalletId>,

    pub wallet_tags: Option<FriskyWalletTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriskyPermissionId {
    #[serde(rename = "in")]
    pub permission_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriskyPolicyId {
    #[serde(rename = "in")]
    pub policy_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriskyWalletId {
    #[serde(rename = "in")]
    pub wallet_id_in: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyWalletTags {
    pub has_all: Option<Vec<String>>,

    pub has_any: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePolicyBodyRule {
    pub configuration: Option<FriskyConfiguration>,

    pub kind: RuleKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyConfiguration {
    pub addresses: Option<Addresses10>,

    pub currency: Option<Currency>,

    pub limit: Option<f64>,

    pub timeframe: Option<f64>,

    pub alerts: Option<FriskyAlerts>,

    pub exposures: Option<FriskyExposures>,

    pub fallback_behaviours: Option<FriskyFallbackBehaviours>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Addresses10 {
    FriskyAddresses(FriskyAddresses),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyAddresses {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyAlerts {
    pub alert_level: AlertLevel,

    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriskyExposures {
    pub direct: FriskyDirect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyDirect {
    pub category_ids: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyFallbackBehaviours {
    pub skip_chainalysis_failure: bool,

    pub skip_unscreenable_transaction: bool,

    pub skip_unsupported_asset: bool,

    pub skip_unsupported_network: bool,
}
