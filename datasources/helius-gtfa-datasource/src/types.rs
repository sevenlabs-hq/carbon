use serde::{Deserialize, Serialize};
use solana_transaction_status::{EncodedTransaction, UiTransactionStatusMeta};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusGtfaRequestConfig {
    pub transaction_details: String,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<String>,
    pub encoding: String,
    pub max_supported_transaction_version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<crate::HeliusGtfaFilters>,
}

#[derive(Debug, Serialize)]
pub struct HeliusGtfaRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGtfaResponse {
    #[serde(default)]
    pub result: Option<HeliusGtfaResult>,
    #[serde(default)]
    pub error: Option<HeliusGtfaError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusGtfaResult {
    pub data: Vec<HeliusGtfaTransaction>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeliusGtfaTransaction {
    pub transaction: EncodedTransaction,
    pub meta: UiTransactionStatusMeta,
    pub slot: u64,
    pub block_time: Option<i64>,
    #[serde(default)]
    pub transaction_index: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGtfaError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct FetchHeliusGtfaTransactionsPageResult {
    pub transactions: Vec<HeliusGtfaTransaction>,
    pub pagination_token: Option<String>,
}
