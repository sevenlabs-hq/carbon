mod types;

use std::str::FromStr;

use crate::types::HeliusGpaV2Result;

use {
    async_trait::async_trait,
    base64::{engine::general_purpose::STANDARD, Engine},
    carbon_core::{
        datasource::{AccountUpdate, Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
        metrics::MetricsCollection,
    },
    solana_account::Account,
    solana_client::rpc_config::RpcProgramAccountsConfig,
    solana_pubkey::Pubkey,
    std::sync::Arc,
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

use types::{FetchHeliusGpaV2AccountsPageResult, HeliusGpaV2Request, HeliusGpaV2Response};

const DEFAULT_LIMIT: u32 = 1000;
const MAX_LIMIT: u32 = 10000;

#[derive(Debug, Clone)]
pub struct HeliusGpaV2Config {
    pub program_accounts_config: RpcProgramAccountsConfig,
    pub changed_since_slot: Option<u64>,
    pub limit: Option<u32>,
}

impl HeliusGpaV2Config {
    pub fn new(
        program_accounts_config: Option<RpcProgramAccountsConfig>,
        changed_since_slot: Option<u64>,
        limit: Option<u32>,
    ) -> Self {
        Self {
            program_accounts_config: program_accounts_config.unwrap_or_default(),
            changed_since_slot,
            limit: limit.or(Some(DEFAULT_LIMIT)),
        }
    }
}

impl Default for HeliusGpaV2Config {
    fn default() -> Self {
        Self {
            program_accounts_config: RpcProgramAccountsConfig::default(),
            changed_since_slot: None,
            limit: Some(DEFAULT_LIMIT),
        }
    }
}

pub struct HeliusGpaV2Datasource {
    pub helius_rpc_url: String,
    pub program_id: Pubkey,
    pub config: HeliusGpaV2Config,
}

impl HeliusGpaV2Datasource {
    pub fn new(helius_rpc_url: String, program_id: Pubkey) -> Self {
        Self::new_with_config(helius_rpc_url, program_id, HeliusGpaV2Config::default())
    }

    pub fn new_with_config(
        helius_rpc_url: String,
        program_id: Pubkey,
        config: HeliusGpaV2Config,
    ) -> Self {
        Self {
            helius_rpc_url,
            program_id,
            config,
        }
    }

    async fn fetch_accounts_page(
        &self,
        client: &reqwest::Client,
        pagination_key: Option<&str>,
    ) -> CarbonResult<FetchHeliusGpaV2AccountsPageResult> {
        let mut params = vec![serde_json::Value::String(self.program_id.to_string())];
        let mut config = serde_json::Map::new();

        let limit = self.config.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);

        config.insert(
            "encoding".to_string(),
            serde_json::Value::String("base64".to_string()),
        );
        config.insert("limit".to_string(), serde_json::Value::Number(limit.into()));
        config.insert("withContext".to_string(), serde_json::Value::Bool(true));

        if let Some(changed_since_slot) = self.config.changed_since_slot {
            config.insert(
                "changedSinceSlot".to_string(),
                serde_json::Value::Number(changed_since_slot.into()),
            );
        }
        if let Some(commitment) = self
            .config
            .program_accounts_config
            .account_config
            .commitment
        {
            config.insert(
                "commitment".to_string(),
                serde_json::Value::String(commitment.commitment.to_string()),
            );
        }
        if let Some(pagination_key) = pagination_key {
            config.insert(
                "paginationKey".to_string(),
                serde_json::Value::String(pagination_key.to_string()),
            );
        }
        if let Some(filters) = &self.config.program_accounts_config.filters {
            let filter_array: Vec<serde_json::Value> = filters
                .iter()
                .map(|f| {
                    serde_json::to_value(f).unwrap_or_else(|_| {
                        log::warn!("Failed to serialize filter, using null");
                        serde_json::Value::Null
                    })
                })
                .collect();
            config.insert(
                "filters".to_string(),
                serde_json::Value::Array(filter_array),
            );
        }

        params.push(serde_json::Value::Object(config));

        let request = HeliusGpaV2Request {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getProgramAccountsV2".to_string(),
            params,
        };

        let response = client
            .post(&self.helius_rpc_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                carbon_core::error::Error::FailedToConsumeDatasource(format!(
                    "HTTP request failed: {e}"
                ))
            })?
            .json::<HeliusGpaV2Response>()
            .await
            .map_err(|e| {
                carbon_core::error::Error::FailedToConsumeDatasource(format!(
                    "Failed to parse response: {e}"
                ))
            })?;

        if let Some(error) = response.error {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                format!("RPC error: {} (code: {})", error.message, error.code),
            ));
        }

        let Some(HeliusGpaV2Result {
            context: Some(context),
            accounts: Some(accounts),
            pagination_key_flat,
            ..
        }) = response.result
        else {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Response missing context or accounts field".to_string(),
            ));
        };

        let accounts = accounts
            .into_iter()
            .filter_map(|account| {
                let pubkey = Pubkey::from_str(&account.pubkey).ok()?;
                let account_data = account
                    .account
                    .data
                    .get(1)
                    .and_then(|data| STANDARD.decode(data).ok())?;
                let account = Account {
                    lamports: account.account.lamports,
                    data: account_data,
                    owner: Pubkey::from_str(&account.account.owner).ok()?,
                    executable: account.account.executable,
                    rent_epoch: account.account.rent_epoch,
                };
                Some((pubkey, account))
            })
            .collect();

        Ok(FetchHeliusGpaV2AccountsPageResult {
            accounts,
            slot: context.slot,
            pagination_key: pagination_key_flat,
        })
    }
}

#[async_trait]
impl Datasource for HeliusGpaV2Datasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let client = reqwest::Client::new();

        log::info!(
            "Starting Helius gPA V2 account indexing for program {}",
            self.program_id
        );

        let id_for_loop = id.clone();
        let mut pagination_key: Option<String> = None;

        loop {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested during account processing");
                break;
            }

            let fetch_start = std::time::Instant::now();
            let result = self
                .fetch_accounts_page(&client, pagination_key.as_deref())
                .await?;
            let fetch_elapsed = fetch_start.elapsed();

            metrics
                .record_histogram(
                    "helius_gpa_v2_fetch_duration_seconds",
                    fetch_elapsed.as_secs_f64(),
                )
                .await
                .unwrap_or_else(|e| log::error!("Error recording histogram: {e}"));

            metrics
                .increment_counter("helius_gpa_v2_pages_fetched", 1)
                .await
                .unwrap_or_else(|e| log::error!("Error recording counter: {e}"));

            log::debug!(
                "Fetched page with {} accounts (slot: {:?})",
                result.accounts.len(),
                result.slot
            );

            for (pubkey, account) in result.accounts {
                if cancellation_token.is_cancelled() {
                    log::info!("Cancellation requested during account processing");
                    break;
                }

                let update = Update::Account(AccountUpdate {
                    pubkey,
                    account,
                    slot: result.slot,
                    transaction_signature: None,
                });

                if let Err(e) = sender.send((update, id_for_loop.clone())).await {
                    log::error!("Failed to send account update: {e:?}");
                };

                metrics
                    .increment_counter("helius_gpa_v2_accounts_processed", 1)
                    .await
                    .unwrap_or_else(|e| log::error!("Error recording counter: {e}"));
            }

            pagination_key = result.pagination_key;

            if pagination_key.is_none() {
                log::info!("Reached end of pagination");
                break;
            }
        }

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
