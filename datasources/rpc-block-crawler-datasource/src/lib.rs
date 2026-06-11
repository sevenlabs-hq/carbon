pub use solana_client::rpc_config::RpcBlockConfig;
use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        metrics::{Counter, Histogram, MetricsRegistry},
        transformers::transaction_metadata_from_original_meta,
    },
    futures::StreamExt,
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction},
    solana_commitment_config::CommitmentConfig,
    solana_hash::Hash,
    solana_transaction_status::UiConfirmedBlock,
    std::{
        str::FromStr,
        sync::{Arc, LazyLock},
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
const GET_BLOCK_MAX_RETRIES: usize = 3;
const GET_BLOCK_RETRY_BACKOFF: Duration = Duration::from_millis(500);

static BLOCKS_FETCH_TIMES_MILLIS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "block_crawler_blocks_fetch_times_milliseconds",
        "Time to fetch block in milliseconds",
        vec![1.0, 10.0, 50.0, 100.0, 500.0, 1000.0, 5000.0],
    )
});
static BLOCKS_FETCHED: Counter = Counter::new(
    "block_crawler_blocks_fetched_total",
    "Blocks fetched by block crawler",
);
static BLOCKS_SKIPPED: Counter = Counter::new(
    "block_crawler_blocks_skipped_total",
    "Blocks skipped by block crawler",
);
static BLOCKS_RECEIVED: Counter = Counter::new(
    "block_crawler_blocks_received_total",
    "Blocks received by task processor",
);
static TRANSACTION_PROCESS_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "block_crawler_transaction_process_time_nanoseconds",
        "Time to process transaction in nanoseconds",
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
static TRANSACTIONS_PROCESSED: Counter = Counter::new(
    "block_crawler_transactions_processed_total",
    "Transactions processed by block crawler",
);
static BLOCK_PROCESS_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "block_crawler_block_process_time_nanoseconds",
        "Time to process block in nanoseconds",
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
static BLOCKS_PROCESSED: Counter = Counter::new(
    "block_crawler_blocks_processed_total",
    "Blocks processed by block crawler",
);

fn register_block_crawler_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&BLOCKS_FETCHED);
    registry.register_counter(&BLOCKS_SKIPPED);
    registry.register_counter(&BLOCKS_RECEIVED);
    registry.register_counter(&TRANSACTIONS_PROCESSED);
    registry.register_counter(&BLOCKS_PROCESSED);
    registry.register_histogram(&BLOCKS_FETCH_TIMES_MILLIS);
    registry.register_histogram(&TRANSACTION_PROCESS_TIME_NANOS);
    registry.register_histogram(&BLOCK_PROCESS_TIME_NANOS);
}

fn skippable_block_error_code(error: &str) -> Option<&'static str> {
    ["-32001", "-32007", "-32009"]
        .into_iter()
        .find(|code| error.contains(code))
}

fn is_retryable_block_error(error: &str) -> bool {
    error.contains("TimedOut")
        || error.contains("IncompleteMessage")
        || error.contains("-32004")
        || error.contains("-32014")
        || error.contains("429")
        || error.contains("Too Many Requests")
}

/// RpcBlockCrawler is a datasource that crawls the Solana blockchain for blocks
/// and sends them to the sender. It uses a channel to send blocks to the task
/// processor.
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
    ) -> CarbonResult<()> {
        register_block_crawler_metrics();
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
        );

        let task_processor = task_processor(block_receiver, sender, id, cancellation_token.clone());

        tokio::spawn(async move {
            if let Err(error) = block_fetcher.await {
                log::error!("RPC Crawler block fetcher task failed: {error:?}");
            }

            if let Err(error) = task_processor.await {
                log::error!("RPC Crawler task processor failed: {error:?}");
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
                                            "Waiting for new blocks... Current: {current_slot}, Latest: {latest_slot}"
                                        );
                                        tokio::time::sleep(block_interval).await;
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    log::error!("Error fetching latest slot: {e:?}");
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

                    async move {
                        let start = Instant::now();
                        let mut attempt = 0usize;
                        loop {
                            match rpc_client.get_block_with_config(slot, block_config).await {
                                Ok(block) => {
                                    let time_taken = start.elapsed().as_millis();
                                    BLOCKS_FETCH_TIMES_MILLIS.record(time_taken as f64);
                                    BLOCKS_FETCHED.inc();
                                    break Some((slot, block));
                                }
                                Err(e) => {
                                    let error = format!("{e:?}");
                                    // https://support.quicknode.com/hc/en-us/articles/16459608696721-Solana-RPC-Error-Code-Reference
                                    // Solana permanent skip errors:
                                    // -32001 cleaned up, -32007 skipped/missing slot,
                                    // -32009 missing in long-term storage.
                                    if let Some(error_code) = skippable_block_error_code(&error) {
                                        log::warn!(
                                            "Skipping block at slot {slot}: skippable RPC error {error_code}: {e:?}"
                                        );
                                        BLOCKS_SKIPPED.inc();
                                        break None;
                                    }

                                    if attempt < GET_BLOCK_MAX_RETRIES
                                        && is_retryable_block_error(&error)
                                    {
                                        attempt += 1;
                                        log::debug!(
                                            "Retrying block fetch for slot {slot}; attempt {attempt}/{GET_BLOCK_MAX_RETRIES}: {e:?}"
                                        );
                                        tokio::time::sleep(
                                            GET_BLOCK_RETRY_BACKOFF.saturating_mul(attempt as u32),
                                        )
                                        .await;
                                        continue;
                                    }

                                    log::error!("Error fetching block at slot {slot}: {e:?}");
                                    break None;
                                }
                            }
                        }
                    }
                })
                .buffer_unordered(max_concurrent_requests)
                .for_each(|result| async {
                    if let Some((slot, block)) = result {
                        if let Err(e) = block_sender.send((slot, block)).await {
                            log::error!("Failed to send block: {e:?}");
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
) -> JoinHandle<()> {
    let mut block_receiver = block_receiver;
    let sender = sender.clone();
    let id_for_loop = id.clone();

    tokio::spawn(async move {
        while let Some((slot, block)) = block_receiver.recv().await {
            BLOCKS_RECEIVED.inc();
            let block_start_time = Instant::now();
            let block_hash = Hash::from_str(&block.blockhash).ok();
            if let Some(transactions) = block.transactions {
                for (tx_index, encoded_transaction_with_status_meta) in
                    transactions.into_iter().enumerate()
                {
                    let start_time = std::time::Instant::now();

                    let meta_original =
                        if let Some(meta) = encoded_transaction_with_status_meta.clone().meta {
                            meta
                        } else {
                            continue;
                        };

                    if meta_original.status.is_err() {
                        continue;
                    }

                    let Some(decoded_transaction) =
                        encoded_transaction_with_status_meta.transaction.decode()
                    else {
                        log::error!(
                            "Failed to decode transaction: {encoded_transaction_with_status_meta:?}"
                        );
                        continue;
                    };

                    let Ok(meta_needed) = transaction_metadata_from_original_meta(meta_original)
                    else {
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

                    TRANSACTION_PROCESS_TIME_NANOS.record(start_time.elapsed().as_nanos() as f64);
                    TRANSACTIONS_PROCESSED.inc();

                    let send_result = tokio::select! {
                        _ = cancellation_token.cancelled() => {
                            log::info!("Cancelling RPC Crawler task processor...");
                            return;
                        }
                        result = sender.send((update, id_for_loop.clone())) => result,
                    };

                    if let Err(err) = send_result {
                        log::error!("Error sending transaction update: {err:?}");
                        cancellation_token.cancel();
                        return;
                    }
                }
            }
            BLOCK_PROCESS_TIME_NANOS.record(block_start_time.elapsed().as_nanos() as f64);
            BLOCKS_PROCESSED.inc();
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_error_classifier_matches_agave_error_semantics() {
        assert!(is_retryable_block_error("-32004 BlockNotAvailable"));
        assert!(is_retryable_block_error(
            "-32014 BlockStatusNotAvailableYet"
        ));
        assert!(is_retryable_block_error("TimedOut"));
        assert!(is_retryable_block_error("IncompleteMessage"));
        assert!(is_retryable_block_error("429 Too Many Requests"));

        assert_eq!(
            skippable_block_error_code("-32001 BlockCleanedUp"),
            Some("-32001")
        );
        assert_eq!(
            skippable_block_error_code("-32007 SlotSkipped"),
            Some("-32007")
        );
        assert_eq!(
            skippable_block_error_code("-32009 LongTermStorageSlotSkipped"),
            Some("-32009")
        );

        assert_eq!(skippable_block_error_code("-32004 BlockNotAvailable"), None);
        assert_eq!(
            skippable_block_error_code("-32014 BlockStatusNotAvailableYet"),
            None
        );
    }

    #[tokio::test]
    async fn task_processor_drains_buffered_blocks_until_channel_closes() {
        let before_received = BLOCKS_RECEIVED.get();
        let before_processed = BLOCKS_PROCESSED.get();
        let (block_sender, block_receiver) = mpsc::channel(2);
        let (update_sender, _update_receiver) = mpsc::channel(1);
        let handle = task_processor(
            block_receiver,
            update_sender,
            DatasourceId::new_named("test"),
            CancellationToken::new(),
        );

        for slot in 1..=3 {
            block_sender
                .send((
                    slot,
                    UiConfirmedBlock {
                        previous_blockhash: "previous".to_string(),
                        blockhash: "current".to_string(),
                        parent_slot: slot.saturating_sub(1),
                        transactions: Some(Vec::new()),
                        signatures: None,
                        rewards: None,
                        num_reward_partitions: None,
                        block_time: None,
                        block_height: None,
                    },
                ))
                .await
                .expect("processor should receive queued block");
        }
        drop(block_sender);

        tokio::time::timeout(Duration::from_secs(1), handle)
            .await
            .expect("processor should exit after the block channel closes")
            .expect("processor should not panic");

        assert_eq!(BLOCKS_RECEIVED.get() - before_received, 3);
        assert_eq!(BLOCKS_PROCESSED.get() - before_processed, 3);
    }

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
        );

        // Create a task to receive blocks
        let receiver_task = tokio::spawn(async move {
            let mut received_blocks = Vec::new();

            while let Some((slot, block)) = block_receiver.recv().await {
                println!("Received block at slot {slot}");
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
