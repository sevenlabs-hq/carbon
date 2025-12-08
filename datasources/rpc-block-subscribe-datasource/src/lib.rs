use carbon_core::datasource::{BlockDetails, DatasourceDisconnection, DatasourceId};
use chrono::Utc;
use solana_hash::Hash;
use std::str::FromStr;

use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        metrics::MetricsCollection,
        transformers::transaction_metadata_from_original_meta,
    },
    core::time::Duration,
    futures::StreamExt,
    solana_client::{
        nonblocking::pubsub_client::PubsubClient,
        rpc_client::SerializableTransaction,
        rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    },
    std::sync::Arc,
    tokio::sync::mpsc,
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

const MAX_RECONNECTION_ATTEMPTS: u32 = 10;
const RECONNECTION_DELAY_MS: u64 = 3000;

#[derive(Debug, Clone)]
pub struct Filters {
    pub block_filter: RpcBlockSubscribeFilter,
    pub block_subscribe_config: Option<RpcBlockSubscribeConfig>,
}

impl Filters {
    pub const fn new(
        block_filter: RpcBlockSubscribeFilter,
        block_subscribe_config: Option<RpcBlockSubscribeConfig>,
    ) -> Self {
        Filters {
            block_filter,
            block_subscribe_config,
        }
    }
}

pub struct RpcBlockSubscribe {
    pub rpc_ws_url: String,
    pub filters: Filters,
    pub disconnect_notifier: Option<mpsc::Sender<DatasourceDisconnection>>,
}

impl RpcBlockSubscribe {
    pub const fn new(rpc_ws_url: String, filters: Filters, disconnect_notifier: Option<mpsc::Sender<DatasourceDisconnection>>) -> Self {
        Self {
            rpc_ws_url,
            filters,
            disconnect_notifier,
        }
    }
}

#[async_trait]
impl Datasource for RpcBlockSubscribe {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let mut reconnection_attempts = 0;
        let mut last_processed_slot = 0u64;
        let mut last_disconnect_time = None;
        let mut last_slot_before_disconnect = None;
        let disconnect_tx_clone = self.disconnect_notifier.clone();

        loop {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested, stopping reconnection attempts");
                break;
            }

            let client = match PubsubClient::new(&self.rpc_ws_url).await {
                Ok(client) => client,
                Err(err) => {
                    log::error!("Failed to create RPC subscribe client: {err}");
                    reconnection_attempts += 1;
                    if reconnection_attempts >= MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to create RPC subscribe client after {MAX_RECONNECTION_ATTEMPTS} attempts: {err}"
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
                    continue;
                }
            };

            let filters = self.filters.clone();
            let sender_clone = sender.clone();
            let id_for_loop = id.clone();

            let (mut block_stream, _block_unsub) = match client
                .block_subscribe(filters.block_filter, filters.block_subscribe_config)
                .await
            {
                Ok(subscription) => subscription,
                Err(err) => {
                    log::error!("Failed to subscribe to block updates: {err:?}");
                    reconnection_attempts += 1;
                    if reconnection_attempts > MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to subscribe after {MAX_RECONNECTION_ATTEMPTS} attempts: {err}"
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
                    continue;
                }
            };

            reconnection_attempts = 0;

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancellation requested, stopping subscription...");
                        return Ok(());
                    }
                    block_event_result = tokio::time::timeout(
                        Duration::from_secs(30),
                        block_stream.next()
                    ) => {
                        let block_event = match block_event_result {
                            Ok(Some(event)) => event,
                            Ok(None) => {
                                log::warn!("Block stream closed");
                                if last_disconnect_time.is_none() {
                                    last_disconnect_time = Some(Utc::now());
                                    last_slot_before_disconnect = Some(last_processed_slot);
                                    log::warn!("Disconnected at slot {}", last_processed_slot);
                                }
                                break;
                            }
                            Err(_) => {
                                log::warn!("Block stream timeout - no messages for 30 seconds");
                                if last_disconnect_time.is_none() {
                                    last_disconnect_time = Some(Utc::now());
                                    last_slot_before_disconnect = Some(last_processed_slot);
                                    log::warn!("Disconnected at slot {} (timeout)", last_processed_slot);
                                }
                                break;
                            }
                        };

                        match Some(block_event) {
                            Some(tx_event) => {
                                let slot = tx_event.context.slot;

                                if last_processed_slot > 0 {
                                    if let (Some(disconnect_time), Some(last_slot)) =
                                        (last_disconnect_time.take(), last_slot_before_disconnect.take())
                                    {
                                        let missed = if slot > last_slot { slot - last_slot } else { 0 };

                                        log::warn!("Reconnected: last_slot={}, new_slot={}, missed={}", last_slot, slot, missed);

                                        let disconnection = DatasourceDisconnection {
                                            source: "rpc-websocket".to_string(),
                                            disconnect_time,
                                            last_slot_before_disconnect: last_slot,
                                            first_slot_after_reconnect: slot,
                                            missed_slots: missed,
                                        };

                                        if let Some(tx) = &disconnect_tx_clone {
                                            match tx.try_send(disconnection) {
                                                Ok(_) => log::warn!("Disconnection event sent successfully"),
                                                Err(e) => log::error!("Failed to send disconnection event: {:?}", e),
                                            }
                                        } else {
                                            log::warn!("No disconnect channel configured");
                                        }
                                    }
                                }

                                last_processed_slot = slot;

                                if let Some(block) = tx_event.value.block {
                                    let block_start_time = std::time::Instant::now();
                                    let block_hash = Hash::from_str(&block.blockhash).ok();
                                    let previous_block_hash = Hash::from_str(&block.previous_blockhash).ok();

                                    let block_deteils = Update::BlockDetails( BlockDetails {
                                                slot,
                                                block_hash,
                                                previous_block_hash,
                                                rewards: block.rewards,
                                                num_reward_partitions: block.num_reward_partitions,
                                                block_time: block.block_time,
                                                block_height: block.block_height,
                                    });

                                    if let Err(err) = sender_clone.try_send((block_deteils, id_for_loop.clone())) {
                                        log::error!("Error sending block details: {err:?}");
                                        break;
                                    }

                                    if let Some(transactions) = block.transactions {
                                        for (tx_index, encoded_transaction_with_status_meta) in transactions.into_iter().enumerate() {
                                            let start_time = std::time::Instant::now();

                                            let meta_original = if let Some(meta) = encoded_transaction_with_status_meta.clone().meta {
                                                meta
                                            } else {
                                                continue;
                                            };

                                            if meta_original.status.is_err() {
                                                continue;
                                            }

                                            let Some(decoded_transaction) = encoded_transaction_with_status_meta.transaction.decode() else {
                                                log::error!("Failed to decode transaction: {encoded_transaction_with_status_meta:?}");
                                                continue;
                                            };

                                            let Ok(meta_needed) = transaction_metadata_from_original_meta(meta_original) else {
                                                log::error!("Error getting metadata from transaction original meta.");
                                                continue;
                                            };

                                            let update = Update::Transaction(Box::new(TransactionUpdate {
                                                signature: *decoded_transaction.get_signature(),
                                                transaction: decoded_transaction.clone(),
                                                meta: meta_needed,
                                                is_vote: false,
                                                slot,
                                                index: Some(tx_index as u64),
                                                block_time: block.block_time,
                                                block_hash,
                                            }));

                                            metrics
                                                .record_histogram(
                                                    "block_subscribe_transaction_process_time_nanoseconds",
                                                    start_time.elapsed().as_nanos() as f64
                                                )
                                                .await
                                                .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));

                                            metrics.increment_counter("block_subscribe_transactions_processed", 1)
                                                .await
                                                .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));

                                            if let Err(err) = sender_clone.try_send((update, id_for_loop.clone())) {
                                                log::error!("Error sending transaction update: {err:?}");
                                                break;
                                            }
                                        }
                                    }

                                    metrics
                                        .record_histogram(
                                            "block_subscribe_block_process_time_nanoseconds",
                                            block_start_time.elapsed().as_nanos() as f64
                                        )
                                        .await
                                        .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));

                                    metrics.increment_counter("block_subscribe_blocks_received", 1)
                                        .await
                                        .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));
                                }
                            }
                            None => {
                                log::warn!("Block stream has been closed, attempting to reconnect...");
                                break;
                            }
                        }
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
        }

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}
