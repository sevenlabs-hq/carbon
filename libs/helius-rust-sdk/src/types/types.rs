use super::{
    enums::{AssetSortBy, AssetSortDirection, Context, Interface, OwnershipModel, RoyaltyModel, Scope, UseMethod},
    AccountWebhookEncoding, CollectionIdentifier, PriorityLevel, SearchAssetsOptions, SearchConditionType, TokenType,
    TransactionStatus, TransactionType, UiTransactionEncoding, WebhookType,
};
use crate::types::{DisplayOptions, GetAssetOptions};
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::{address_lookup_table::AddressLookupTableAccount, instruction::Instruction, signature::Signer};

/// Defines the available clusters supported by Helius
#[derive(Debug, Clone, PartialEq)]
pub enum Cluster {
    Devnet,
    MainnetBeta,
    StakedMainnetBeta,
}

/// Stores the API and RPC endpoint URLs for a specific Helius cluster
#[derive(Debug, Clone)]
pub struct HeliusEndpoints {
    pub api: String,
    pub rpc: String,
}

impl HeliusEndpoints {
    pub fn for_cluster(cluster: &Cluster) -> Self {
        match cluster {
            Cluster::Devnet => HeliusEndpoints {
                api: "https://api-devnet.helius-rpc.com/".to_string(),
                rpc: "https://devnet.helius-rpc.com/".to_string(),
            },
            Cluster::MainnetBeta => HeliusEndpoints {
                api: "https://api.helius-rpc.com/".to_string(),
                rpc: "https://mainnet.helius-rpc.com/".to_string(),
            },
            Cluster::StakedMainnetBeta => HeliusEndpoints {
                api: "https://api.helius-rpc.com/".to_string(),
                rpc: "https://staked.helius-rpc.com/".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RpcRequest<T> {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    #[serde(rename = "params")]
    pub parameters: T,
}

impl<T> RpcRequest<T> {
    pub fn new(method: String, parameters: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method,
            parameters,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: T,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GetAssetsByOwner {
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    pub page: u32,
    pub limit: Option<i32>,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(rename = "displayOptions")]
    pub display_options: Option<DisplayOptions>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<AssetSorting>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GetAssetsByAuthority {
    #[serde(rename = "authorityAddress")]
    pub authority_address: String,
    pub page: u32,
    pub limit: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(rename = "displayOptions")]
    pub display_options: Option<DisplayOptions>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<AssetSorting>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetAsset {
    pub id: String,
    #[serde(rename = "displayOptions")]
    pub display_options: Option<GetAssetOptions>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetsByCreator {
    pub creator_address: String,
    pub only_verified: Option<bool>,
    pub sort_by: Option<AssetSorting>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default, alias = "displayOptions")]
    pub options: Option<DisplayOptions>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAssetBatch {
    pub ids: Vec<String>,
    #[serde(rename = "displayOptions")]
    pub display_options: Option<GetAssetOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAssetProof {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAssetProofBatch {
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetProof {
    pub root: String,
    pub proof: Vec<String>,
    pub node_index: i32,
    pub leaf: String,
    pub tree_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetsByGroup {
    pub group_key: String,
    pub group_value: String,
    pub sort_by: Option<AssetSorting>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default, alias = "displayOptions")]
    pub options: Option<DisplayOptions>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchAssets {
    pub negate: Option<bool>,
    pub condition_type: Option<SearchConditionType>,
    pub interface: Option<Interface>,
    pub owner_address: Option<String>,
    pub owner_type: Option<OwnershipModel>,
    pub creator_address: Option<String>,
    pub creator_verified: Option<bool>,
    pub authority_address: Option<String>,
    pub grouping: Option<(String, String)>,
    pub delegate: Option<String>,
    pub frozen: Option<bool>,
    pub supply: Option<u64>,
    pub supply_mint: Option<String>,
    pub compressed: Option<bool>,
    pub compressible: Option<bool>,
    pub royalty_target_type: Option<RoyaltyModel>,
    pub royalty_target: Option<String>,
    pub royalty_amount: Option<u32>,
    pub burnt: Option<bool>,
    pub sort_by: Option<AssetSorting>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default)]
    pub json_uri: Option<String>,
    #[serde(default)]
    pub not: Option<NotFilter>,
    #[serde(default, alias = "displayOptions")]
    pub options: Option<SearchAssetsOptions>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub collections: Option<Vec<String>>,
    #[serde(default)]
    pub token_type: Option<TokenType>,
    // #[serde(default)]
    // pub created_at: Option<CreatedAtFilter>, //TODO: Uncomment this line when the CreatedAtFilter struct is defined
    #[serde(default)]
    pub tree: Option<String>,
    #[serde(default)]
    pub collection_nft: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetSignatures {
    pub id: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub tree: Option<String>,
    pub leaf_index: Option<i64>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub sort_direction: Option<AssetSortDirection>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccounts {
    pub owner: Option<String>,
    pub mint: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default, alias = "displayOptions")]
    pub options: Option<DisplayOptions>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNftEditions {
    pub mint: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetSorting {
    pub sort_by: AssetSortBy,
    pub sort_direction: Option<AssetSortDirection>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ApiResponse<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssetList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grand_total: Option<u64>,
    pub total: u32,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub items: Vec<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AssetError>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct TransactionSignatureList {
    pub total: u32,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    pub items: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct TokenAccountsList {
    pub total: u32,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    pub token_accounts: Vec<TokenAccount>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EditionsList {
    pub total: u32,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    pub master_edition_address: String,
    pub supply: u64,
    pub max_supply: Option<u64>,
    pub editions: Vec<Edition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub interface: Interface,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorities: Option<Vec<Authorities>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<Compression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<Vec<Group>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty: Option<Royalty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<Creator>>,
    pub ownership: Ownership,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<Uses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply: Option<Supply>,
    pub mutable: bool,
    pub burnt: bool,
    pub mint_extensions: Option<Value>,
    #[serde(rename = "tokenSupply")]
    pub token_info: Option<TokenInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_definition: Option<GroupDefinition>,
    // pub system: Option<SystemInfo>, TODO: Uncomment this line when the SystemInfo struct is defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_plugins: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpl_core_info: Option<MplCoreInfo>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct AssetError {
    pub id: String,
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MintExtensions {
    pub confidential_transfer_mint: Option<ConfidentialTransferMint>,
    pub confidential_transfer_fee_config: Option<ConfidentialTransferFeeConfig>,
    pub transfer_fee_config: Option<TransferFeeConfig>,
    pub metadata_pointer: MetadataPointer,
    pub mint_close_authority: MintCloseAuthority,
    pub permanent_delegate: PermanentDelegate,
    pub transfer_hook: TransferHook,
    pub interest_bearing_config: InterestBearingConfig,
    pub default_account_state: DefaultAccountState,
    pub confidential_transfer_account: ConfidentialTransferAccount,
    pub metadata: MintExtensionMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfidentialTransferMint {
    pub authority: String,
    pub auto_approve_new_accounts: bool,
    pub auditor_elgamal_pubkey: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfidentialTransferFeeConfig {
    pub authority: String,
    pub withdraw_withheld_authority_elgamal_pubkey: String,
    pub harvest_to_mint_enabled: bool,
    pub withheld_amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferFeeConfig {
    pub transfer_fee_config_authority: String,
    pub withdraw_withheld_authority: String,
    pub withheld_amount: i32,
    pub older_transfer_fee: OlderTransferFee,
    pub new_transfer_fee: NewTransferFee,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OlderTransferFee {
    pub epoch: String,
    pub maximum_fee: String,
    pub transfer_fee_basis_points: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTransferFee {
    pub epoch: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataPointer {
    pub authority: String,
    #[serde(rename = "metadataAddress")]
    pub metadata_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MintCloseAuthority {
    pub close_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermanentDelegate {
    pub delegate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferHook {
    pub authority: String,
    #[serde(rename = "programId")]
    pub program_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InterestBearingConfig {
    pub rate_authority: String,
    pub initialization_timestamp: i32,
    pub pre_update_average_rate: i32,
    pub last_update_timestamp: i32,
    pub current_rate: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultAccountState {
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfidentialTransferAccount {
    pub approved: bool,
    pub elgamal_pubkey: String,
    pub pending_balance_lo: String,
    pub pending_balance_hi: String,
    pub available_balance: String,
    pub decryptable_available_balance: String,
    pub allow_confidential_credits: bool,
    pub allow_non_confidential_credits: bool,
    pub pending_balance_credit_counter: i32,
    pub maximum_pending_balance_credit_counter: i32,
    pub expected_pending_balance_credit_counter: i32,
    pub actual_pending_balance_credit_counter: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MintExtensionMetadata {
    #[serde(rename = "updateAuthority")]
    pub update_authority: String,
    pub mint: String,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    #[serde(rename = "additionalMetadata")]
    pub additional_metadata: AdditionalMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalMetadata {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenInfo {
    pub symbol: Option<String>,
    pub balance: Option<u64>,
    pub supply: Option<u64>,
    pub decimals: Option<i32>,
    #[serde(rename = "tokenProgram")]
    pub token_program: Option<String>,
    #[serde(rename = "associatedTokenAddress")]
    pub associated_token_address: Option<String>,
    #[serde(rename = "priceInfo")]
    pub price_info: Option<PriceInfo>,
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<String>,
    #[serde(rename = "freezeAuthority")]
    pub freeze_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceInfo {
    #[serde(rename = "pricePerToken")]
    pub price_per_token: f32,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inscription {
    pub order: i32,
    pub size: i32,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub encoding: String,
    #[serde(rename = "validationHash")]
    pub validation_hash: String,
    #[serde(rename = "inscriptionDataAccount")]
    pub inscription_data_account: String,
    pub authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "schema", default)]
    #[serde(alias = "$schema")]
    pub schema: String,
    pub json_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    pub metadata: Metadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub uri: Option<String>,
    pub mime: Option<String>,
    pub cdn_uri: Option<String>,
    pub quality: Option<FileQuality>,
    pub contexts: Option<Vec<Context>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileQuality {
    #[serde(rename = "$$schema")]
    pub schema: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub attributes: Option<Vec<Attribute>>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub value: Value,
    pub trait_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub external_url: Option<String>,
    pub image: Option<String>,
    pub animation_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorities {
    pub address: String,
    pub scopes: Vec<Scope>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub group_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_metadata: Option<CollectionMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionMetadata {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub external_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Compression {
    pub eligible: bool,
    pub compressed: bool,
    pub data_hash: String,
    pub creator_hash: String,
    pub asset_hash: String,
    pub tree: String,
    pub seq: i64,
    pub leaf_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Creator {
    pub address: String,
    pub share: i32,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Royalty {
    pub royalty_model: RoyaltyModel,
    pub target: Option<String>,
    pub percent: f64,
    pub basis_points: u32,
    pub primary_sale_happened: bool,
    pub locked: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ownership {
    pub frozen: bool,
    pub delegated: bool,
    pub delegate: Option<String>,
    pub ownership_model: OwnershipModel,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Supply {
    pub print_max_supply: Option<u64>,
    pub print_current_supply: Option<u64>,
    pub edition_nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_number: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_edition_mint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GroupDefinition {
    pub group_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing)]
    pub asset_id: Vec<u8>,
}

// #[derive(Debug, Serialize, Deserialize, JsonSchema)]
// pub struct SystemInfo {
//     pub created_at: Option<DateTime<Utc>>,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct MplCoreInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_minted: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_size: Option<i32>,
    pub plugins_json_version: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotFilter {
    pub collections: Option<Vec<String>>,
    pub owners: Option<Vec<Vec<u8>>>,
    pub creators: Option<Vec<Vec<u8>>>,
    pub authorities: Option<Vec<Vec<u8>>>,
}

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// #[serde(deny_unknown_fields, rename_all = "camelCase")]
// pub struct CreatedAtFilter {
//     #[serde(default)]
//     pub after: Option<DateTime<Utc>>,
//     #[serde(default)]
//     pub before: Option<DateTime<Utc>>,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAccount {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_extensions: Option<Value>,
    pub frozen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edition {
    pub mint: String,
    pub edition_address: String,
    pub edition: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MintCompressedNftRequest {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub attributes: Vec<Attribute>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "externalUrl", skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(rename = "sellerFeeBasisPoints", skip_serializing_if = "Option::is_none")]
    pub seller_fee_basis_points: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<Creator>>,
    #[serde(rename = "confirmTransaction", skip_serializing_if = "Option::is_none")]
    pub confirm_transaction: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MintResponse {
    pub signature: String,
    pub minted: bool,
    #[serde(rename = "assetId")]
    pub asset_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPriorityFeeEstimateOptions {
    pub priority_level: Option<PriorityLevel>,
    pub include_all_priority_fee_levels: Option<bool>,
    pub transaction_encoding: Option<UiTransactionEncoding>,
    pub lookback_slots: Option<u8>,
    pub recommended: Option<bool>,
    pub include_vote: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetPriorityFeeEstimateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
    #[serde(rename = "accountKeys", skip_serializing_if = "Option::is_none")]
    pub account_keys: Option<Vec<String>>,
    pub options: Option<GetPriorityFeeEstimateOptions>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MicroLamportPriorityFeeLevels {
    pub min: f64,
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    #[serde(rename = "veryHigh")]
    pub very_high: f64,
    #[serde(rename = "unsafeMax")]
    pub unsafe_max: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPriorityFeeEstimateResponse {
    pub priority_fee_estimate: Option<f64>,
    pub priority_fee_levels: Option<MicroLamportPriorityFeeLevels>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRwaAssetRequest {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetRwaAssetResponse {
    pub items: FullRwaAccount,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullRwaAccount {
    pub asset_controller: Option<AssetControllerAccount>,
    pub data_registry: Option<DataRegistryAccount>,
    pub identity_registry: Option<IdentityRegistryAccount>,
    pub policy_engine: Option<PolicyEngine>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetControllerAccount {
    pub address: String,
    pub mint: String,
    pub authority: String,
    pub delegate: String,
    pub version: u32,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataRegistryAccount {
    pub address: String,
    pub mint: String,
    pub version: u32,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityRegistryAccount {
    pub address: String,
    pub mint: String,
    pub authority: String,
    pub delegate: String,
    pub version: u32,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolicyEngine {
    pub address: String,
    pub mint: String,
    pub authority: String,
    pub delegate: String,
    pub policies: Vec<String>,
    pub version: u32,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    #[serde(rename = "webhookID")]
    pub webhook_id: String,
    pub wallet: String,
    pub project: String,
    #[serde(rename = "webhookURL")]
    pub webhook_url: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transaction_types: Vec<TransactionType>,
    pub account_addresses: Vec<String>,
    pub webhook_type: WebhookType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    #[serde(default)]
    pub txn_status: TransactionStatus,
    #[serde(default)]
    pub encoding: AccountWebhookEncoding,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookRequest {
    #[serde(rename = "webhookURL")]
    pub webhook_url: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transaction_types: Vec<TransactionType>,
    pub account_addresses: Vec<String>,
    pub webhook_type: WebhookType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    #[serde(default)]
    pub txn_status: TransactionStatus,
    #[serde(default)]
    pub encoding: AccountWebhookEncoding,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionWebhookRequest {
    pub collection_query: CollectionIdentifier,
    #[serde(rename = "webhookURL")]
    pub webhook_url: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transaction_types: Vec<TransactionType>,
    pub account_addresses: Vec<String>,
    pub webhook_type: WebhookType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    #[serde(default)]
    pub txn_status: TransactionStatus,
    #[serde(default)]
    pub encoding: AccountWebhookEncoding,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EditWebhookRequest {
    #[serde(skip_serializing)]
    pub webhook_id: String,
    #[serde(rename = "webhookURL")]
    pub webhook_url: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transaction_types: Vec<TransactionType>,
    pub account_addresses: Vec<String>,
    pub webhook_type: WebhookType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    #[serde(default)]
    pub txn_status: TransactionStatus,
    #[serde(default)]
    pub encoding: AccountWebhookEncoding,
}

pub struct CreateSmartTransactionConfig<'a> {
    pub instructions: Vec<Instruction>,
    pub signers: Vec<&'a dyn Signer>,
    pub lookup_tables: Option<Vec<AddressLookupTableAccount>>,
    pub fee_payer: Option<&'a dyn Signer>,
}

impl<'a> CreateSmartTransactionConfig<'a> {
    pub fn new(instructions: Vec<Instruction>, signers: Vec<&'a dyn Signer>) -> Self {
        Self {
            instructions,
            signers,
            lookup_tables: None,
            fee_payer: None,
        }
    }
}

pub struct SmartTransactionConfig<'a> {
    pub create_config: CreateSmartTransactionConfig<'a>,
    pub send_options: RpcSendTransactionConfig,
}

impl<'a> SmartTransactionConfig<'a> {
    pub fn new(instructions: Vec<Instruction>, signers: Vec<&'a dyn Signer>) -> Self {
        Self {
            create_config: CreateSmartTransactionConfig::new(instructions, signers),
            send_options: RpcSendTransactionConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BasicRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: Vec<Vec<String>>,
}

#[derive(Clone)]
pub struct CreateSmartTransactionSeedConfig {
    pub instructions: Vec<Instruction>,
    pub signer_seeds: Vec<[u8; 32]>,
    pub fee_payer_seed: Option<[u8; 32]>,
    pub lookup_tables: Option<Vec<AddressLookupTableAccount>>,
}

impl CreateSmartTransactionSeedConfig {
    pub fn new(instructions: Vec<Instruction>, signer_seeds: Vec<[u8; 32]>) -> Self {
        Self {
            instructions,
            signer_seeds,
            fee_payer_seed: None,
            lookup_tables: None,
        }
    }

    pub fn with_fee_payer_seed(mut self, seed: [u8; 32]) -> Self {
        self.fee_payer_seed = Some(seed);
        self
    }

    pub fn with_lookup_tables(mut self, lookup_tables: Vec<AddressLookupTableAccount>) -> Self {
        self.lookup_tables = Some(lookup_tables);
        self
    }
}
