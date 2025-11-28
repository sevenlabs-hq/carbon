use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct HeliusGpaV2Request {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct HeliusGpaV2Response {
    pub jsonrpc: String,
    pub id: String,
    #[serde(default)]
    pub result: Option<HeliusGpaV2Result>,
    #[serde(default)]
    pub error: Option<HeliusGpaV2Error>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGpaV2Result {
    #[serde(default)]
    pub context: Option<HeliusGpaV2Context>,
    #[serde(default)]
    pub value: Option<HeliusGpaV2Value>,
    #[serde(flatten)]
    pub accounts: Option<Vec<HeliusGpaV2Account>>,
    #[serde(flatten)]
    #[serde(rename = "paginationKey")]
    pub pagination_key_flat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGpaV2Context {
    pub slot: u64,
    #[serde(rename = "apiVersion")]
    #[allow(dead_code)]
    pub api_version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGpaV2Value {
    pub accounts: Vec<HeliusGpaV2Account>,
    #[serde(rename = "paginationKey")]
    pub pagination_key: Option<String>,
    #[allow(dead_code)]
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGpaV2Account {
    pub pubkey: String,
    pub account: HeliusGpaV2AccountData,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGpaV2AccountData {
    pub lamports: u64,
    pub owner: String,
    pub data: Vec<String>,
    pub executable: bool,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    #[allow(dead_code)]
    pub space: u64,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGpaV2Error {
    pub code: i32,
    pub message: String,
}
