use serde::{Deserialize, Serialize};
use solana_transaction_status::{EncodedTransaction, UiTransactionStatusMeta};

#[derive(Debug, Serialize)]
pub struct HeliusGtfaRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct HeliusGtfaResponse {
    pub jsonrpc: String,
    pub id: String,
    #[serde(default)]
    pub result: Option<HeliusGtfaResult>,
    #[serde(default)]
    pub error: Option<HeliusGtfaError>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGtfaResult {
    pub data: Vec<HeliusGtfaTransaction>,
    #[serde(rename = "paginationToken")]
    pub pagination_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HeliusGtfaTransaction {
    pub transaction: EncodedTransaction,
    pub meta: UiTransactionStatusMeta,
    pub slot: u64,
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,
    #[serde(rename = "idxInBlock")]
    #[serde(default)]
    #[allow(dead_code)]
    pub idx_in_block: Option<u64>,
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
