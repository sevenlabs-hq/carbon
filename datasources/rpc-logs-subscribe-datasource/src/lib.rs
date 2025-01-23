use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, LogsUpdate, Update, UpdateType},
    error::CarbonResult,
    metrics::MetricsCollection,
};
use futures::StreamExt;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::{str::FromStr, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Filters {
    pub pubkeys: Vec<Pubkey>,
    pub config: RpcTransactionLogsConfig,
}

impl Filters {
    pub fn new(pubkeys: Vec<Pubkey>, config: RpcTransactionLogsConfig) -> Self {
        Filters { pubkeys, config }
    }
}

pub struct RpcLogsSubscribe {
    pub rpc_ws_url: String,
    pub filters: Filters,
}

impl RpcLogsSubscribe {
    pub fn new(rpc_ws_url: String, filters: Filters) -> Self {
        Self {
            rpc_ws_url,
            filters,
        }
    }
}

#[async_trait]
impl Datasource for RpcLogsSubscribe {
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
            let program_mentions: Vec<String> =
                filters.pubkeys.iter().map(|p| p.to_string()).collect();
            let (mut stream, _unsub) = match client
                .logs_subscribe(
                    RpcTransactionLogsFilter::Mentions(program_mentions),
                    filters.config,
                )
                .await
            {
                Ok(subscription) => subscription,
                Err(err) => {
                    log::error!("Failed to subscribe to logs updates: {:?}", err);
                    return;
                }
            };

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancelling RPC logs subscription...");
                        break;
                    }
                    event_result = stream.next() => {
                        match event_result {
                            Some(log_event) => {
                                let start_time = std::time::Instant::now();

                                let Ok(signature) = Signature::from_str(&log_event.value.signature) else {
                                    log::error!("Error parsing signature. Value: {}", &log_event.value.signature);
                                    continue;
                                };

                                let update = Update::Logs(LogsUpdate {
                                    signature,
                                    slot: log_event.context.slot,
                                    logs: log_event.value.logs,
                                });

                                metrics
                                    .record_histogram(
                                        "logs_subscribe_process_time_nanoseconds",
                                        start_time.elapsed().as_nanos() as f64
                                    )
                                    .await
                                    .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                metrics.increment_counter("logs_subscribe_logs_processed", 1)
                                    .await
                                    .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                if let Err(err) = sender_clone.send(update) {
                                    log::error!("Error sending logs update: {:?}", err);
                                    break;
                                }
                            }
                            None => {
                                log::info!("Logs stream has been closed");
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
        vec![UpdateType::Logs]
    }
}
