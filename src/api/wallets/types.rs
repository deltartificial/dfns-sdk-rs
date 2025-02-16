// @dfns-sdk-rs/src/api/wallets/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type TagWalletResponse = HashMap<String, Option<serde_json::Value>>;
pub type UntagWalletResponse = HashMap<String, Option<serde_json::Value>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTransactionParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTransactionResponse {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: BroadcastTransactionResponseNetwork,

    pub reason: Option<String>,

    pub request_body: BroadcastTransactionResponseRequestBody,

    pub requester: BroadcastTransactionResponseRequester,

    pub status: BroadcastTransactionResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BroadcastTransactionResponseNetwork {
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

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTransactionResponseRequestBody {
    pub external_id: Option<String>,

    pub kind: BroadcastTransactionBodyKind,

    pub transaction: Option<String>,

    pub data: Option<String>,

    pub gas_limit: Option<String>,

    pub nonce: Option<Nonce>,

    pub to: Option<String>,

    pub value: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,

    pub gas_price: Option<String>,

    pub psbt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BroadcastTransactionBodyKind {
    Eip1559,

    Evm,

    #[serde(rename = "EvmLegacy")]
    EvmLegacy,

    Psbt,

    Transaction,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nonce {
    Double(f64),

    #[default]
    #[serde(rename = "*")]
    None,

    String(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTransactionResponseRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BroadcastTransactionResponseStatus {
    Broadcasted,

    Confirmed,

    Executing,

    Failed,

    Pending,

    Rejected,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTransactionRequest {
    pub body: BroadcastTransactionBody,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTransactionBody {
    pub external_id: Option<String>,

    pub kind: BroadcastTransactionBodyKind,

    pub transaction: Option<String>,

    pub data: Option<String>,

    pub gas_limit: Option<String>,

    pub nonce: Option<Nonce>,

    pub to: Option<String>,

    pub value: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,

    pub gas_price: Option<String>,

    pub psbt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletBody {
    pub delay_delegation: Option<bool>,

    pub delegate_to: Option<String>,

    pub external_id: Option<String>,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub signing_key: Option<CreateWalletBodySigningKey>,

    pub tags: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateWalletBodyNetwork {
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

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWalletBodySigningKey {
    pub curve: Option<Curve>,

    pub scheme: Option<Scheme>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Curve {
    Ed25519,

    Secp256K1,

    Stark,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Scheme {
    #[serde(rename = "ECDSA")]
    Ecdsa,

    #[serde(rename = "EdDSA")]
    EdDsa,

    Schnorr,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletResponse {
    pub address: Option<String>,

    pub custodial: bool,

    pub date_created: String,

    pub date_exported: Option<String>,

    pub exported: Option<bool>,

    pub external_id: Option<String>,

    pub id: String,

    pub imported: Option<bool>,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub signing_key: CreateWalletResponseSigningKey,

    pub status: CreateWalletResponseStatus,

    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletResponseSigningKey {
    pub curve: Curve,

    pub public_key: String,

    pub scheme: Scheme,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateWalletResponseStatus {
    Active,

    Archived,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub body: CreateWalletRequestBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletRequestBody {
    pub delay_delegation: Option<bool>,

    pub delegate_to: Option<String>,

    pub external_id: Option<String>,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub signing_key: Option<BodySigningKey>,

    pub tags: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodySigningKey {
    pub curve: Option<Curve>,

    pub scheme: Option<Scheme>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWalletBody {
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWalletParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWalletResponse {
    pub status: DelegateWalletResponseStatus,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DelegateWalletResponseStatus {
    Delegated,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWalletRequest {
    pub body: DelegateWalletRequestBody,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWalletRequestBody {
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportWalletBody {
    pub encryption_key: String,

    pub supported_schemes: Vec<ExportWalletBodySupportedScheme>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportWalletBodySupportedScheme {
    pub curve: Curve,

    pub protocol: Protocol,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Protocol {
    Cggmp21,

    Frost,

    #[serde(rename = "FROST_BITCOIN")]
    FrostBitcoin,

    Ku23,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportWalletParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportWalletResponse {
    pub curve: Curve,

    /// Keyshares of the exported wallet. They are encrypted with the provided encryption key.
    /// The exported private key is re-constructed from these keyshares.
    pub encrypted_key_shares: Vec<ExportWalletResponseEncryptedKeyShare>,

    /// The TSS threshold of the wallet private signing key shares
    pub min_signers: f64,

    pub protocol: Protocol,

    pub public_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportWalletResponseEncryptedKeyShare {
    /// Base64-encoded keyshare
    pub encrypted_key_share: String,

    /// Base64-encoded ID of the signer exported the encrypted keyshare
    pub signer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportWalletRequest {
    pub body: ExportWalletRequestBody,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportWalletRequestBody {
    pub encryption_key: String,

    pub supported_schemes: Vec<BodySupportedScheme>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodySupportedScheme {
    pub curve: Curve,

    pub protocol: Protocol,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSignatureParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSignatureResponse {
    pub approval_id: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub date_signed: Option<String>,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: CreateWalletBodyNetwork,

    pub reason: Option<String>,

    pub request_body: GenerateSignatureResponseRequestBody,

    pub requester: GenerateSignatureResponseRequester,

    pub signature: Option<PurpleSignature>,

    pub signatures: Option<Vec<FluffySignature>>,

    pub signed_data: Option<String>,

    pub status: GenerateSignatureResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSignatureResponseRequestBody {
    pub external_id: Option<String>,

    pub kind: GenerateSignatureBodyKind,

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleDomain {
    pub chain_id: Option<Nonce>,

    pub name: Option<String>,

    pub salt: Option<String>,

    pub verifying_contract: Option<String>,

    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Format {
    Full,

    Simple,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenerateSignatureBodyKind {
    Bip322,

    Eip712,

    Hash,

    Message,

    Psbt,

    #[serde(rename = "SignDocDirect")]
    SignDocDirect,

    Transaction,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),

    String(String),

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleType {
    pub name: String,

    #[serde(rename = "type")]
    pub type_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSignatureResponseRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleSignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffySignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenerateSignatureResponseStatus {
    Confirmed,

    Executing,

    Failed,

    Pending,

    Rejected,

    Signed,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSignatureRequest {
    pub body: GenerateSignatureBody,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSignatureBody {
    pub external_id: Option<String>,

    pub kind: GenerateSignatureBodyKind,

    pub sign_doc: Option<String>,

    pub hash: Option<String>,

    pub taproot_merkle_root: Option<String>,

    pub message: Option<Message>,

    pub transaction: Option<String>,

    pub domain: Option<GenerateSignatureBodyDomain>,

    pub types: Option<HashMap<String, Vec<GenerateSignatureBodyType>>>,

    pub psbt: Option<String>,

    pub format: Option<Format>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSignatureBodyDomain {
    pub chain_id: Option<Nonce>,

    pub name: Option<String>,

    pub salt: Option<String>,

    pub verifying_contract: Option<String>,

    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateSignatureBodyType {
    pub name: String,

    #[serde(rename = "type")]
    pub type_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignatureParams {
    pub signature_id: String,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignatureResponse {
    pub approval_id: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub date_signed: Option<String>,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: CreateWalletBodyNetwork,

    pub reason: Option<String>,

    pub request_body: GetSignatureResponseRequestBody,

    pub requester: GetSignatureResponseRequester,

    pub signature: Option<TentacledSignature>,

    pub signatures: Option<Vec<StickySignature>>,

    pub signed_data: Option<String>,

    pub status: GenerateSignatureResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignatureResponseRequestBody {
    pub external_id: Option<String>,

    pub kind: GenerateSignatureBodyKind,

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyDomain {
    pub chain_id: Option<Nonce>,

    pub name: Option<String>,

    pub salt: Option<String>,

    pub verifying_contract: Option<String>,

    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyType {
    pub name: String,

    #[serde(rename = "type")]
    pub type_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignatureResponseRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledSignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickySignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignatureRequest {
    pub signature_id: String,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionParams {
    pub transaction_id: String,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionResponse {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: BroadcastTransactionResponseNetwork,

    pub reason: Option<String>,

    pub request_body: GetTransactionResponseRequestBody,

    pub requester: GetTransactionResponseRequester,

    pub status: BroadcastTransactionResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionResponseRequestBody {
    pub external_id: Option<String>,

    pub kind: BroadcastTransactionBodyKind,

    pub transaction: Option<String>,

    pub data: Option<String>,

    pub gas_limit: Option<String>,

    pub nonce: Option<Nonce>,

    pub to: Option<String>,

    pub value: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,

    pub gas_price: Option<String>,

    pub psbt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionResponseRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionRequest {
    pub transaction_id: String,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferParams {
    pub transfer_id: String,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferResponse {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub metadata: GetTransferResponseMetadata,

    pub network: BroadcastTransactionResponseNetwork,

    pub reason: Option<String>,

    pub request_body: GetTransferResponseRequestBody,

    pub requester: GetTransferResponseRequester,

    pub status: BroadcastTransactionResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransferResponseMetadata {
    pub asset: PurpleAsset,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferResponseRequestBody {
    pub amount: Option<String>,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: TransferAssetBodyKind,

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransferAssetBodyKind {
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

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Fast,

    Slow,

    Standard,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferResponseRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferRequest {
    pub transfer_id: String,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletResponse {
    pub address: Option<String>,

    pub custodial: bool,

    pub date_created: String,

    pub date_exported: Option<String>,

    pub exported: Option<bool>,

    pub external_id: Option<String>,

    pub id: String,

    pub imported: Option<bool>,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub signing_key: GetWalletResponseSigningKey,

    pub status: CreateWalletResponseStatus,

    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletResponseSigningKey {
    pub curve: Curve,

    pub public_key: String,

    pub scheme: Scheme,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletRequest {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletAssetsParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletAssetsQuery {
    pub net_worth: Option<NetWorth>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetWorth {
    True,

    #[default]
    #[serde(rename = "*")]
    None,
}

impl std::fmt::Display for NetWorth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetWorth::True => write!(f, "true"),
            NetWorth::None => write!(f, "*"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletAssetsResponse {
    pub assets: Vec<AssetElement>,

    pub network: BroadcastTransactionResponseNetwork,

    pub net_worth: Option<HashMap<String, f64>>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetElement {
    pub balance: String,

    pub decimals: f64,

    pub kind: AssetKind,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,

    pub metadata: Option<String>,

    pub asset_id: Option<String>,

    pub contract: Option<String>,

    pub asset_code: Option<String>,

    pub issuer: Option<String>,

    pub token_id: Option<String>,

    pub mint: Option<String>,

    pub master: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetKind {
    Aip21,

    Asa,

    Erc20,

    Native,

    Sep41,

    Spl,

    Spl2022,

    Tep74,

    Trc10,

    Trc20,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletAssetsRequest {
    pub query: Option<GetWalletAssetsRequestQuery>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletAssetsRequestQuery {
    pub net_worth: Option<NetWorth>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletHistoryParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletHistoryQuery {
    pub contract: Option<String>,

    pub direction: Option<Direction>,

    pub kind: Option<GetWalletHistoryQueryKind>,

    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    In,

    Out,

    #[default]
    #[serde(rename = "*")]
    None,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::In => write!(f, "in"),
            Direction::Out => write!(f, "out"),
            Direction::None => write!(f, "*"),
        }
    }
}

impl std::fmt::Display for GetWalletHistoryQueryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetWalletHistoryQueryKind::Aip21Transfer => write!(f, "Aip21Transfer"),
            GetWalletHistoryQueryKind::AsaTransfer => write!(f, "AsaTransfer"),
            GetWalletHistoryQueryKind::Erc20Transfer => write!(f, "Erc20Transfer"),
            GetWalletHistoryQueryKind::Erc721Transfer => write!(f, "Erc721Transfer"),
            GetWalletHistoryQueryKind::NativeTransfer => write!(f, "NativeTransfer"),
            GetWalletHistoryQueryKind::Sep41Transfer => write!(f, "Sep41Transfer"),
            GetWalletHistoryQueryKind::Spl2022Transfer => write!(f, "Spl2022Transfer"),
            GetWalletHistoryQueryKind::SplTransfer => write!(f, "SplTransfer"),
            GetWalletHistoryQueryKind::Tep74Transfer => write!(f, "Tep74Transfer"),
            GetWalletHistoryQueryKind::Trc10Transfer => write!(f, "Trc10Transfer"),
            GetWalletHistoryQueryKind::Trc20Transfer => write!(f, "Trc20Transfer"),
            GetWalletHistoryQueryKind::Trc721Transfer => write!(f, "Trc721Transfer"),
            GetWalletHistoryQueryKind::UtxoTransfer => write!(f, "UtxoTransfer"),
            GetWalletHistoryQueryKind::None => write!(f, "*"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetWalletHistoryQueryKind {
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

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletHistoryResponse {
    pub items: Vec<GetWalletHistoryResponseItem>,

    pub network: BroadcastTransactionResponseNetwork,

    pub next_page_token: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletHistoryResponseItem {
    pub balance_id: Option<String>,

    pub block_number: f64,

    pub decimals: Option<f64>,

    pub direction: Direction,

    pub fee: Option<String>,

    pub from: Option<String>,

    pub index: Option<String>,

    pub kind: GetWalletHistoryQueryKind,

    pub liquidity_pool: Option<String>,

    pub memo: Option<String>,

    pub metadata: PurpleMetadata,

    pub network: BroadcastTransactionResponseNetwork,

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleMetadata {
    pub asset: FluffyAsset,

    pub fee: Option<Fee>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fee {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletHistoryRequest {
    pub query: Option<GetWalletHistoryRequestQuery>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletHistoryRequestQuery {
    pub contract: Option<String>,

    pub direction: Option<Direction>,

    pub kind: Option<GetWalletHistoryQueryKind>,

    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletNftsParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletNftsResponse {
    pub network: BroadcastTransactionResponseNetwork,

    pub nfts: Vec<Nft>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nft {
    pub asset_id: Option<String>,

    pub kind: NftKind,

    pub symbol: Option<String>,

    pub token_uri: Option<String>,

    pub contract: Option<String>,

    pub token_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NftKind {
    Asa,

    Erc721,

    Trc721,

    #[default]
    #[serde(rename = "*")]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletNftsRequest {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportWalletBody {
    pub curve: Curve,

    pub encrypted_key_shares: Vec<ImportWalletBodyEncryptedKeyShare>,

    pub external_id: Option<String>,

    pub min_signers: f64,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub protocol: Protocol,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportWalletBodyEncryptedKeyShare {
    pub encrypted_key_share: String,

    pub signer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportWalletResponse {
    pub address: Option<String>,

    pub custodial: bool,

    pub date_created: String,

    pub date_exported: Option<String>,

    pub exported: Option<bool>,

    pub external_id: Option<String>,

    pub id: String,

    pub imported: Option<bool>,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub signing_key: ImportWalletResponseSigningKey,

    pub status: CreateWalletResponseStatus,

    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportWalletResponseSigningKey {
    pub curve: Curve,

    pub public_key: String,

    pub scheme: Scheme,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportWalletRequest {
    pub body: ImportWalletRequestBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportWalletRequestBody {
    pub curve: Curve,

    pub encrypted_key_shares: Vec<BodyEncryptedKeyShare>,

    pub external_id: Option<String>,

    pub min_signers: f64,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub protocol: Protocol,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyEncryptedKeyShare {
    pub encrypted_key_share: String,

    pub signer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSignaturesParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSignaturesQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSignaturesResponse {
    pub items: Vec<ListSignaturesResponseItem>,

    pub next_page_token: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSignaturesResponseItem {
    pub approval_id: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub date_signed: Option<String>,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: CreateWalletBodyNetwork,

    pub reason: Option<String>,

    pub request_body: PurpleRequestBody,

    pub requester: PurpleRequester,

    pub signature: Option<IndigoSignature>,

    pub signatures: Option<Vec<IndecentSignature>>,

    pub signed_data: Option<String>,

    pub status: GenerateSignatureResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRequestBody {
    pub external_id: Option<String>,

    pub kind: GenerateSignatureBodyKind,

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledDomain {
    pub chain_id: Option<Nonce>,

    pub name: Option<String>,

    pub salt: Option<String>,

    pub verifying_contract: Option<String>,

    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledType {
    pub name: String,

    #[serde(rename = "type")]
    pub type_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoSignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentSignature {
    pub encoded: Option<String>,

    pub r: String,

    pub recid: Option<f64>,

    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSignaturesRequest {
    pub query: Option<ListSignaturesRequestQuery>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSignaturesRequestQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsResponse {
    pub items: Vec<ListTransactionsResponseItem>,

    pub next_page_token: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsResponseItem {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub network: BroadcastTransactionResponseNetwork,

    pub reason: Option<String>,

    pub request_body: FluffyRequestBody,

    pub requester: FluffyRequester,

    pub status: BroadcastTransactionResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRequestBody {
    pub external_id: Option<String>,

    pub kind: BroadcastTransactionBodyKind,

    pub transaction: Option<String>,

    pub data: Option<String>,

    pub gas_limit: Option<String>,

    pub nonce: Option<Nonce>,

    pub to: Option<String>,

    pub value: Option<String>,

    pub max_fee_per_gas: Option<String>,

    pub max_priority_fee_per_gas: Option<String>,

    pub gas_price: Option<String>,

    pub psbt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsRequest {
    pub query: Option<ListTransactionsRequestQuery>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsRequestQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransfersParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransfersQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransfersResponse {
    pub items: Vec<ListTransfersResponseItem>,

    pub next_page_token: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransfersResponseItem {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub metadata: FluffyMetadata,

    pub network: BroadcastTransactionResponseNetwork,

    pub reason: Option<String>,

    pub request_body: TentacledRequestBody,

    pub requester: TentacledRequester,

    pub status: BroadcastTransactionResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyMetadata {
    pub asset: TentacledAsset,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledRequestBody {
    pub amount: Option<String>,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: TransferAssetBodyKind,

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransfersRequest {
    pub query: Option<ListTransfersRequestQuery>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransfersRequestQuery {
    pub limit: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsQuery {
    pub limit: Option<String>,

    pub owner_id: Option<String>,

    pub owner_username: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsResponse {
    pub items: Vec<ListWalletsResponseItem>,

    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsResponseItem {
    pub address: Option<String>,

    pub custodial: bool,

    pub date_created: String,

    pub date_exported: Option<String>,

    pub exported: Option<bool>,

    pub external_id: Option<String>,

    pub id: String,

    pub imported: Option<bool>,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub signing_key: ItemSigningKey,

    pub status: CreateWalletResponseStatus,

    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSigningKey {
    pub curve: Curve,

    pub public_key: String,

    pub scheme: Scheme,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListWalletsRequest {
    pub query: Option<ListWalletsRequestQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsRequestQuery {
    pub limit: Option<String>,

    pub owner_id: Option<String>,

    pub owner_username: Option<String>,

    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagWalletBody {
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagWalletParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagWalletRequest {
    pub body: TagWalletRequestBody,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagWalletRequestBody {
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetResponse {
    pub approval_id: Option<String>,

    pub date_broadcasted: Option<String>,

    pub date_confirmed: Option<String>,

    pub date_policy_resolved: Option<String>,

    pub date_requested: String,

    pub external_id: Option<String>,

    pub fee: Option<String>,

    pub id: String,

    pub metadata: TransferAssetResponseMetadata,

    pub network: BroadcastTransactionResponseNetwork,

    pub reason: Option<String>,

    pub request_body: TransferAssetResponseRequestBody,

    pub requester: TransferAssetResponseRequester,

    pub status: BroadcastTransactionResponseStatus,

    pub tx_hash: Option<String>,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferAssetResponseMetadata {
    pub asset: StickyAsset,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyAsset {
    pub decimals: Option<f64>,

    pub quotes: Option<HashMap<String, f64>>,

    pub symbol: Option<String>,

    pub verified: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetResponseRequestBody {
    pub amount: Option<String>,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: TransferAssetBodyKind,

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetResponseRequester {
    pub app_id: Option<String>,

    pub token_id: Option<String>,

    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetRequest {
    pub body: TransferAssetBody,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetBody {
    pub amount: Option<String>,

    pub create_destination_account: Option<bool>,

    pub external_id: Option<String>,

    pub kind: TransferAssetBodyKind,

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UntagWalletBody {
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntagWalletParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntagWalletRequest {
    pub body: UntagWalletRequestBody,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UntagWalletRequestBody {
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletBody {
    pub external_id: Option<String>,

    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletParams {
    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletResponse {
    pub address: Option<String>,

    pub custodial: bool,

    pub date_created: String,

    pub date_exported: Option<String>,

    pub exported: Option<bool>,

    pub external_id: Option<String>,

    pub id: String,

    pub imported: Option<bool>,

    pub name: Option<String>,

    pub network: CreateWalletBodyNetwork,

    pub signing_key: UpdateWalletResponseSigningKey,

    pub status: CreateWalletResponseStatus,

    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletResponseSigningKey {
    pub curve: Curve,

    pub public_key: String,

    pub scheme: Scheme,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletRequest {
    pub body: UpdateWalletRequestBody,

    pub wallet_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletRequestBody {
    pub external_id: Option<String>,

    pub name: Option<String>,
}
