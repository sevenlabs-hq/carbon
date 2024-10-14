#![allow(unused_imports)]
/// Jito Smart Transactions
///
/// This module allows the creation and sending of smart transactions with Jito tips.
/// It includes methods to add tips to transactions, create smart transactions with tips, and
/// send these smart transactions as bundles. Additionally, it provides the ability to check
/// the status of sent bundles  
use crate::error::{HeliusError, Result};
use crate::types::{
    BasicRequest, CreateSmartTransactionConfig, GetPriorityFeeEstimateOptions, GetPriorityFeeEstimateRequest,
    GetPriorityFeeEstimateResponse, SmartTransaction, SmartTransactionConfig,
};
use crate::Helius;

use bincode::{serialize, ErrorKind};
use chrono::format::parse;
use phf::phf_map;
use rand::seq::SliceRandom;
use reqwest::{Method, StatusCode, Url};
use serde::Serialize;
use serde_json::Value;
use solana_client::rpc_config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig};
use solana_client::rpc_response::{Response, RpcSimulateTransactionResult};
use solana_sdk::system_instruction;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    bs58::encode,
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    instruction::Instruction,
    message::{v0, VersionedMessage},
    pubkey::Pubkey,
    signature::{Signature, Signer},
    transaction::{Transaction, VersionedTransaction},
};
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout_at};

/// Jito tip accounts
pub const JITO_TIP_ACCOUNTS: [&str; 8] = [
    "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5",
    "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe",
    "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY",
    "ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49",
    "DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh",
    "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt",
    "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",
    "3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT",
];

/// Jito API URLs for different regions
pub static JITO_API_URLS: phf::Map<&'static str, &'static str> = phf_map! {
    "Default" => "https://mainnet.block-engine.jito.wtf",
    "NY" => "https://ny.mainnet.block-engine.jito.wtf",
    "Amsterdam" => "https://amsterdam.mainnet.block-engine.jito.wtf",
    "Frankfurt" => "https://frankfurt.mainnet.block-engine.jito.wtf",
    "Tokyo" => "https://tokyo.mainnet.block-engine.jito.wtf",
};

/// Type alias for Jito regions
pub type JitoRegion = &'static str;

impl Helius {
    /// Adds a tip instruction to the provided instructions
    ///
    /// # Arguments
    /// * `instructions` - The transaction instructions to which the tip_instruction will be added
    /// * `fee_payer` - The public key of the fee payer
    /// * `tip_account` - The public key of the tip account as a string
    /// * `tip_amount` - The amount of lamports to tip
    pub fn add_tip_instruction(
        &self,
        instructions: &mut Vec<Instruction>,
        fee_payer: Pubkey,
        tip_account: &str,
        tip_amount: u64,
    ) {
        let tip_instruction: Instruction =
            system_instruction::transfer(&fee_payer, &Pubkey::from_str(tip_account).unwrap(), tip_amount);
        instructions.push(tip_instruction);
    }

    /// Creates a smart transaction with a Jito tip
    ///
    /// # Arguments
    /// * `config` - The configuration for creating the smart transaction
    /// * `tip_amount` - The amount of lamports to tip. Defaults to `1000`
    ///
    /// # Returns
    /// A `Result` containing the serialized transaction as a base58-encoded string and the last valid block height
    pub async fn create_smart_transaction_with_tip(
        &self,
        mut config: CreateSmartTransactionConfig<'_>,
        tip_amount: Option<u64>,
    ) -> Result<(String, u64)> {
        if config.signers.is_empty() {
            return Err(HeliusError::InvalidInput(
                "The transaction must have at least one signer".to_string(),
            ));
        }

        let tip_amount: u64 = tip_amount.unwrap_or(1000);
        let random_tip_account: &str = *JITO_TIP_ACCOUNTS.choose(&mut rand::thread_rng()).unwrap();
        let payer_key: Pubkey = config
            .fee_payer
            .map_or_else(|| config.signers[0].pubkey(), |signer| signer.pubkey());

        self.add_tip_instruction(&mut config.instructions, payer_key, random_tip_account, tip_amount);

        let (smart_transaction, last_valid_block_height) = self.create_smart_transaction(&config).await?;
        let serialized_transaction: Vec<u8> = match smart_transaction {
            SmartTransaction::Legacy(tx) => {
                serialize(&tx).map_err(|e: Box<ErrorKind>| HeliusError::InvalidInput(e.to_string()))?
            }
            SmartTransaction::Versioned(tx) => {
                serialize(&tx).map_err(|e: Box<ErrorKind>| HeliusError::InvalidInput(e.to_string()))?
            }
        };
        let transaction_base58: String = encode(&serialized_transaction).into_string();

        Ok((transaction_base58, last_valid_block_height))
    }

    /// Sends a bundle of transactions to the Jito Block Engine
    ///
    /// # Arguments
    /// * `serialized_transactions` - The serialized transactions in the bundle
    /// * `jito_api_url` - The Jito Block Engine API URL
    ///
    /// # Returns
    /// A `Result` containing the bundle ID
    pub async fn send_jito_bundle(&self, serialized_transactions: Vec<String>, jito_api_url: &str) -> Result<String> {
        let request: BasicRequest = BasicRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "sendBundle".to_string(),
            params: vec![serialized_transactions],
        };

        let parsed_url: Url = Url::parse(jito_api_url).expect("Failed to parse URL");

        let response: Value = self
            .rpc_client
            .handler
            .send(Method::POST, parsed_url, Some(&request))
            .await?;

        if let Some(error) = response.get("error") {
            return Err(HeliusError::BadRequest {
                path: jito_api_url.to_string(),
                text: format!("Error sending bundles: {:?}", error),
            });
        }

        if let Some(result) = response.get("result") {
            if let Some(bundle_id) = result.as_str() {
                return Ok(bundle_id.to_string());
            }
        }

        Err(HeliusError::Unknown {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            text: "Unexpected response format".to_string(),
        })
    }

    /// Get the status of Jito bundles
    ///
    /// # Arguments
    /// * `bundle_ids` - An array of bundle IDs to check the status for
    /// * `jito_api_url` - The Jito Block Engine API URL
    ///
    /// # Returns
    /// A `Result` containing the status of the bundles as a `serde_json::Value`
    pub async fn get_bundle_statuses(&self, bundle_ids: Vec<String>, jito_api_url: &str) -> Result<Value> {
        let request: BasicRequest = BasicRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getBundleStatuses".to_string(),
            params: vec![bundle_ids],
        };

        let parsed_url: Url = Url::parse(jito_api_url).expect("Failed to parse URL");

        let response: Value = self
            .rpc_client
            .handler
            .send(Method::POST, parsed_url, Some(&request))
            .await?;

        if let Some(error) = response.get("error") {
            return Err(HeliusError::BadRequest {
                path: jito_api_url.to_string(),
                text: format!("Error getting bundle statuses: {:?}", error),
            });
        }

        // Return the response value
        Ok(response)
    }

    /// Sends a smart transaction as a Jito bundle with a tip
    ///
    /// # Arguments
    /// * `config` - The configuration for sending the smart transaction
    /// * `tip_amount` - The amount of lamports tp tip. Defaults to `1000`
    /// * `region` - The Jito Block Engine region. Defaults to `"Default"`
    ///
    /// # Returns
    /// A `Result` containing the bundle IDc
    pub async fn send_smart_transaction_with_tip(
        &self,
        config: SmartTransactionConfig<'_>,
        tip_amount: Option<u64>,
        region: Option<JitoRegion>,
    ) -> Result<String> {
        if config.create_config.signers.is_empty() {
            return Err(HeliusError::InvalidInput(
                "The transaction must have at least one signer".to_string(),
            ));
        }

        let tip: u64 = tip_amount.unwrap_or(1000);
        let user_provided_region: &str = region.unwrap_or("Default");
        let jito_region: &str = *JITO_API_URLS
            .get(user_provided_region)
            .ok_or_else(|| HeliusError::InvalidInput("Invalid Jito region".to_string()))?;
        let jito_api_url_string: String = format!("{}/api/v1/bundles", jito_region);
        let jito_api_url: &str = jito_api_url_string.as_str();

        // Create the smart transaction with tip
        let (serialized_transaction, last_valid_block_height) = self
            .create_smart_transaction_with_tip(config.create_config, Some(tip))
            .await?;

        // Send the transaction as a Jito bundle
        let bundle_id: String = self
            .send_jito_bundle(vec![serialized_transaction], jito_api_url)
            .await?;

        // Poll for confirmation status
        let timeout: Duration = Duration::from_secs(60);
        let interval: Duration = Duration::from_secs(5);
        let start: tokio::time::Instant = tokio::time::Instant::now();

        while start.elapsed() < timeout || self.connection().get_block_height()? <= last_valid_block_height {
            let bundle_statuses: Value = self.get_bundle_statuses(vec![bundle_id.clone()], jito_api_url).await?;

            if let Some(values) = bundle_statuses["result"]["value"].as_array() {
                if !values.is_empty() {
                    if let Some(status) = values[0]["confirmation_status"].as_str() {
                        if status == "confirmed" {
                            return Ok(values[0]["transactions"][0].as_str().unwrap().to_string());
                        }
                    }
                }
            }

            sleep(interval).await;
        }

        Err(HeliusError::Timeout {
            code: StatusCode::REQUEST_TIMEOUT,
            text: "Bundle failed to confirm within the timeout period".to_string(),
        })
    }
}
