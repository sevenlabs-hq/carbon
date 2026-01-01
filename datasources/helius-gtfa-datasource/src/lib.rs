mod types;

use crate::types::{FetchHeliusGtfaTransactionsPageResult, HeliusGtfaRequest, HeliusGtfaResponse};
use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        metrics::MetricsCollection
    },
    solana_client::rpc_client::SerializableTransaction,
    solana_commitment_config::CommitmentConfig,
    solana_pubkey::Pubkey,
    std::sync::Arc,
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

const DEFAULT_LIMIT: u32 = 100;
const MAX_LIMIT: u32 = 100;

#[derive(Debug, Clone)]
pub struct HeliusGtfaFilters {
    pub slot: Option<SlotFilter>,
    pub block_time: Option<BlockTimeFilter>,
    pub signature: Option<SignatureFilter>,
    pub status: Option<TransactionStatusFilter>,
}

#[derive(Debug, Clone)]
pub struct SlotFilter {
    pub gte: Option<u64>,
    pub gt: Option<u64>,
    pub lte: Option<u64>,
    pub lt: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct BlockTimeFilter {
    pub gte: Option<i64>,
    pub gt: Option<i64>,
    pub lte: Option<i64>,
    pub lt: Option<i64>,
    pub eq: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct SignatureFilter {
    pub gte: Option<String>,
    pub gt: Option<String>,
    pub lte: Option<String>,
    pub lt: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TransactionStatusFilter {
    Succeeded,
    Failed,
    Any,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct HeliusGtfaConfig {
    pub sort_order: Option<SortOrder>,
    pub limit: Option<u32>,
    pub commitment: Option<CommitmentConfig>,
    pub filters: Option<HeliusGtfaFilters>,
    pub min_context_slot: Option<u64>,
}

impl HeliusGtfaConfig {
    pub fn new(
        sort_order: Option<SortOrder>,
        limit: Option<u32>,
        commitment: Option<CommitmentConfig>,
        filters: Option<HeliusGtfaFilters>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self {
            sort_order,
            limit: limit.or(Some(DEFAULT_LIMIT)),
            commitment,
            filters,
            min_context_slot,
        }
    }
}

impl Default for HeliusGtfaConfig {
    fn default() -> Self {
        Self {
            sort_order: Some(SortOrder::Asc),
            limit: Some(DEFAULT_LIMIT),
            commitment: Some(CommitmentConfig::confirmed()),
            filters: None,
            min_context_slot: None,
        }
    }
}

pub struct HeliusGtfaDatasource {
    pub helius_rpc_url: String,
    pub address: Pubkey,
    pub config: HeliusGtfaConfig,
}

impl HeliusGtfaDatasource {
    pub fn new(helius_rpc_url: String, address: Pubkey) -> Self {
        Self::new_with_config(helius_rpc_url, address, HeliusGtfaConfig::default())
    }

    pub fn new_with_config(
        helius_rpc_url: String,
        address: Pubkey,
        config: HeliusGtfaConfig,
    ) -> Self {
        Self {
            helius_rpc_url,
            address,
            config,
        }
    }

    async fn fetch_transactions_page(
        &self,
        client: &reqwest::Client,
        pagination_token: Option<&str>,
    ) -> CarbonResult<FetchHeliusGtfaTransactionsPageResult> {
        let mut params = vec![serde_json::Value::String(self.address.to_string())];
        let mut config = serde_json::Map::new();

        config.insert(
            "transactionDetails".to_string(),
            serde_json::Value::String("full".to_string()),
        );

        let limit = self.config.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
        config.insert("limit".to_string(), serde_json::Value::Number(limit.into()));

        if let Some(sort_order) = &self.config.sort_order {
            let order_str = match sort_order {
                SortOrder::Asc => "asc",
                SortOrder::Desc => "desc",
            };
            config.insert(
                "sortOrder".to_string(),
                serde_json::Value::String(order_str.to_string()),
            );
        }

        if let Some(pagination_token) = pagination_token {
            config.insert(
                "paginationToken".to_string(),
                serde_json::Value::String(pagination_token.to_string()),
            );
        }

        if let Some(commitment) = &self.config.commitment {
            config.insert(
                "commitment".to_string(),
                serde_json::Value::String(commitment.commitment.to_string()),
            );
        }

        config.insert(
            "encoding".to_string(),
            serde_json::Value::String("base64".to_string()),
        );

        config.insert(
            "maxSupportedTransactionVersion".to_string(),
            serde_json::Value::Number(0.into()),
        );

        if let Some(min_context_slot) = self.config.min_context_slot {
            config.insert(
                "minContextSlot".to_string(),
                serde_json::Value::Number(min_context_slot.into()),
            );
        }

        if let Some(filters) = &self.config.filters {
            let mut filter_obj = serde_json::Map::new();

            if let Some(slot_filter) = &filters.slot {
                let mut slot_obj = serde_json::Map::new();
                if let Some(gte) = slot_filter.gte {
                    slot_obj.insert("gte".to_string(), serde_json::Value::Number(gte.into()));
                }
                if let Some(gt) = slot_filter.gt {
                    slot_obj.insert("gt".to_string(), serde_json::Value::Number(gt.into()));
                }
                if let Some(lte) = slot_filter.lte {
                    slot_obj.insert("lte".to_string(), serde_json::Value::Number(lte.into()));
                }
                if let Some(lt) = slot_filter.lt {
                    slot_obj.insert("lt".to_string(), serde_json::Value::Number(lt.into()));
                }
                if !slot_obj.is_empty() {
                    filter_obj.insert("slot".to_string(), serde_json::Value::Object(slot_obj));
                }
            }

            if let Some(block_time_filter) = &filters.block_time {
                let mut block_time_obj = serde_json::Map::new();
                if let Some(gte) = block_time_filter.gte {
                    block_time_obj.insert("gte".to_string(), serde_json::Value::Number(gte.into()));
                }
                if let Some(gt) = block_time_filter.gt {
                    block_time_obj.insert("gt".to_string(), serde_json::Value::Number(gt.into()));
                }
                if let Some(lte) = block_time_filter.lte {
                    block_time_obj.insert("lte".to_string(), serde_json::Value::Number(lte.into()));
                }
                if let Some(lt) = block_time_filter.lt {
                    block_time_obj.insert("lt".to_string(), serde_json::Value::Number(lt.into()));
                }
                if let Some(eq) = block_time_filter.eq {
                    block_time_obj.insert("eq".to_string(), serde_json::Value::Number(eq.into()));
                }
                if !block_time_obj.is_empty() {
                    filter_obj.insert(
                        "blockTime".to_string(),
                        serde_json::Value::Object(block_time_obj),
                    );
                }
            }

            if let Some(signature_filter) = &filters.signature {
                let mut signature_obj = serde_json::Map::new();
                if let Some(gte) = &signature_filter.gte {
                    signature_obj.insert("gte".to_string(), serde_json::Value::String(gte.clone()));
                }
                if let Some(gt) = &signature_filter.gt {
                    signature_obj.insert("gt".to_string(), serde_json::Value::String(gt.clone()));
                }
                if let Some(lte) = &signature_filter.lte {
                    signature_obj.insert("lte".to_string(), serde_json::Value::String(lte.clone()));
                }
                if let Some(lt) = &signature_filter.lt {
                    signature_obj.insert("lt".to_string(), serde_json::Value::String(lt.clone()));
                }
                if !signature_obj.is_empty() {
                    filter_obj.insert(
                        "signature".to_string(),
                        serde_json::Value::Object(signature_obj),
                    );
                }
            }

            if let Some(status) = &filters.status {
                let status_str = match status {
                    TransactionStatusFilter::Succeeded => "succeeded",
                    TransactionStatusFilter::Failed => "failed",
                    TransactionStatusFilter::Any => "any",
                };
                filter_obj.insert(
                    "status".to_string(),
                    serde_json::Value::String(status_str.to_string()),
                );
            }

            if !filter_obj.is_empty() {
                config.insert("filters".to_string(), serde_json::Value::Object(filter_obj));
            }
        }

        params.push(serde_json::Value::Object(config));

        let request = HeliusGtfaRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getTransactionsForAddress".to_string(),
            params,
        };

        let http_response = client
            .post(&self.helius_rpc_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                carbon_core::error::Error::FailedToConsumeDatasource(format!(
                    "HTTP request failed: {e}"
                ))
            })?;

        let status = http_response.status();
        let response_text = http_response.text().await.map_err(|e| {
            carbon_core::error::Error::FailedToConsumeDatasource(format!(
                "Failed to read response body: {e}"
            ))
        })?;

        if !status.is_success() {
            log::error!("RPC request failed with status {status}: {response_text}");
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                format!("HTTP error {status}: {response_text}"),
            ));
        }

        let response: HeliusGtfaResponse = serde_json::from_str(&response_text).map_err(|e| {
            log::error!("Failed to parse JSON response. Response body: {response_text}");
            carbon_core::error::Error::FailedToConsumeDatasource(format!(
                "Failed to parse response: {e}"
            ))
        })?;

        if let Some(error) = response.error {
            log::error!("RPC error response: {error:?}");
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                format!("RPC error: {} (code: {})", error.message, error.code),
            ));
        }

        let Some(result) = response.result else {
            log::error!("Response missing result field. Full response: {response:?}");
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Response missing result field".to_string(),
            ));
        };

        Ok(FetchHeliusGtfaTransactionsPageResult {
            transactions: result.data,
            pagination_token: result.pagination_token,
        })
    }
}

#[async_trait]
impl Datasource for HeliusGtfaDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let client = reqwest::Client::new();

        log::info!(
            "Starting Helius GTFA transaction indexing for address {}",
            self.address
        );

        let id_for_loop = id.clone();
        let mut pagination_token: Option<String> = None;

        loop {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested during transaction processing");
                break;
            }

            let fetch_start = std::time::Instant::now();
            let result = self
                .fetch_transactions_page(&client, pagination_token.as_deref())
                .await?;
            let fetch_elapsed = fetch_start.elapsed();

            metrics
                .record_histogram(
                    "helius_gtfa_fetch_duration_seconds",
                    fetch_elapsed.as_secs_f64(),
                )
                .await?;

            metrics
                .increment_counter("helius_gtfa_pages_fetched", 1)
                .await?;

            let status_filter = self.config.filters.clone().map(|filters| filters.status).flatten();

            for tx in result.transactions {
                if cancellation_token.is_cancelled() {
                    log::info!("Cancellation requested during transaction processing");
                    break;
                }

                match (&tx.meta.err, &status_filter) {
                    (Some(_), Some(TransactionStatusFilter::Succeeded) | None) => {
                        continue;
                    }
                    (None, Some(TransactionStatusFilter::Failed)) => {
                        continue;
                    }
                    _ => {}
                };

                let Some(decoded_transaction) = tx.transaction.decode() else {
                    log::warn!("Failed to decode transaction");
                    continue;
                };

                let update = Update::Transaction(Box::new(TransactionUpdate {
                    signature: *decoded_transaction.get_signature(),
                    transaction: decoded_transaction,
                    meta: carbon_core::transformers::transaction_metadata_from_original_meta(tx.meta.clone())?,
                    is_vote: false,
                    slot: tx.slot,
                    index: tx.transaction_index,
                    block_time: tx.block_time,
                    block_hash: None,
                }));

                if let Err(e) = sender.send((update, id_for_loop.clone())).await {
                    log::error!("Failed to send transaction update: {e:?}");
                };

                metrics
                    .increment_counter("helius_gtfa_transactions_processed", 1)
                    .await?;
            }

            pagination_token = result.pagination_token;

            if pagination_token.is_none() {
                log::info!("Reached end of pagination");
                break;
            }
        }

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}
