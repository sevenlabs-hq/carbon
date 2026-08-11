#[cfg(feature = "batch")]
use carbon_core::datasource::BatchUpdateId;
use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{
            AccountDeletion, AccountUpdate, Datasource, DatasourceDisconnection, DatasourceId,
            TransactionUpdate, Update, UpdateType,
        },
        error::CarbonResult,
        metrics::{Counter, Histogram, MetricsRegistry},
    },
    chrono::{DateTime, Utc},
    futures::{sink::SinkExt, StreamExt},
    solana_account::Account,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::{
        collections::{HashMap, HashSet},
        convert::TryFrom,
        sync::{Arc, LazyLock},
        time::Duration,
    },
    tokio::sync::{
        mpsc::{self, Sender},
        Mutex, RwLock,
    },
    tokio_util::sync::CancellationToken,
    yellowstone_grpc_client::{GeyserGrpcBuilder, GeyserGrpcBuilderResult, GeyserGrpcClient},
    yellowstone_grpc_convert::convert_from::{create_tx_meta, create_tx_versioned},
    yellowstone_grpc_proto::{
        geyser::{
            subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterBlocks,
            SubscribeRequestPing, SubscribeUpdateAccountInfo, SubscribeUpdateTransactionInfo,
        },
        tonic::{codec::CompressionEncoding, transport::ClientTlsConfig},
    },
};

static ACCOUNT_PROCESS_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "yellowstone_grpc_account_process_time_nanoseconds",
        "Time taken to process account updates in nanoseconds",
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
static ACCOUNT_UPDATES_RECEIVED: Counter = Counter::new(
    "yellowstone_grpc_account_updates_received_total",
    "Total account updates received from Yellowstone gRPC",
);
static TRANSACTION_PROCESS_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "yellowstone_grpc_transaction_process_time_nanoseconds",
        "Time taken to process transaction updates in nanoseconds",
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
static TRANSACTION_UPDATES_RECEIVED: Counter = Counter::new(
    "yellowstone_grpc_transaction_updates_received_total",
    "Total transaction updates received from Yellowstone gRPC",
);

fn register_yellowstone_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&ACCOUNT_UPDATES_RECEIVED);
    registry.register_counter(&TRANSACTION_UPDATES_RECEIVED);
    registry.register_histogram(&ACCOUNT_PROCESS_TIME_NANOS);
    registry.register_histogram(&TRANSACTION_PROCESS_TIME_NANOS);
}

/// Default timeout for detecting stale connections (30 seconds)
pub const DEFAULT_STREAM_TIMEOUT_SECS: u64 = 30;

#[derive(Debug)]
pub struct YellowstoneGrpcGeyserClient {
    endpoint: String,
    x_token: Option<String>,
    geyser_config: YellowstoneGrpcClientConfig,
    initial_subscribe_request: SubscribeRequest,
    retain_block_failed_transactions: Option<bool>,
    account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
    disconnect_notifier: Option<mpsc::Sender<DatasourceDisconnection>>,
    /// Timeout for detecting hung/stale connections. Default: 30 seconds.
    stream_timeout: Duration,
    pub reconnect_notifier: mpsc::Sender<SubscribeRequest>,
    reconnect_receiver: Arc<Mutex<Option<mpsc::Receiver<SubscribeRequest>>>>,
}

impl Clone for YellowstoneGrpcGeyserClient {
    fn clone(&self) -> Self {
        YellowstoneGrpcGeyserClient {
            endpoint: self.endpoint.clone(),
            x_token: self.x_token.clone(),
            geyser_config: self.geyser_config.clone(),
            initial_subscribe_request: self.initial_subscribe_request.clone(),
            retain_block_failed_transactions: self.retain_block_failed_transactions,
            account_deletions_tracked: self.account_deletions_tracked.clone(),
            disconnect_notifier: self.disconnect_notifier.clone(),
            stream_timeout: self.stream_timeout,
            reconnect_notifier: self.reconnect_notifier.clone(),
            reconnect_receiver: self.reconnect_receiver.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct YellowstoneGrpcClientConfig {
    pub timeout: Option<Duration>,
    pub tcp_nodelay: Option<bool>,
    pub connect_timeout: Option<Duration>,
    pub tls_config: Option<ClientTlsConfig>,
    pub compression: Option<CompressionEncoding>,
    pub max_decoding_message_size: Option<usize>,
}

impl Default for YellowstoneGrpcClientConfig {
    fn default() -> Self {
        Self {
            tls_config: None,
            tcp_nodelay: None,
            compression: None,
            max_decoding_message_size: None,
            timeout: Some(Duration::from_secs(15)),
            connect_timeout: Some(Duration::from_secs(15)),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BlockFilters {
    pub failed_transactions: Option<bool>,
    pub filters: HashMap<String, SubscribeRequestFilterBlocks>,
}

fn drain_latest_with(
    rx: &mut mpsc::Receiver<SubscribeRequest>,
    initial: SubscribeRequest,
) -> SubscribeRequest {
    let mut latest = initial;

    while let Ok(request) = rx.try_recv() {
        latest = request;
    }

    latest
}

impl YellowstoneGrpcGeyserClient {
    /// Creates a new YellowstoneGrpcGeyserClient with optional stream timeout.
    /// If `stream_timeout` is None, defaults to 30 seconds.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        endpoint: String,
        x_token: Option<String>,
        initial_subscribe_request: SubscribeRequest,
        account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
        geyser_config: YellowstoneGrpcClientConfig,
        retain_block_failed_transactions: Option<bool>,
        disconnect_notifier: Option<mpsc::Sender<DatasourceDisconnection>>,
        stream_timeout: Option<Duration>,
    ) -> Self {
        let (reconnect_notifier, reconnect_receiver) = mpsc::channel(1);

        YellowstoneGrpcGeyserClient {
            x_token,
            endpoint,
            geyser_config,
            reconnect_notifier,
            disconnect_notifier,
            account_deletions_tracked,
            initial_subscribe_request,
            retain_block_failed_transactions,
            reconnect_receiver: Arc::new(Mutex::new(Some(reconnect_receiver))),
            stream_timeout: stream_timeout
                .unwrap_or(Duration::from_secs(DEFAULT_STREAM_TIMEOUT_SECS)),
        }
    }
}

impl YellowstoneGrpcClientConfig {
    pub const fn new(
        compression: Option<CompressionEncoding>,
        connect_timeout: Option<Duration>,
        timeout: Option<Duration>,
        max_decoding_message_size: Option<usize>,
        tls_config: Option<ClientTlsConfig>,
        tcp_nodelay: Option<bool>,
    ) -> Self {
        YellowstoneGrpcClientConfig {
            compression,
            connect_timeout,
            timeout,
            max_decoding_message_size,
            tls_config,
            tcp_nodelay,
        }
    }

    pub fn geyser_config_builder(
        &self,
        mut builder: GeyserGrpcBuilder,
    ) -> GeyserGrpcBuilderResult<GeyserGrpcBuilder> {
        builder = builder.connect_timeout(self.connect_timeout.unwrap_or(Duration::from_secs(15)));

        builder = builder.timeout(self.timeout.unwrap_or(Duration::from_secs(15)));
        let tls = self
            .tls_config
            .clone()
            .unwrap_or_else(|| ClientTlsConfig::new().with_enabled_roots());
        builder = builder.tls_config(tls)?;

        if let Some(compression) = self.compression {
            builder = builder
                .send_compressed(compression)
                .accept_compressed(compression);
        }
        if let Some(val) = self.max_decoding_message_size {
            builder = builder.max_decoding_message_size(val);
        }

        if let Some(val) = self.tcp_nodelay {
            builder = builder.tcp_nodelay(val);
        }
        Ok(builder)
    }
}

#[async_trait]
impl Datasource for YellowstoneGrpcGeyserClient {
    async fn consume(
        &self,
        id: DatasourceId,
        #[cfg(feature = "batch")] sender: Sender<((BatchUpdateId, Vec<Update>), DatasourceId)>,
        #[cfg(not(feature = "batch"))] sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        register_yellowstone_metrics();
        let x_token = self.x_token.clone();
        let endpoint = self.endpoint.clone();
        let account_deletions_tracked = self.account_deletions_tracked.clone();
        let retain_block_failed_transactions =
            self.retain_block_failed_transactions.unwrap_or(true);

        let builder = GeyserGrpcClient::build_from_shared(endpoint)
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?
            .x_token(x_token)
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?;

        let mut geyser_client = self
            .geyser_config
            .geyser_config_builder(builder)
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?
            .connect()
            .await
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?;

        let stream_timeout = self.stream_timeout;
        let disconnect_tx_clone = self.disconnect_notifier.clone();

        self.reconnect_notifier
            .send(self.initial_subscribe_request.clone())
            .await
            .unwrap();

        let reconnect_receiver = self
            .reconnect_receiver
            .lock()
            .await
            .take()
            .expect("reconnect_receiver should only be taken once");

        tokio::spawn(async move {
            let mut reconnect_receiver = reconnect_receiver;

            let mut last_disconnect_time: Option<DateTime<Utc>> = None;
            let mut last_slot_before_disconnect: Option<u64> = None;
            let mut last_processed_slot: u64 = 0;

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancelling Yellowstone gRPC subscription.");
                        break;
                    }
                    request = reconnect_receiver.recv() => {
                        match request {
                            None => {}
                            Some(new_request) => {
                                let subscribe_request = drain_latest_with(&mut reconnect_receiver, new_request);

                                match geyser_client.subscribe_with_request(Some(subscribe_request.clone())).await {
                                        Ok((mut subscribe_tx, mut stream)) => {
                                            let mut first_message_after_reconnect = last_disconnect_time.is_some();

                                            loop {
                                                if cancellation_token.is_cancelled() {
                                                    break;
                                                }

                                                let message_result = tokio::time::timeout(
                                                    stream_timeout,
                                                    stream.next()
                                                ).await;

                                                let message = match message_result {
                                                    Ok(Some(msg)) => msg,
                                                    Ok(None) => {
                                                        log::warn!("Stream closed");
                                                        if last_disconnect_time.is_none() {
                                                            last_disconnect_time = Some(Utc::now());
                                                            last_slot_before_disconnect = Some(last_processed_slot);
                                                            log::warn!("Disconnected at slot {last_processed_slot}");
                                                        }
                                                        break;
                                                    }
                                                    Err(_) => {
                                                        log::warn!("Stream timeout - no messages for {stream_timeout:?}");
                                                        if last_disconnect_time.is_none() {
                                                            last_disconnect_time = Some(Utc::now());
                                                            last_slot_before_disconnect = Some(last_processed_slot);
                                                            log::warn!("Disconnected at slot {last_processed_slot} (timeout)");
                                                        }
                                                        break;
                                                    }
                                                };

                                                match message {
                                                    Ok(msg) => {
                                                        let mut updates: Vec<Update> = vec![];

                                                        if first_message_after_reconnect {
                                                            let current_slot = match &msg.update_oneof {
                                                                Some(UpdateOneof::Account(ref update)) => Some(update.slot),
                                                                Some(UpdateOneof::Transaction(ref update)) => Some(update.slot),
                                                                Some(UpdateOneof::Block(ref update)) => Some(update.slot),
                                                                _ => None,
                                                            };

                                                            if let Some(slot) = current_slot {
                                                                first_message_after_reconnect = false;

                                                                if let (Some(disconnect_time), Some(last_slot)) =
                                                                    (last_disconnect_time.take(), last_slot_before_disconnect.take())
                                                                {
                                                                    let missed = slot.saturating_sub(last_slot);

                                                                    let disconnection = DatasourceDisconnection {
                                                                        source: "yellowstone-grpc".to_string(),
                                                                        disconnect_time,
                                                                        last_slot_before_disconnect: last_slot,
                                                                        first_slot_after_reconnect: slot,
                                                                        missed_slots: missed,
                                                                    };

                                                                    if let Some(tx) = &disconnect_tx_clone {
                                                                        let _ = tx.try_send(disconnection);
                                                                    }

                                                                    log::info!("Reconnected. Slots: {last_slot} -> {slot} (missed: {missed})");
                                                                }
                                                            }
                                                        }

                                                        match msg.update_oneof {
                                                            Some(UpdateOneof::Account(account_update)) => {
                                                                last_processed_slot = account_update.slot;
                                                                collect_subscribe_account_update_info(
                                                                    account_update.account,
                                                                    account_update.slot,
                                                                    &mut updates,
                                                                    &account_deletions_tracked,
                                                                ).await
                                                            }

                                                            Some(UpdateOneof::Transaction(transaction_update)) => {
                                                                last_processed_slot = transaction_update.slot;
                                                                collect_subscribe_update_transaction_info(
                                                                    transaction_update.transaction,
                                                                    transaction_update.slot,
                                                                    None,
                                                                    &mut updates,
                                                                )
                                                            }
                                                            Some(UpdateOneof::Block(block_update)) => {
                                                                last_processed_slot = block_update.slot;
                                                                let block_time = block_update.block_time.map(|ts| ts.timestamp);

                                                                for transaction_update in block_update.transactions {
                                                                    if retain_block_failed_transactions || transaction_update.meta.as_ref().map(|meta| meta.err.is_none()).unwrap_or(false) {
                                                                        collect_subscribe_update_transaction_info(
                                                                            Some(transaction_update),
                                                                            block_update.slot,
                                                                            block_time,
                                                                            &mut updates,
                                                                        )
                                                                    }
                                                                }

                                                                for account_info in block_update.accounts {
                                                                    collect_subscribe_account_update_info(
                                                                        Some(account_info),
                                                                        block_update.slot,
                                                                        &mut updates,
                                                                        &account_deletions_tracked,
                                                                    ).await
                                                                }
                                                            }

                                                            Some(UpdateOneof::Ping(_)) => {
                                                                match subscribe_tx
                                                                    .send(SubscribeRequest {
                                                                        ping: Some(SubscribeRequestPing { id: 1 }),
                                                                        ..Default::default()
                                                                    })
                                                                    .await {
                                                                        Ok(()) => (),
                                                                        Err(error) => {
                                                                            log::error!("Failed to send ping error: {error:?}");
                                                                            break;
                                                                        },
                                                                    }
                                                            }

                                                            _ => {}
                                                        }

                                                       if !updates.is_empty() {
                                                           #[cfg(feature = "batch")]
                                                           match sender.try_send(((BatchUpdateId::new_unique(), updates), id.clone())) {
                                                               Ok(()) => {},
                                                               Err(error) => {
                                                                   log::error!("Failed to send update slot {last_processed_slot}: {error:?}");
                                                               },
                                                           }

                                                           #[cfg(not(feature =  "batch"))]
                                                           for update in updates {
                                                               match sender.try_send((update, id.clone())) {
                                                                   Ok(()) => {},
                                                                   Err(error) => {
                                                                       log::error!("Failed to send update slot {last_processed_slot}: {error:?}");
                                                                   },
                                                               }
                                                           }
                                                       }
                                                    }
                                                    Err(error) => {
                                                        log::error!("Geyser stream error: {error:?}");

                                                        if last_disconnect_time.is_none() {
                                                            last_disconnect_time = Some(Utc::now());
                                                            last_slot_before_disconnect = Some(last_processed_slot);
                                                            log::error!("Disconnected at slot {last_processed_slot}");
                                                        }

                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            log::error!("Failed to subscribe: {e:?}");

                                            if last_disconnect_time.is_none() {
                                                last_disconnect_time = Some(Utc::now());
                                                last_slot_before_disconnect = Some(last_processed_slot);
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![
            UpdateType::AccountUpdate,
            UpdateType::Transaction,
            UpdateType::AccountDeletion,
        ]
    }
}

pub async fn collect_subscribe_account_update_info(
    account_update_info: Option<SubscribeUpdateAccountInfo>,
    slot: u64,
    updates: &mut Vec<Update>,
    account_deletions_tracked: &RwLock<HashSet<Pubkey>>,
) {
    let start_time = std::time::Instant::now();

    if let Some(account_info) = account_update_info {
        let Ok(account_pubkey) = Pubkey::try_from(account_info.pubkey) else {
            return;
        };

        let Ok(account_owner_pubkey) = Pubkey::try_from(account_info.owner) else {
            return;
        };

        let account = Account {
            lamports: account_info.lamports,
            data: account_info.data,
            owner: account_owner_pubkey,
            executable: account_info.executable,
            rent_epoch: account_info.rent_epoch,
        };

        if account.lamports == 0
            && account.data.is_empty()
            && account_owner_pubkey == solana_system_interface::program::ID
        {
            let accounts = account_deletions_tracked.read().await;
            if accounts.contains(&account_pubkey) {
                let update = Update::AccountDeletion(AccountDeletion {
                    pubkey: account_pubkey,
                    slot,
                    transaction_signature: account_info
                        .txn_signature
                        .and_then(|sig| Signature::try_from(sig).ok()),
                });

                updates.push(update);
            }
        } else {
            let update = Update::Account(AccountUpdate {
                pubkey: account_pubkey,
                account,
                slot,
                transaction_signature: account_info
                    .txn_signature
                    .and_then(|sig| Signature::try_from(sig).ok()),
            });

            updates.push(update);
        }

        ACCOUNT_PROCESS_TIME_NANOS.record(start_time.elapsed().as_nanos() as f64);
        ACCOUNT_UPDATES_RECEIVED.inc();
    } else {
        log::error!("No account info in UpdateOneof::Account at slot {slot}");
    }
}

pub fn collect_subscribe_update_transaction_info(
    transaction_info: Option<SubscribeUpdateTransactionInfo>,
    slot: u64,
    block_time: Option<i64>,
    updates: &mut Vec<Update>,
) {
    if let Some(transaction_info) = transaction_info {
        let Ok(signature) = Signature::try_from(transaction_info.signature) else {
            return;
        };
        let Some(yellowstone_transaction) = transaction_info.transaction else {
            return;
        };
        let Some(yellowstone_tx_meta) = transaction_info.meta else {
            return;
        };
        let Ok(versioned_transaction) = create_tx_versioned(yellowstone_transaction) else {
            return;
        };
        let meta_original = match create_tx_meta(yellowstone_tx_meta) {
            Ok(meta) => meta,
            Err(err) => {
                log::error!("Failed to create transaction meta: {err:?}");
                return;
            }
        };
        let update = Update::Transaction(Box::new(TransactionUpdate {
            signature,
            transaction: versioned_transaction,
            meta: meta_original,
            is_vote: transaction_info.is_vote,
            slot,
            index: Some(transaction_info.index),
            block_time,
            block_hash: None,
        }));

        updates.push(update);
        TRANSACTION_UPDATES_RECEIVED.inc();
    } else {
        log::error!("No transaction info in `UpdateOneof::Transaction` at slot {slot}");
    }
}
