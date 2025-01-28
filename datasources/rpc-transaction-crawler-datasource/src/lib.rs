use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
    metrics::MetricsCollection,
    transformers::transaction_metadata_from_original_meta,
};
use futures::StreamExt;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
    rpc_config::RpcTransactionConfig,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, UiLoadedAddresses, UiTransactionEncoding,
};
use std::{collections::HashSet, str::FromStr, sync::Arc, time::Duration};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::JoinHandle,
    time::Instant,
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Filters {
    pub accounts: Option<Vec<Pubkey>>,
    pub before_signature: Option<Signature>,
    pub until_signature: Option<Signature>,
}

impl Filters {
    pub fn new(
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

pub struct RpcTransactionCrawler {
    pub rpc_url: String,
    pub account: Pubkey,
    pub batch_limit: usize,
    pub polling_interval: Duration,
    pub filters: Filters,
    pub commitment: Option<CommitmentConfig>,
    pub max_concurrent_requests: usize,
}

impl RpcTransactionCrawler {
    pub fn new(
        rpc_url: String,
        account: Pubkey,
        batch_limit: usize,
        polling_interval: Duration,
        filters: Filters,
        commitment: Option<CommitmentConfig>,
        max_concurrent_requests: usize,
    ) -> Self {
        RpcTransactionCrawler {
            rpc_url,
            account,
            batch_limit,
            polling_interval,
            filters,
            commitment,
            max_concurrent_requests,
        }
    }
}

#[async_trait]
impl Datasource for RpcTransactionCrawler {
    async fn consume(
        &self,
        sender: &mpsc::UnboundedSender<Update>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            self.rpc_url.clone(),
            self.commitment.unwrap_or(CommitmentConfig::confirmed()),
        ));
        let account = self.account;
        let batch_limit = self.batch_limit;
        let polling_interval = self.polling_interval;
        let filters = self.filters.clone();
        let sender = sender.clone();
        let commitment = self.commitment;
        let max_concurrent_requests = self.max_concurrent_requests;

        let (signature_sender, signature_receiver) = mpsc::channel(1000);
        let (transaction_sender, transaction_receiver) = mpsc::channel(1000);

        let signature_fetcher = signature_fetcher(
            rpc_client.clone(),
            account,
            batch_limit,
            polling_interval,
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
            commitment,
            max_concurrent_requests,
            cancellation_token.clone(),
            metrics.clone(),
        );

        let task_processor = task_processor(
            transaction_receiver,
            sender,
            filters,
            cancellation_token.clone(),
            metrics.clone(),
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

fn signature_fetcher(
    rpc_client: Arc<RpcClient>,
    account: Pubkey,
    batch_limit: usize,
    polling_interval: Duration,
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

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    log::info!("Cancelling RPC Crawler signature fetcher...");
                    break;
                }
                result = rpc_client.get_signatures_for_address_with_config(
                    &account,
                    GetConfirmedSignaturesForAddress2Config {
                        before: last_fetched_signature,
                        until: filters.until_signature,
                        limit: Some(batch_limit),
                        commitment: Some(commitment.unwrap_or(CommitmentConfig::confirmed())),
                    }
                ) => {
                    match result {
                        Ok(signatures) => {
                            let start = Instant::now();

                            if signatures.is_empty() {
                                tokio::time::sleep(polling_interval).await;
                                continue;
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

                            metrics.record_histogram("transaction_crawler_signatures_fetch_times_milliseconds",   time_taken as f64)
                                .await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                            metrics.increment_counter("transaction_crawler_signatures_fetched", signatures.len() as u64).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                        }
                        Err(e) => {
                            log::error!("Error fetching signatures: {:?}", e);
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
            }
        }
    })
}

fn transaction_fetcher(
    rpc_client: Arc<RpcClient>,
    signature_receiver: Receiver<Signature>,
    transaction_sender: Sender<(Signature, EncodedConfirmedTransactionWithStatusMeta)>,
    commitment: Option<CommitmentConfig>,
    max_concurrent_requests: usize,
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
                    let rpc_client = Arc::clone(&rpc_client);
                    let metrics = metrics.clone();
                    async move {
                        let start = Instant::now();

                        match rpc_client
                            .get_transaction_with_config(
                                &signature,
                                RpcTransactionConfig {
                                    encoding: Some(UiTransactionEncoding::Base64),
                                    commitment: Some(
                                        commitment.unwrap_or(CommitmentConfig::confirmed()),
                                    ),
                                    max_supported_transaction_version: Some(0),
                                },
                            )
                            .await
                        {
                            Ok(tx) => {
                                let time_taken = start.elapsed().as_millis();

                                metrics
                                    .record_histogram(
                                        "transaction_crawler_transaction_fetch_times_milliseconds",
                                        time_taken as f64,
                                    )
                                    .await
                                    .unwrap();

                                Some((signature, tx))
                            }
                            Err(e) => {
                                log::error!("Error fetching transaction {}: {:?}", signature, e);
                                None
                            }
                        }
                    }
                })
                .buffer_unordered(max_concurrent_requests)
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
    sender: mpsc::UnboundedSender<Update>,
    filters: Filters,
    cancellation_token: CancellationToken,
    metrics: Arc<MetricsCollection>,
) -> JoinHandle<()> {
    let mut transaction_receiver = transaction_receiver;
    let sender = sender.clone();

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

                    let update = Update::Transaction(TransactionUpdate {
                        signature,
                        transaction: decoded_transaction.clone(),
                        meta: meta_needed,
                        is_vote: false,
                        slot: fetched_transaction.slot,
                    });


                    metrics
                            .record_histogram(
                                "transaction_crawler_transaction_process_time_milliseconds",
                                start.elapsed().as_millis() as f64
                            )
                            .await
                            .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));


                    if let Err(e) = sender.send(update) {
                        log::error!("Failed to send update: {:?}", e);
                        continue;
                    }
                }
            }
        }
    })
}
