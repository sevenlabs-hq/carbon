mod types;

use crate::types::{
    FetchHeliusGtfaTransactionsPageResult, HeliusGtfaRequest, HeliusGtfaRequestConfig,
    HeliusGtfaResponse,
};
use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        metrics::MetricsCollection,
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

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusGtfaFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<SlotFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<BlockTimeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<SignatureFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionStatusFilter>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RangeFilter<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<T>,
}

pub type SlotFilter = RangeFilter<u64>;
pub type SignatureFilter = RangeFilter<String>;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BlockTimeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum TransactionStatusFilter {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "any")]
    Any,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
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

    fn build_request_config(&self, pagination_token: Option<&str>) -> HeliusGtfaRequestConfig {
        let limit = self.config.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
        let sort_order = self.config.sort_order.as_ref().map(|so| match so {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        });

        let filters = self.config.filters.as_ref().and_then(|filters| {
            let has_slot = filters.slot.as_ref().is_some_and(|sf| {
                sf.gte.is_some() || sf.gt.is_some() || sf.lte.is_some() || sf.lt.is_some()
            });
            let has_block_time = filters.block_time.as_ref().is_some_and(|btf| {
                btf.gte.is_some()
                    || btf.gt.is_some()
                    || btf.lte.is_some()
                    || btf.lt.is_some()
                    || btf.eq.is_some()
            });
            let has_signature = filters.signature.as_ref().is_some_and(|sigf| {
                sigf.gte.is_some() || sigf.gt.is_some() || sigf.lte.is_some() || sigf.lt.is_some()
            });
            let has_status = filters.status.is_some();

            if has_slot || has_block_time || has_signature || has_status {
                Some(filters.clone())
            } else {
                None
            }
        });

        HeliusGtfaRequestConfig {
            transaction_details: "full".to_string(),
            limit,
            sort_order: sort_order.map(|s| s.to_string()),
            pagination_token: pagination_token.map(|s| s.to_string()),
            commitment: self
                .config
                .commitment
                .as_ref()
                .map(|c| c.commitment.to_string()),
            encoding: "base64".to_string(),
            max_supported_transaction_version: 0,
            min_context_slot: self.config.min_context_slot,
            filters,
        }
    }

    async fn fetch_transactions_page(
        &self,
        client: &reqwest::Client,
        pagination_token: Option<&str>,
    ) -> CarbonResult<FetchHeliusGtfaTransactionsPageResult> {
        let config = self.build_request_config(pagination_token);
        let config_value = serde_json::to_value(config).map_err(|e| {
            carbon_core::error::Error::FailedToConsumeDatasource(format!(
                "Failed to serialize request config: {e}"
            ))
        })?;

        let params = vec![
            serde_json::Value::String(self.address.to_string()),
            config_value,
        ];

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

            let status_filter = self
                .config
                .filters
                .clone()
                .and_then(|filters| filters.status);

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
                    meta: carbon_core::transformers::transaction_metadata_from_original_meta(
                        tx.meta.clone(),
                    )?,
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
