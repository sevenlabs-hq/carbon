mod types;

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

use types::{HeliusGpaV2Request, HeliusGpaV2Response, HeliusGpaV2Value};

const DEFAULT_LIMIT: u32 = 1000;
const MAX_LIMIT: u32 = 10000;

#[derive(Debug, Clone)]
pub struct HeliusGpaV2Config {
    pub program_accounts_config: Option<RpcProgramAccountsConfig>,
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
            program_accounts_config,
            changed_since_slot,
            limit: limit.or(Some(DEFAULT_LIMIT)),
        }
    }
}

impl Default for HeliusGpaV2Config {
    fn default() -> Self {
        Self {
            program_accounts_config: None,
            changed_since_slot: None,
            limit: Some(DEFAULT_LIMIT),
        }
    }
}

pub struct HeliusGpaV2Datasource {
    pub endpoint: String,
    pub api_key: String,
    pub program_id: Pubkey,
    pub config: HeliusGpaV2Config,
}

impl HeliusGpaV2Datasource {
    pub fn new(
        endpoint: String,
        api_key: String,
        program_id: Pubkey,
        config: HeliusGpaV2Config,
    ) -> Self {
        Self {
            endpoint,
            api_key,
            program_id,
            config,
        }
    }

    pub fn new_with_defaults(endpoint: String, api_key: String, program_id: Pubkey) -> Self {
        Self::new(endpoint, api_key, program_id, HeliusGpaV2Config::default())
    }

    async fn fetch_accounts_page(
        &self,
        client: &reqwest::Client,
        pagination_key: Option<&str>,
    ) -> CarbonResult<(HeliusGpaV2Value, Option<u64>)> {
        let mut params = vec![serde_json::Value::String(self.program_id.to_string())];

        let mut config = serde_json::Map::new();

        let encoding = self
            .config
            .program_accounts_config
            .as_ref()
            .and_then(|c| c.account_config.encoding.as_ref())
            .map(|e| match e {
                solana_account_decoder::UiAccountEncoding::Base64 => "base64",
                solana_account_decoder::UiAccountEncoding::Base58 => "base58",
                solana_account_decoder::UiAccountEncoding::JsonParsed => "jsonParsed",
                solana_account_decoder::UiAccountEncoding::Binary => "base64", // Binary not supported by Helius, fallback to base64
                solana_account_decoder::UiAccountEncoding::Base64Zstd => "base64", // Base64Zstd not supported by Helius, fallback to base64
            })
            .unwrap_or("base64");
        config.insert(
            "encoding".to_string(),
            serde_json::Value::String(encoding.to_string()),
        );

        let limit = self.config.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
        config.insert("limit".to_string(), serde_json::Value::Number(limit.into()));

        if let Some(changed_since_slot) = self.config.changed_since_slot {
            config.insert(
                "changedSinceSlot".to_string(),
                serde_json::Value::Number(changed_since_slot.into()),
            );
        }

        if let Some(commitment) = self
            .config
            .program_accounts_config
            .as_ref()
            .and_then(|c| c.account_config.commitment.as_ref())
        {
            let commitment_str = if commitment.is_finalized() {
                "finalized"
            } else if commitment.is_confirmed() {
                "confirmed"
            } else if commitment.is_processed() {
                "processed"
            } else {
                "confirmed"
            };
            config.insert(
                "commitment".to_string(),
                serde_json::Value::String(commitment_str.to_string()),
            );
        }

        if let Some(pagination_key) = pagination_key {
            config.insert(
                "paginationKey".to_string(),
                serde_json::Value::String(pagination_key.to_string()),
            );
        }

        if let Some(program_config) = &self.config.program_accounts_config {
            if let Some(filters) = &program_config.filters {
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
        }

        config.insert("withContext".to_string(), serde_json::Value::Bool(true));

        params.push(serde_json::Value::Object(config));

        let request = HeliusGpaV2Request {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getProgramAccountsV2".to_string(),
            params,
        };

        let separator = if self.endpoint.contains('?') {
            "&"
        } else {
            "?"
        };
        let url = format!("{}{}api-key={}", self.endpoint, separator, self.api_key);

        let response = client.post(&url).json(&request).send().await.map_err(|e| {
            carbon_core::error::Error::FailedToConsumeDatasource(format!(
                "HTTP request failed: {e}"
            ))
        })?;

        let status = response.status();
        if !status.is_success() {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                format!("HTTP error: {status}"),
            ));
        }

        let response_body: HeliusGpaV2Response = response.json().await.map_err(|e| {
            carbon_core::error::Error::FailedToConsumeDatasource(format!(
                "Failed to parse response: {e}"
            ))
        })?;

        if let Some(error) = response_body.error {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                format!("RPC error: {} (code: {})", error.message, error.code),
            ));
        }

        let result = response_body.result.ok_or_else(|| {
            carbon_core::error::Error::FailedToConsumeDatasource(
                "Response missing result field".to_string(),
            )
        })?;

        let value = if let Some(v) = result.value {
            v
        } else if let Some(accounts) = result.accounts {
            HeliusGpaV2Value {
                accounts,
                pagination_key: result.pagination_key_flat,
                count: None,
            }
        } else {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Response missing accounts data".to_string(),
            ));
        };

        let slot = result.context.map(|ctx| ctx.slot);

        Ok((value, slot))
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
        let mut accounts_sent = 0u64;
        let mut pages_fetched = 0u64;
        let mut pagination_key: Option<String> = None;

        loop {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested during account processing");
                break;
            }

            let fetch_start = std::time::Instant::now();
            let (result, slot) = self
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

            pages_fetched += 1;
            metrics
                .increment_counter("helius_gpa_v2_pages_fetched", 1)
                .await
                .unwrap_or_else(|e| log::error!("Error recording counter: {e}"));
            log::debug!(
                "Fetched page {} with {} accounts (slot: {:?})",
                pages_fetched,
                result.accounts.len(),
                slot
            );

            for account_data in result.accounts {
                if cancellation_token.is_cancelled() {
                    log::info!("Cancellation requested during account processing");
                    break;
                }

                let pubkey = match Pubkey::try_from(account_data.pubkey.as_str()) {
                    Ok(pk) => pk,
                    Err(e) => {
                        log::warn!("Failed to parse pubkey {}: {e}", account_data.pubkey);
                        continue;
                    }
                };

                let decoded_account = if account_data.account.data.len() >= 2 {
                    let data_str = &account_data.account.data[0];

                    let data_bytes = match STANDARD.decode(data_str) {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            log::warn!("Failed to decode base64 data for account {pubkey}: {e}");
                            continue;
                        }
                    };

                    Account {
                        lamports: account_data.account.lamports,
                        data: data_bytes,
                        owner: match Pubkey::try_from(account_data.account.owner.as_str()) {
                            Ok(owner) => owner,
                            Err(e) => {
                                log::warn!("Failed to parse owner for account {pubkey}: {e}");
                                continue;
                            }
                        },
                        executable: account_data.account.executable,
                        rent_epoch: account_data.account.rent_epoch,
                    }
                } else {
                    log::warn!(
                        "Account {pubkey} has invalid data format (expected [data, encoding])"
                    );
                    continue;
                };

                let update = Update::Account(AccountUpdate {
                    pubkey,
                    account: decoded_account,
                    slot: slot.unwrap_or(0),
                    transaction_signature: None,
                });

                match sender.try_send((update, id_for_loop.clone())) {
                    Ok(_) => {
                        accounts_sent += 1;

                        metrics
                            .increment_counter("helius_gpa_v2_accounts_processed", 1)
                            .await
                            .unwrap_or_else(|e| log::error!("Error recording counter: {e}"));
                    }
                    Err(e) => {
                        log::error!("Failed to send account update: {e:?}");
                    }
                }
            }

            pagination_key = result.pagination_key;

            if pagination_key.is_none() {
                log::info!("Reached end of pagination");
                break;
            }
        }

        log::info!(
            "Helius gPA V2 account indexing completed: {accounts_sent} accounts sent, {pages_fetched} pages fetched"
        );

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
