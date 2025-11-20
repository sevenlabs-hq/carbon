use carbon_core::datasource::{BlockDetails, DatasourceId};
use solana_hash::Hash;
use std::str::FromStr;

use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        metrics::{Counter, Histogram, MetricsRegistry},
        transformers::transaction_metadata_from_original_meta,
    },
    core::time::Duration,
    futures::StreamExt,
    solana_client::{
        nonblocking::pubsub_client::PubsubClient,
        rpc_client::SerializableTransaction,
        rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    },
    std::sync::OnceLock,
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

const MAX_RECONNECTION_ATTEMPTS: u32 = 10;
const RECONNECTION_DELAY_MS: u64 = 3000;

static TRANSACTION_PROCESS_TIME_NANOS: OnceLock<Histogram> = OnceLock::new();
static TRANSACTIONS_PROCESSED: Counter = Counter::new(
    "block_subscribe_transactions_processed",
    "Total transactions processed from block subscriptions",
);
static BLOCK_PROCESS_TIME_NANOS: OnceLock<Histogram> = OnceLock::new();
static BLOCKS_RECEIVED: Counter = Counter::new(
    "block_subscribe_blocks_received",
    "Total blocks received from block subscriptions",
);

fn init_histograms() {
    TRANSACTION_PROCESS_TIME_NANOS.get_or_init(|| {
        Histogram::new(
            "block_subscribe_transaction_process_time_nanoseconds",
            "Time taken to process transactions in nanoseconds",
            vec![
                1_000.0,
                10_000.0,
                100_000.0,
                1_000_000.0,
                10_000_000.0,
                100_000_000.0,
                1_000_000_000.0,
            ],
        )
    });
    BLOCK_PROCESS_TIME_NANOS.get_or_init(|| {
        Histogram::new(
            "block_subscribe_block_process_time_nanoseconds",
            "Time taken to process blocks in nanoseconds",
            vec![
                1_000_000.0,
                10_000_000.0,
                100_000_000.0,
                1_000_000_000.0,
                10_000_000_000.0,
            ],
        )
    });
}

fn register_block_subscribe_metrics() {
    init_histograms();
    let registry = MetricsRegistry::global();
    registry.register_counter(&TRANSACTIONS_PROCESSED);
    registry.register_counter(&BLOCKS_RECEIVED);
    registry.register_histogram(TRANSACTION_PROCESS_TIME_NANOS.get().unwrap());
    registry.register_histogram(BLOCK_PROCESS_TIME_NANOS.get().unwrap());
}

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
}

impl RpcBlockSubscribe {
    pub const fn new(rpc_ws_url: String, filters: Filters) -> Self {
        Self {
            rpc_ws_url,
            filters,
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
    ) -> CarbonResult<()> {
        register_block_subscribe_metrics();
        let mut reconnection_attempts = 0;

        loop {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested, stopping reconnection attempts");
                break;
            }

            let client = match PubsubClient::new(&self.rpc_ws_url).await {
                Ok(client) => client,
                Err(err) => {
                    log::error!("Failed to create RPC subscribe client: {}", err);
                    reconnection_attempts += 1;
                    if reconnection_attempts >= MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to create RPC subscribe client after {} attempts: {}",
                            MAX_RECONNECTION_ATTEMPTS, err
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
                    log::error!("Failed to subscribe to block updates: {:?}", err);
                    reconnection_attempts += 1;
                    if reconnection_attempts > MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to subscribe after {} attempts: {}",
                            MAX_RECONNECTION_ATTEMPTS, err
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
                    block_event = block_stream.next() => {
                        match block_event {
                            Some(tx_event) => {
                                let slot = tx_event.context.slot;

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
                                        log::error!("Error sending block details: {:?}", err);
                                        break;
                                    }

                                    if let Some(transactions) = block.transactions {
                                        for encoded_transaction_with_status_meta in transactions {
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
                                                log::error!("Failed to decode transaction: {:?}", encoded_transaction_with_status_meta);
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
                                                block_time: block.block_time,
                                                block_hash,
                                            }));

                                            TRANSACTION_PROCESS_TIME_NANOS
                                                .get()
                                                .unwrap()
                                                .record(start_time.elapsed().as_nanos() as f64);
                                            TRANSACTIONS_PROCESSED.inc();

                                            if let Err(err) = sender_clone.try_send((update, id_for_loop.clone())) {
                                                log::error!("Error sending transaction update: {:?}", err);
                                                break;
                                            }
                                        }
                                    }

                                    BLOCK_PROCESS_TIME_NANOS
                                        .get()
                                        .unwrap()
                                        .record(block_start_time.elapsed().as_nanos() as f64);
                                    BLOCKS_RECEIVED.inc();
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
