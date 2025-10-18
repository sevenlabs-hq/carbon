use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{
            AccountDeletion, AccountUpdate, Datasource, DatasourceId, TransactionUpdate, Update,
            UpdateType,
        },
        error::CarbonResult,
        metrics::MetricsCollection,
    },
    futures::{sink::SinkExt, StreamExt},
    solana_account::Account,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::{
        collections::{HashMap, HashSet},
        convert::TryFrom,
        sync::Arc,
        time::Duration,
    },
    tokio::sync::{mpsc::Sender, RwLock},
    tokio_util::sync::CancellationToken,
    yellowstone_grpc_client::{GeyserGrpcBuilder, GeyserGrpcBuilderResult, GeyserGrpcClient},
    yellowstone_grpc_proto::{
        convert_from::{create_tx_meta, create_tx_versioned},
        geyser::{
            subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
            SubscribeRequestFilterAccounts, SubscribeRequestFilterBlocks,
            SubscribeRequestFilterTransactions, SubscribeRequestPing, SubscribeUpdateAccountInfo,
            SubscribeUpdateTransactionInfo,
        },
        tonic::{codec::CompressionEncoding, transport::ClientTlsConfig},
    },
};

#[derive(Debug)]
pub struct YellowstoneGrpcGeyserClient {
    pub endpoint: String,
    pub x_token: Option<String>,
    pub commitment: Option<CommitmentLevel>,
    pub account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
    pub transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
    pub block_filters: BlockFilters,
    pub account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
    pub geyser_config: YellowstoneGrpcClientConfig,
}

#[derive(Debug, Clone)]
pub struct YellowstoneGrpcClientConfig {
    pub compression: Option<CompressionEncoding>,
    pub connect_timeout: Option<Duration>,
    pub timeout: Option<Duration>,
    pub max_decoding_message_size: Option<usize>,
    pub tls_config: Option<ClientTlsConfig>,
    pub tcp_nodelay: Option<bool>,
}

impl Default for YellowstoneGrpcClientConfig {
    fn default() -> Self {
        Self {
            compression: None,
            connect_timeout: Some(Duration::from_secs(15)),
            timeout: Some(Duration::from_secs(15)),
            max_decoding_message_size: None,
            tls_config: None,
            tcp_nodelay: None,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BlockFilters {
    pub filters: HashMap<String, SubscribeRequestFilterBlocks>,
    pub failed_transactions: Option<bool>,
}

impl YellowstoneGrpcGeyserClient {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        endpoint: String,
        x_token: Option<String>,
        commitment: Option<CommitmentLevel>,
        account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
        transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
        block_filters: BlockFilters,
        account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
        geyser_config: YellowstoneGrpcClientConfig,
    ) -> Self {
        YellowstoneGrpcGeyserClient {
            endpoint,
            x_token,
            commitment,
            account_filters,
            transaction_filters,
            block_filters,
            account_deletions_tracked,
            geyser_config,
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
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let endpoint = self.endpoint.clone();
        let x_token = self.x_token.clone();
        let commitment = self.commitment;
        let account_filters = self.account_filters.clone();
        let transaction_filters = self.transaction_filters.clone();
        let account_deletions_tracked = self.account_deletions_tracked.clone();
        let BlockFilters {
            filters,
            failed_transactions: block_failed_transactions,
        } = self.block_filters.clone();
        let retain_block_failed_transactions = block_failed_transactions.unwrap_or(true);

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

        tokio::spawn(async move {
            let subscribe_request = SubscribeRequest {
                slots: HashMap::new(),
                accounts: account_filters,
                transactions: transaction_filters,
                transactions_status: HashMap::new(),
                entry: HashMap::new(),
                blocks: filters,
                blocks_meta: HashMap::new(),
                commitment: commitment.map(|x| x as i32),
                accounts_data_slice: vec![],
                ping: None,
                from_slot: None,
            };

            let id_for_loop = id.clone();

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancelling Yellowstone gRPC subscription.");
                        break;
                    }
                    result = geyser_client.subscribe_with_request(Some(subscribe_request.clone())) => {
                        match result {
                            Ok((mut subscribe_tx, mut stream)) => {
                                while let Some(message) = stream.next().await {
                                    if cancellation_token.is_cancelled() {
                                        break;
                                    }

                                    match message {
                                        Ok(msg) => match msg.update_oneof {
                                            Some(UpdateOneof::Account(account_update)) => {
                                                send_subscribe_account_update_info(
                                                    account_update.account,
                                                    &metrics,
                                                    &sender,
                                                    id_for_loop.clone(),
                                                    account_update.slot,
                                                    &account_deletions_tracked,
                                                )
                                                .await
                                            }

                                            Some(UpdateOneof::Transaction(transaction_update)) => {
                                                send_subscribe_update_transaction_info(transaction_update.transaction, &metrics, &sender, id_for_loop.clone(), transaction_update.slot, None).await
                                            }
                                            Some(UpdateOneof::Block(block_update)) => {
                                                let block_time = block_update.block_time.map(|ts| ts.timestamp);

                                                for transaction_update in block_update.transactions {
                                                    if retain_block_failed_transactions || transaction_update.meta.as_ref().map(|meta| meta.err.is_none()).unwrap_or(false) {
                                                        send_subscribe_update_transaction_info(Some(transaction_update), &metrics, &sender, id_for_loop.clone(), block_update.slot, block_time).await
                                                    }
                                                }

                                                for account_info in block_update.accounts {
                                                    send_subscribe_account_update_info(
                                                        Some(account_info),
                                                        &metrics,
                                                        &sender,
                                                        id_for_loop.clone(),
                                                        block_update.slot,
                                                        &account_deletions_tracked,
                                                    )
                                                    .await;
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
                                        },
                                        Err(error) => {
                                            log::error!("Geyser stream error: {error:?}");
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to subscribe: {:?}", e);
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

async fn send_subscribe_account_update_info(
    account_update_info: Option<SubscribeUpdateAccountInfo>,
    metrics: &MetricsCollection,
    sender: &Sender<(Update, DatasourceId)>,
    id: DatasourceId,
    slot: u64,
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
            && account_owner_pubkey == solana_program::system_program::ID
        {
            let accounts = account_deletions_tracked.read().await;
            if accounts.contains(&account_pubkey) {
                let account_deletion = AccountDeletion {
                    pubkey: account_pubkey,
                    slot,
                    transaction_signature: account_info
                        .txn_signature
                        .and_then(|sig| Signature::try_from(sig).ok()),
                };
                if let Err(e) = sender.try_send((Update::AccountDeletion(account_deletion), id)) {
                    log::error!(
                        "Failed to send account deletion update for pubkey {:?} at slot {}: {:?}",
                        account_pubkey,
                        slot,
                        e
                    );
                }
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

            if let Err(e) = sender.try_send((update, id)) {
                log::error!(
                    "Failed to send account update for pubkey {:?} at slot {}: {:?}",
                    account_pubkey,
                    slot,
                    e
                );
            }
        }

        metrics
            .record_histogram(
                "yellowstone_grpc_account_process_time_nanoseconds",
                start_time.elapsed().as_nanos() as f64,
            )
            .await
            .expect("Failed to record histogram");

        metrics
            .increment_counter("yellowstone_grpc_account_updates_received", 1)
            .await
            .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));
    } else {
        log::error!("No account info in UpdateOneof::Account at slot {}", slot);
    }
}

async fn send_subscribe_update_transaction_info(
    transaction_info: Option<SubscribeUpdateTransactionInfo>,
    metrics: &MetricsCollection,
    sender: &Sender<(Update, DatasourceId)>,
    id: DatasourceId,
    slot: u64,
    block_time: Option<i64>,
) {
    let start_time = std::time::Instant::now();

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
                log::error!("Failed to create transaction meta: {:?}", err);
                return;
            }
        };
        let update = Update::Transaction(Box::new(TransactionUpdate {
            signature,
            transaction: versioned_transaction,
            meta: meta_original,
            is_vote: transaction_info.is_vote,
            slot,
            block_time,
            block_hash: None,
        }));
        if let Err(e) = sender.try_send((update, id)) {
            log::error!(
                "Failed to send transaction update with signature {:?} at slot {}: {:?}",
                signature,
                slot,
                e
            );
            return;
        }

        metrics
            .record_histogram(
                "yellowstone_grpc_transaction_process_time_nanoseconds",
                start_time.elapsed().as_nanos() as f64,
            )
            .await
            .expect("Failed to record histogram");

        metrics
            .increment_counter("yellowstone_grpc_transaction_updates_received", 1)
            .await
            .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));
    } else {
        log::error!(
            "No transaction info in `UpdateOneof::Transaction` at slot {}",
            slot
        );
    }
}
