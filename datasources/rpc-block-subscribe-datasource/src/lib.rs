use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
    metrics::MetricsCollection,
    transformers::transaction_metadata_from_original_meta,
};
use futures::StreamExt;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_client::SerializableTransaction,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Filters {
    pub block_filter: RpcBlockSubscribeFilter,
    pub block_subscribe_config: Option<RpcBlockSubscribeConfig>,
}

impl Filters {
    pub fn new(
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
    pub fn new(rpc_ws_url: String, filters: Filters) -> Self {
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
        sender: &UnboundedSender<Update>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let client = PubsubClient::new(&self.rpc_ws_url).await.map_err(|err| {
            carbon_core::error::Error::Custom(format!(
                "Failed to create an RPC subscribe client: {err}"
            ))
        })?;

        let filters = self.filters.clone();
        let sender = sender.clone();

        tokio::spawn(async move {
            let sender_clone = sender.clone();
            let (mut stream, _unsub) = match client
                .block_subscribe(filters.block_filter, filters.block_subscribe_config)
                .await
            {
                Ok(subscription) => subscription,
                Err(err) => {
                    log::error!("Failed to subscribe to blocks updates: {:?}", err);
                    return;
                }
            };

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancelling RPC blocks subscription...");
                        break;
                    }
                    event_result = stream.next() => {
                        match event_result {
                            Some(tx_event) => {
                                let slot = tx_event.context.slot;

                                if let Some(block) = tx_event.value.block {
                                    let block_start_time = std::time::Instant::now();
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

                                            let update = Update::Transaction(TransactionUpdate {
                                                signature: *decoded_transaction.get_signature(),
                                                transaction: decoded_transaction.clone(),
                                                meta: meta_needed,
                                                is_vote: false,
                                                slot,
                                            });

                                            metrics
                                                    .record_histogram(
                                                        "block_subscribe_transaction_process_time_nanoseconds",
                                                        start_time.elapsed().as_nanos() as f64
                                                    )
                                                    .await
                                                    .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                            metrics.increment_counter("block_subscribe_transactions_processed", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));


                                            if let Err(err) = sender_clone.send(update) {
                                                log::error!("Error sending transaction update: {:?}", err);
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
                                            .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                    metrics.increment_counter("block_subscribe_blocks_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));
                                }
                            }
                            None => {
                                log::info!("Blocks stream has been closed");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}
