use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        metrics::MetricsCollection,
        transformers::transaction_metadata_from_original_meta,
    },
    futures::StreamExt,
    solana_client::{
        nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
        rpc_config::RpcTransactionConfig,
    },
    solana_commitment_config::CommitmentConfig,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_transaction_status::{
        EncodedConfirmedTransactionWithStatusMeta, UiLoadedAddresses, UiTransactionEncoding,
    },
    std::{collections::HashSet, str::FromStr, sync::Arc, time::Duration},
    tokio::{
        sync::mpsc::{self, Receiver, Sender},
        task::JoinHandle,
        time::Instant,
    },
    tokio_util::sync::CancellationToken,
};

#[derive(Debug, Clone)]
pub struct Filters {
    pub accounts: Option<Vec<Pubkey>>,
    pub before_signature: Option<Signature>,
    pub until_signature: Option<Signature>,
}

impl Filters {
    pub const fn new(
        accounts: Option<Vec<Pubkey>>,
        before_signature: Option<Signature>,
        until_signature: Option<Signature>,
    ) -> Self {
        Filters {
            accounts,
            before_signature,
            until_signature,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub backoff_multiplier: f64,
}

impl RetryConfig {
    pub const fn new(
        max_retries: u32,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
        backoff_multiplier: f64,
    ) -> Self {
        RetryConfig {
            max_retries,
            initial_backoff_ms,
            max_backoff_ms,
            backoff_multiplier,
        }
    }

    pub const fn default() -> Self {
        RetryConfig {
            max_retries: 3,
            initial_backoff_ms: 1000,
            max_backoff_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }

    pub const fn no_retry() -> Self {
        RetryConfig {
            max_retries: 0,
            initial_backoff_ms: 0,
            max_backoff_ms: 0,
            backoff_multiplier: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub batch_limit: usize,
    pub polling_interval: Duration,
    pub max_concurrent_requests: usize,
    pub max_signature_channel_size: Option<usize>,
    pub max_transaction_channel_size: Option<usize>,
    pub retry_config: RetryConfig,
    pub blocking_send: bool,
}

impl ConnectionConfig {
    pub const fn new(
        batch_limit: usize,
        polling_interval: Duration,
        max_concurrent_requests: usize,
        retry_config: RetryConfig,
        max_signature_channel_size: Option<usize>, // None will default to 1000
        max_transaction_channel_size: Option<usize>, // None will default to 1000
        blocking_send: bool,
    ) -> Self {
        ConnectionConfig {
            batch_limit,
            polling_interval,
            max_concurrent_requests,
            retry_config,
            max_signature_channel_size,
            max_transaction_channel_size,
            blocking_send,
        }
    }

    pub const fn default() -> Self {
        ConnectionConfig {
            batch_limit: 100,
            polling_interval: Duration::from_secs(5),
            max_concurrent_requests: 5,
            retry_config: RetryConfig::default(),
            max_signature_channel_size: None,
            max_transaction_channel_size: None,
            blocking_send: false,
        }
    }
}

pub struct RpcTransactionCrawler {
    pub rpc_url: String,
    pub account: Pubkey,
    pub connection_config: ConnectionConfig,
    pub filters: Filters,
    pub commitment: Option<CommitmentConfig>,
}

impl RpcTransactionCrawler {
    pub const fn new(
        rpc_url: String,
        account: Pubkey,
        connection_config: ConnectionConfig,
        filters: Filters,
        commitment: Option<CommitmentConfig>,
    ) -> Self {
        RpcTransactionCrawler {
            rpc_url,
            account,
            connection_config,
            filters,
            commitment,
        }
    }
}

#[async_trait]
impl Datasource for RpcTransactionCrawler {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            self.rpc_url.clone(),
            self.commitment.unwrap_or(CommitmentConfig::confirmed()),
        ));
        let account = self.account;
        let filters = self.filters.clone();
        let sender = sender.clone();
        let commitment = self.commitment;

        let (signature_sender, signature_receiver) = mpsc::channel(
            self.connection_config
                .max_signature_channel_size
                .unwrap_or(1000),
        );
        let (transaction_sender, transaction_receiver) = mpsc::channel(
            self.connection_config
                .max_transaction_channel_size
                .unwrap_or(1000),
        );

        let signature_fetcher = signature_fetcher(
            rpc_client.clone(),
            account,
            self.connection_config.clone(),
            signature_sender,
            filters.clone(),
            commitment,
            cancellation_token.clone(),
            metrics.clone(),
        );

        let transaction_fetcher = transaction_fetcher(
            rpc_client,
            signature_receiver,
            transaction_sender,
            self.connection_config.clone(),
            commitment,
            cancellation_token.clone(),
            metrics.clone(),
        );

        let task_processor = task_processor(
            transaction_receiver,
            sender,
            id,
            filters,
            cancellation_token.clone(),
            metrics.clone(),
            self.connection_config.clone(),
        );

        tokio::spawn(async move {
            tokio::select! {
                _ = signature_fetcher => {},
                _ = transaction_fetcher => {},
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
fn signature_fetcher(
    rpc_client: Arc<RpcClient>,
    account: Pubkey,
    connection_config: ConnectionConfig,
    signature_sender: Sender<Signature>,
    filters: Filters,
    commitment: Option<CommitmentConfig>,
    cancellation_token: CancellationToken,
    metrics: Arc<MetricsCollection>,
) -> JoinHandle<()> {
    let rpc_client = Arc::clone(&rpc_client);
    let filters = filters.clone();
    let signature_sender = signature_sender.clone();

    tokio::spawn(async move {
        let mut last_fetched_signature = filters.before_signature;
        let mut until_signature = filters.until_signature;
        let mut most_recent_signature: Option<Signature> = None;
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    log::info!("Cancelling RPC Crawler signature fetcher...");
                    break;
                }
                _ = async {
                    let mut retries = 0;
                    let mut backoff = connection_config.retry_config.initial_backoff_ms;

                    loop {
                        match rpc_client.get_signatures_for_address_with_config(
                            &account,
                            GetConfirmedSignaturesForAddress2Config {
                                before: last_fetched_signature,
                                until: until_signature,
                                limit: Some(connection_config.batch_limit),
                                commitment: Some(commitment.unwrap_or(CommitmentConfig::confirmed())),
                            }
                        ).await {
                            Ok(signatures) => {
                                let start = Instant::now();

                                if signatures.is_empty() {
                                    // no more signatures to fetch, so we've gone through
                                    // all transactions that have been sent up until we started polling for signatures
                                    // update `last_fetched_signature` to None so we can detect newly sent transactions
                                    last_fetched_signature = None;
                                    if most_recent_signature.is_some() {
                                            // set the `until` signature to the most recent signature
                                            // this will prevent reindexing old transactions
                                            until_signature = most_recent_signature;
                                            // set the most recent signature to None
                                            // this will prevent reindexing old transactions
                                            // after we run out of new
                                            most_recent_signature = None;
                                    }
                                     tokio::time::sleep(connection_config.polling_interval).await;
                                    break;
                                }

                                // if we have not seen a signature, then update the most recent signature
                                // on subsequent loop's, this will prevent us from reindexing already seen transactions
                                if most_recent_signature.is_none() {
                                    match Signature::from_str(&signatures[0].signature) {
                                        Ok(sig) => most_recent_signature = Some(sig),
                                        Err(e) => {
                                            log::error!("Invalid signature: {:?}", e);
                                        }
                                    }
                                }

                                for sig_info in signatures.iter() {
                                    let signature = match Signature::from_str(&sig_info.signature) {
                                        Ok(sig) => sig,
                                        Err(e) => {
                                            log::error!("Invalid signature: {:?}", e);
                                            continue;
                                        }
                                    };

                                    if let Err(e) = signature_sender.send(signature).await {
                                        log::error!("Failed to send signature: {:?}", e);
                                        break;
                                    }
                                }

                                last_fetched_signature = signatures
                                    .last()
                                    .and_then(|s| Signature::from_str(&s.signature).ok());

                                let time_taken = start.elapsed().as_millis();

                                metrics.record_histogram("transaction_crawler_signatures_fetch_times_milliseconds", time_taken as f64)
                                    .await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                metrics.increment_counter("transaction_crawler_signatures_fetched", signatures.len() as u64)
                                    .await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                break;
                            }
                            Err(e) => {
                                if retries >= connection_config.retry_config.max_retries {
                                    log::error!("Failed to fetch signatures after {} retries: {:?}", retries, e);
                                    break;
                                }

                                log::warn!(
                                    "Failed to fetch signatures (attempt {}/{}), retrying in {}ms: {:?}",
                                    retries + 1,
                                    connection_config.retry_config.max_retries,
                                    backoff,
                                    e
                                );

                                tokio::time::sleep(Duration::from_millis(backoff)).await;
                                retries += 1;
                                backoff = (backoff as f64 * connection_config.retry_config.backoff_multiplier) as u64;
                                backoff = backoff.min(connection_config.retry_config.max_backoff_ms);
                            }
                        }
                    }
                } => {}
            }
        }
    })
}

fn transaction_fetcher(
    rpc_client: Arc<RpcClient>,
    signature_receiver: Receiver<Signature>,
    transaction_sender: Sender<(Signature, EncodedConfirmedTransactionWithStatusMeta)>,
    connection_config: ConnectionConfig,
    commitment: Option<CommitmentConfig>,
    cancellation_token: CancellationToken,
    metrics: Arc<MetricsCollection>,
) -> JoinHandle<()> {
    let rpc_client = Arc::clone(&rpc_client);
    let transaction_sender = transaction_sender.clone();
    let mut signature_receiver = signature_receiver;

    tokio::spawn(async move {
        let fetch_stream_task = async {
            let fetch_stream = async_stream::stream! {
                while let Some(signature) = signature_receiver.recv().await {
                    yield signature;
                }
            };

            fetch_stream
                .map(|signature| {
                    let metrics = metrics.clone();
                    let connection_config = connection_config.clone();
                    let rpc_client = Arc::clone(&rpc_client);
                    async move {
                        let start = Instant::now();
                        let mut retries = 0;
                        let mut backoff = connection_config.retry_config.initial_backoff_ms;

                        loop {
                            match rpc_client.get_transaction_with_config(
                                &signature,
                                RpcTransactionConfig {
                                    encoding: Some(UiTransactionEncoding::Base64),
                                    commitment: Some(
                                        commitment.unwrap_or(CommitmentConfig::confirmed()),
                                    ),
                                    max_supported_transaction_version: Some(0),
                                },
                            ).await {
                                Ok(tx) => {
                                    let time_taken = start.elapsed().as_millis();

                                    metrics
                                        .record_histogram(
                                            "transaction_crawler_transaction_fetch_times_milliseconds",
                                            time_taken as f64,
                                        )
                                        .await
                                        .expect("Error recording metric");

                                    return Some((signature, tx));
                                }
                                Err(e) => {
                                    if retries >= connection_config.retry_config.max_retries {
                                        log::error!("Failed to fetch transaction {} after {} retries: {:?}", signature, retries, e);
                                        return None;
                                    }

                                    log::warn!(
                                        "Failed to fetch transaction {} (attempt {}/{}), retrying in {}ms: {:?}",
                                        signature,
                                        retries + 1,
                                        connection_config.retry_config.max_retries,
                                        backoff,
                                        e
                                    );

                                    tokio::time::sleep(Duration::from_millis(backoff)).await;
                                    retries += 1;
                                    backoff = (backoff as f64 * connection_config.retry_config.backoff_multiplier) as u64;
                                    backoff = backoff.min(connection_config.retry_config.max_backoff_ms);
                                }
                            }
                        }
                    }
                })
                .buffer_unordered(connection_config.max_concurrent_requests)
                .for_each(|result| async {
                    metrics
                        .increment_counter("transaction_crawler_transactions_fetched", 1)
                        .await
                        .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                    if let Some((signature, fetched_transaction)) = result {
                        if let Err(e) = transaction_sender
                            .send((signature, fetched_transaction))
                            .await
                        {
                            log::error!("Failed to send transaction: {:?}", e);
                        }
                    }
                })
                .await;
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                log::info!("Cancelling RPC Crawler transaction fetcher...");
            }
            _ = fetch_stream_task => {}
        }
    })
}

fn task_processor(
    transaction_receiver: Receiver<(Signature, EncodedConfirmedTransactionWithStatusMeta)>,
    sender: Sender<(Update, DatasourceId)>,
    id: DatasourceId,
    filters: Filters,
    cancellation_token: CancellationToken,
    metrics: Arc<MetricsCollection>,
    connection_config: ConnectionConfig,
) -> JoinHandle<()> {
    let mut transaction_receiver = transaction_receiver;
    let sender = sender.clone();
    let id_for_loop = id.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    log::info!("Cancelling RPC Crawler task processor...");
                    break;
                }
                Some((signature, fetched_transaction)) = transaction_receiver.recv() => {
                    let start = Instant::now();
                    let transaction = fetched_transaction.transaction;

                    let meta_original = if let Some(meta) = transaction.clone().meta {
                        meta
                    } else {
                        log::warn!("Meta is malformed for transaction: {:?}", signature);
                        continue;
                    };

                    if meta_original.status.is_err() {
                        continue;
                    }

                    let Some(decoded_transaction) = transaction.transaction.decode() else {
                        log::error!("Failed to decode transaction: {:?}", transaction);
                        continue;
                    };

                    if let Some(accounts) = &filters.accounts {
                        let account_set: HashSet<Pubkey> = accounts.iter().cloned().collect();

                        let static_accounts = decoded_transaction.message.static_account_keys();

                        let loaded_addresses =
                            meta_original
                                .loaded_addresses
                                .clone()
                                .unwrap_or_else(|| UiLoadedAddresses {
                                    writable: vec![],
                                    readonly: vec![],
                                });

                        let all_accounts: HashSet<Pubkey> = static_accounts
                            .iter()
                            .cloned()
                            .chain(
                                loaded_addresses
                                    .writable
                                    .iter()
                                    .filter_map(|s| Pubkey::from_str(s).ok()),
                            )
                            .chain(
                                loaded_addresses
                                    .readonly
                                    .iter()
                                    .filter_map(|s| Pubkey::from_str(s).ok()),
                            )
                            .collect();

                        if !all_accounts
                            .iter()
                            .any(|account| account_set.contains(account))
                        {
                            continue;
                        }
                    }

                    let Ok(meta_needed) = transaction_metadata_from_original_meta(meta_original) else {
                        log::error!("Error getting metadata from transaction original meta.");
                        continue;
                    };

                    let update = Update::Transaction(Box::new(TransactionUpdate {
                        signature,
                        transaction: decoded_transaction.clone(),
                        meta: meta_needed,
                        is_vote: false,
                        slot: fetched_transaction.slot,
                        block_time: fetched_transaction.block_time,
                        block_hash: None,
                    }));


                    metrics
                            .record_histogram(
                                "transaction_crawler_transaction_process_time_milliseconds",
                                start.elapsed().as_millis() as f64
                            )
                            .await
                            .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));


                    if connection_config.blocking_send {
                        if let Err(e) = sender.send((update.clone(), id_for_loop.clone())).await {
                            log::warn!("Failed to send update: {:?}", e);
                            continue;
                        }
                    }
                    if !connection_config.blocking_send {
                        if let Err(e) = sender.try_send((update.clone(), id_for_loop.clone())) {
                            log::warn!("Failed to send update: {:?}", e);
                            continue;
                        }
                    }
                }
            }
        }
    })
}
