use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::EncodedTransactionWithStatusMeta;

#[derive(Debug, Clone, PartialEq, Default, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSubscribeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_exclude: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_required: Option<Vec<String>>,
}

impl TransactionSubscribeFilter {
    pub fn standard(key: &Pubkey) -> Self {
        Self {
            account_include: Some(vec![key.to_string()]),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UiEnhancedTransactionEncoding {
    Base58,
    Base64,
    #[serde(rename = "base64+zstd")]
    Base64Zstd,
    JsonParsed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionCommitment {
    Processed,
    Confirmed,
    Finalized,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionDetails {
    Full,
    Signatures,
    Accounts,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSubscribeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<TransactionCommitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<UiEnhancedTransactionEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_details: Option<TransactionDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_rewards: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
}

impl Default for TransactionSubscribeOptions {
    fn default() -> Self {
        Self {
            commitment: Some(TransactionCommitment::Confirmed),
            encoding: Some(UiEnhancedTransactionEncoding::JsonParsed),
            transaction_details: Some(TransactionDetails::Full),
            show_rewards: Some(true),
            max_supported_transaction_version: Some(0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionsConfig {
    pub filter: TransactionSubscribeFilter,
    pub options: TransactionSubscribeOptions,
}

// Websocket transaction response

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionNotification {
    pub transaction: EncodedTransactionWithStatusMeta,
    pub signature: String,
}
