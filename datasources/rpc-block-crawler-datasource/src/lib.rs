use carbon_core::datasource::DatasourceId;
pub use solana_client::rpc_config::RpcBlockConfig;
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
    futures::StreamExt,
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction},
    solana_commitment_config::CommitmentConfig,
    solana_transaction_status::UiConfirmedBlock,
    std::{
        sync::Arc,
        time::{Duration, Instant},
    },
    tokio::{
        sync::mpsc::{self, Receiver, Sender},
        task::JoinHandle,
    },
    tokio_util::sync::CancellationToken,
};

const CHANNEL_BUFFER_SIZE: usize = 1000;
const MAX_CONCURRENT_REQUESTS: usize = 10;
const BLOCK_INTERVAL: Duration = Duration::from_millis(100);

/// RpcBlockCrawler is a datasource that crawls the Solana blockchain for blocks and sends them to the sender.
/// It uses a channel to send blocks to the task processor.
pub struct RpcBlockCrawler {
    pub rpc_url: String,
    pub start_slot: u64,
    pub end_slot: Option<u64>,
    pub block_interval: Duration,
    pub block_config: RpcBlockConfig,
    pub max_concurrent_requests: usize,
    pub channel_buffer_size: usize,
}

impl RpcBlockCrawler {
    pub fn new(
        rpc_url: String,
        start_slot: u64,
        end_slot: Option<u64>,
        block_interval: Option<Duration>,
        block_config: RpcBlockConfig,
        max_concurrent_requests: Option<usize>,
        channel_buffer_size: Option<usize>,
    ) -> Self {
        Self {
            rpc_url,
            start_slot,
            end_slot,
            block_config,
            block_interval: block_interval.unwrap_or(BLOCK_INTERVAL),
            max_concurrent_requests: max_concurrent_requests.unwrap_or(MAX_CONCURRENT_REQUESTS),
            channel_buffer_size: channel_buffer_size.unwrap_or(CHANNEL_BUFFER_SIZE),
        }
    }
}

#[async_trait]
impl Datasource for RpcBlockCrawler {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            self.rpc_url.clone(),
            self.block_config
                .commitment
                .unwrap_or(CommitmentConfig::confirmed()),
        ));
        let (block_sender, block_receiver) = mpsc::channel(self.channel_buffer_size);

        let block_fetcher = block_fetcher(
            rpc_client,
            self.start_slot,
            self.end_slot,
            self.block_interval,
            self.block_config,
            block_sender,
            self.max_concurrent_requests,
            cancellation_token.clone(),
            metrics.clone(),
        );

        let task_processor = task_processor(
            block_receiver,
            sender,
            id,
            cancellation_token.clone(),
            metrics.clone(),
        );

        tokio::spawn(async move {
            tokio::select! {
                _ = block_fetcher => {},
                _ = task_processor => {},
            }
        });

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}

#[allow(clippy::too_many_arguments)]
fn block_fetcher(
    rpc_client: Arc<RpcClient>,
    start_slot: u64,
    end_slot: Option<u64>,
    block_interval: Duration,
    block_config: RpcBlockConfig,
    block_sender: Sender<(u64, UiConfirmedBlock)>,
    max_concurrent_requests: usize,
    cancellation_token: CancellationToken,
    metrics: Arc<MetricsCollection>,
) -> JoinHandle<()> {
    let rpc_client_clone = rpc_client.clone();
    tokio::spawn(async move {
        let fetch_stream_task = async {
            let fetch_stream = async_stream::stream! {
                let mut current_slot = start_slot;
                let mut latest_slot = current_slot;
                loop {
                    if let Some(end) = end_slot {
                        if current_slot > end {
                            break;
                        }
                    } else {
                        if current_slot >= latest_slot {
                            match rpc_client_clone.get_slot().await {
                                Ok(slot) => {
                                    latest_slot = slot;
                                    if current_slot > latest_slot {
                                        log::debug!(
                                            "Waiting for new blocks... Current: {}, Latest: {}",
                                            current_slot,
                                            latest_slot
                                        );
                                        tokio::time::sleep(block_interval).await;
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    log::error!("Error fetching latest slot: {:?}", e);
                                    tokio::time::sleep(block_interval).await;
                                    continue;
                                }
                            }
                        }
                        if latest_slot - current_slot > 100 {
                            log::debug!(
                                "Current slot {} is behind latest slot {} by {}",
                                current_slot,
                                latest_slot,
                                latest_slot - current_slot
                            );
                        }
                    }
                    yield current_slot;
                    current_slot += 1;
                }
            };

            fetch_stream
                .map(|slot| {
                    let rpc_client = Arc::clone(&rpc_client);
                    let metrics = metrics.clone();

                    async move {
                        let start = Instant::now();
                        match rpc_client.get_block_with_config(slot, block_config).await {
                            Ok(block) => {
                                let time_taken = start.elapsed().as_millis();
                                metrics
                                    .record_histogram(
                                        "block_crawler_blocks_fetch_times_milliseconds",
                                        time_taken as f64,
                                    )
                                    .await
                                    .unwrap_or_else(|value| {
                                        log::error!("Error recording metric: {}", value)
                                    });

                                metrics
                                    .increment_counter("block_crawler_blocks_fetched", 1)
                                    .await
                                    .unwrap_or_else(|value| {
                                        log::error!("Error recording metric: {}", value)
                                    });

                                Some((slot, block))
                            }
                            Err(e) => {
                                // https://support.quicknode.com/hc/en-us/articles/16459608696721-Solana-RPC-Error-Code-Reference
                                // solana skippable errors
                                // -32004, // Block not available for slot x
                                // -32007, // Slot {} was skipped, or missing due to ledger jump to recent snapshot
                                // -32009, // Slot {} was skipped, or missing in long-term storage
                                if e.to_string().contains("-32009")
                                    || e.to_string().contains("-32004")
                                    || e.to_string().contains("-32007")
                                {
                                    metrics
                                        .increment_counter("block_crawler_blocks_skipped", 1)
                                        .await
                                        .unwrap_or_else(|value| {
                                            log::error!("Error recording metric: {}", value)
                                        });
                                } else {
                                    log::error!("Error fetching block at slot {}: {:?}", slot, e);
                                }
                                None
                            }
                        }
                    }
                })
                .buffer_unordered(max_concurrent_requests)
                .for_each(|result| async {
                    if let Some((slot, block)) = result {
                        if let Err(e) = block_sender.send((slot, block)).await {
                            log::error!("Failed to send block: {:?}", e);
                        }
                    }
                })
                .await;
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                log::info!("Cancelling RPC Crawler block fetcher...");
            }
            _ = fetch_stream_task => {}
        }
    })
}

/// Process the block and send the transactions to the sender
fn task_processor(
    block_receiver: Receiver<(u64, UiConfirmedBlock)>,
    sender: Sender<(Update, DatasourceId)>,
    id: DatasourceId,
    cancellation_token: CancellationToken,
    metrics: Arc<MetricsCollection>,
) -> JoinHandle<()> {
    let mut block_receiver = block_receiver;
    let sender = sender.clone();
    let id_for_loop = id.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
            _ = cancellation_token.cancelled() => {
                log::info!("Cancelling RPC Crawler task processor...");
                break;
            }
            maybe_block = block_receiver.recv() => {
                match maybe_block {
                    Some((slot, block)) => {

                        metrics
                            .increment_counter("block_crawler_blocks_received", 1)
                            .await
                            .unwrap_or_else(|value| {
                                log::error!("Error recording metric: {}", value)
                            });
                        let block_start_time = Instant::now();
                        let block_hash = Hash::from_str(&block.blockhash).ok();
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

                                metrics
                                    .record_histogram(
                                        "block_crawler_transaction_process_time_nanoseconds",
                                        start_time.elapsed().as_nanos() as f64
                                    )
                                    .await
                                    .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                metrics.increment_counter("block_crawler_transactions_processed", 1)
                                    .await
                                    .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                if let Err(err) = sender.try_send((update, id_for_loop.clone())) {
                                    log::error!("Error sending transaction update: {:?}", err);
                                    break;
                                }
                            }
                        }
                        metrics
                            .record_histogram(
                                "block_crawler_block_process_time_nanoseconds",
                                block_start_time.elapsed().as_nanos() as f64
                            ).await
                            .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                        metrics
                            .increment_counter("block_crawler_blocks_processed", 1)
                            .await
                            .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));
                    }
                    None => {
                        break;
                    }
                }
            }}
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_block_fetcher_with_end_slot() {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            "https://api.mainnet-beta.solana.com/".to_string(),
            CommitmentConfig::confirmed(),
        ));
        let block_interval = Duration::from_millis(100);
        let cancellation_token = CancellationToken::new();
        let (block_sender, mut block_receiver) = mpsc::channel(1);

        let block_config = RpcBlockConfig {
            max_supported_transaction_version: Some(0),
            ..Default::default()
        };

        // Start block_fetcher
        let block_fetcher = block_fetcher(
            rpc_client,
            328837890,
            Some(328837901),
            block_interval,
            block_config,
            block_sender,
            1,
            cancellation_token.clone(),
            Arc::new(MetricsCollection::new(vec![])),
        );

        // Create a task to receive blocks
        let receiver_task = tokio::spawn(async move {
            let mut received_blocks = Vec::new();

            while let Some((slot, block)) = block_receiver.recv().await {
                received_blocks.push((slot, block));

                if received_blocks.len() == 2 {
                    break;
                }
            }
            received_blocks
        });

        tokio::spawn(async move {
            block_fetcher.await.expect("Block fetcher should not panic");
        });

        // Wait for both block_fetcher and receiver task to complete
        let exit_reason = tokio::select! {
            result = receiver_task => {
                let received_blocks = result.expect("Receiver task should not panic");
                println!("Received {} blocks", received_blocks.len());

                for (slot, block) in received_blocks {
                    println!("Block at slot {}: {} transactions",
                        slot,
                        block.transactions.map(|t| t.len()).unwrap_or(0)
                    );
                }
                "receiver_completed"
            }
            _ = cancellation_token.cancelled() => {
                println!("Cancellation token triggered");
                "cancellation_token"
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {
                println!("Timeout");
                "timeout"
            }
        };

        assert_eq!(
            exit_reason, "receiver_completed",
            "Test should exit because block fetcher completed"
        );
    }

    #[tokio::test]
    async fn test_block_fetcher_without_end_slot() {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            "https://api.mainnet-beta.solana.com/".to_string(),
            CommitmentConfig::confirmed(),
        ));
        let latest_slot = rpc_client
            .get_slot()
            .await
            .expect("Failed to get last slot");

        let block_interval = Duration::from_millis(100);
        let cancellation_token = CancellationToken::new();
        let (block_sender, mut block_receiver) = mpsc::channel(1);

        let block_config = RpcBlockConfig {
            max_supported_transaction_version: Some(0),
            ..Default::default()
        };

        // Start block_fetcher
        let block_fetcher = block_fetcher(
            rpc_client,
            latest_slot,
            None,
            block_interval,
            block_config,
            block_sender,
            2,
            cancellation_token.clone(),
            Arc::new(MetricsCollection::new(vec![])),
        );

        // Create a task to receive blocks
        let receiver_task = tokio::spawn(async move {
            let mut received_blocks = Vec::new();

            while let Some((slot, block)) = block_receiver.recv().await {
                println!("Received block at slot {}", slot);
                received_blocks.push((slot, block));

                if received_blocks.len() == 2 {
                    break;
                }
            }
            received_blocks
        });

        tokio::spawn(async move {
            block_fetcher.await.expect("Block fetcher should not panic");
        });

        // Wait for both block_fetcher and receiver task to complete
        let exit_reason = tokio::select! {
            result = receiver_task => {
                let received_blocks = result.expect("Receiver task should not panic");
                println!("Received {} blocks", received_blocks.len());

                for (slot, block) in received_blocks {
                    println!("Block at slot {}: {} transactions",
                        slot,
                        block.transactions.map(|t| t.len()).unwrap_or(0)
                    );
                }
                "receiver_completed"
            }
            _ = cancellation_token.cancelled() => {
                println!("Cancellation token triggered");
                "cancellation_token"
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {
                println!("Timeout");
                "timeout"
            }
        };

        assert_eq!(
            exit_reason, "receiver_completed",
            "Test should exit because block fetcher completed"
        );
    }
}
