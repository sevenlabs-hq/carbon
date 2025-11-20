use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{
            AccountDeletion, AccountUpdate, Datasource, DatasourceId, TransactionUpdate, Update,
            UpdateType,
        },
        error::CarbonResult,
        metrics::{Counter, Histogram, MetricsRegistry},
    },
    log::{error, warn},
    solana_pubkey::Pubkey,
    std::{
        collections::HashSet,
        sync::{Arc, OnceLock},
    },
    tokio::{
        select,
        sync::{
            mpsc::{Receiver, Sender},
            RwLock,
        },
    },
    tokio_util::sync::CancellationToken,
};

static ACCOUNT_PROCESS_TIME_NANOS: OnceLock<Histogram> = OnceLock::new();
static ACCOUNT_UPDATES_RECEIVED: Counter = Counter::new(
    "agave_grpc_account_updates_received",
    "Total account updates received from stream message datasource",
);
static TRANSACTION_PROCESS_TIME_NANOS: OnceLock<Histogram> = OnceLock::new();
static TRANSACTION_UPDATES_RECEIVED: Counter = Counter::new(
    "agave_grpc_transaction_updates_received",
    "Total transaction updates received from stream message datasource",
);

fn init_histograms() {
    ACCOUNT_PROCESS_TIME_NANOS.get_or_init(|| {
        Histogram::new(
            "agave_grpc_account_process_time_nanoseconds",
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
    TRANSACTION_PROCESS_TIME_NANOS.get_or_init(|| {
        Histogram::new(
            "agave_grpc_transaction_process_time_nanoseconds",
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
}

fn register_stream_message_metrics() {
    init_histograms();
    let registry = MetricsRegistry::global();
    registry.register_counter(&ACCOUNT_UPDATES_RECEIVED);
    registry.register_counter(&TRANSACTION_UPDATES_RECEIVED);
    registry.register_histogram(ACCOUNT_PROCESS_TIME_NANOS.get().unwrap());
    registry.register_histogram(TRANSACTION_PROCESS_TIME_NANOS.get().unwrap());
}

pub enum UnifiedMessage {
    Account(AccountUpdate),
    Transaction(Box<TransactionUpdate>),
}

#[derive(Debug)]
pub struct StreamMessageClient {
    receiver: std::sync::Mutex<Option<Receiver<UnifiedMessage>>>,
    account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
}

impl StreamMessageClient {
    pub fn new(
        account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
        receiver: Receiver<UnifiedMessage>,
    ) -> Self {
        Self {
            receiver: std::sync::Mutex::new(Some(receiver)),
            account_deletions_tracked,
        }
    }
}

#[async_trait]
impl Datasource for StreamMessageClient {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        register_stream_message_metrics();
        let mut receiver_lock = self.receiver.lock().unwrap();
        let Some(receiver) = receiver_lock.take() else {
            error!("StreamMessageClient.consume() called more than once; receiver already taken");
            return Ok(());
        };
        drop(receiver_lock);

        let account_deletions_tracked = Arc::clone(&self.account_deletions_tracked);

        let id = id.clone();

        tokio::spawn(async move {
            handle_message_stream(
                receiver,
                cancellation_token,
                sender,
                &account_deletions_tracked,
                id,
            )
            .await;
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

pub async fn handle_message_stream(
    mut receiver: Receiver<UnifiedMessage>,
    cancellation_token: CancellationToken,
    sender: Sender<(Update, DatasourceId)>,
    account_deletions_tracked: &RwLock<HashSet<Pubkey>>,
    id: DatasourceId,
) {
    while !cancellation_token.is_cancelled() {
        select! {
            _ = cancellation_token.cancelled() => {
                break;
            }

            maybe_msg = receiver.recv() => {
                match maybe_msg {
                    Some(msg) => {
                        match msg {
                            UnifiedMessage::Account(account_info) => {
                               send_subscribe_account_update_info(
                                    account_info,
                                    &sender,
                                    account_deletions_tracked,
                                    id.clone()
                                ).await;
                            }

                            UnifiedMessage::Transaction(transaction_update) => {
                                send_subscribe_update_transaction_info(
                                    transaction_update,
                                    &sender,
                                    id.clone()
                                ).await;
                            }
                        }
                    }

                    None => {
                        warn!("Receiver closed");
                        break;
                    }
                }
            }
        }
    }

    log::info!("Geyser message loop exited");
}

async fn send_subscribe_account_update_info(
    account_update: AccountUpdate,
    sender: &Sender<(Update, DatasourceId)>,
    account_deletions_tracked: &RwLock<HashSet<Pubkey>>,
    id: DatasourceId,
) {
    let start_time = std::time::Instant::now();
    let account = &account_update.account;
    let account_pubkey = account_update.pubkey;

    if account.lamports == 0
        && account.data.is_empty()
        && account.owner == solana_system_interface::program::ID
    {
        let accounts = account_deletions_tracked.read().await;
        if accounts.contains(&account_pubkey) {
            let account_deletion = AccountDeletion {
                pubkey: account_pubkey,
                slot: account_update.slot,
                transaction_signature: account_update.transaction_signature,
            };
            if let Err(e) = sender
                .send((Update::AccountDeletion(account_deletion), id.clone()))
                .await
            {
                log::error!(
                    "Failed to send account deletion update for pubkey {:?}, sender capacity {:?} / max_capacity: {:?} : {:?}",
                    account_pubkey,
                    sender.capacity(),
                    sender.max_capacity(),
                    e
                );
            }
        }
    } else if let Err(e) = sender
        .send((Update::Account(account_update), id.clone()))
        .await
    {
        log::error!(
            "Failed to send account update for pubkey {:?},  sender capacity {:?} / max_capacity: {:?} : {:?},",
            account_pubkey,
            sender.capacity(),
            sender.max_capacity(),
            e
        );
    }

    ACCOUNT_PROCESS_TIME_NANOS
        .get()
        .unwrap()
        .record(start_time.elapsed().as_nanos() as f64);
    ACCOUNT_UPDATES_RECEIVED.inc();
}

async fn send_subscribe_update_transaction_info(
    transaction_info: Box<TransactionUpdate>,
    sender: &Sender<(Update, DatasourceId)>,
    id: DatasourceId,
) {
    let start_time = std::time::Instant::now();

    let signature = &transaction_info.signature.clone();

    if let Err(e) = sender
        .send((Update::Transaction(transaction_info), id))
        .await
    {
        log::error!(
            "Failed to send transaction update with signature {:?}, sender capacity {:?} / max_capacity: {:?} {:?}",
            signature,
            sender.capacity(),
            sender.max_capacity(),
            e
        );
        return;
    }

    TRANSACTION_PROCESS_TIME_NANOS
        .get()
        .unwrap()
        .record(start_time.elapsed().as_nanos() as f64);
    TRANSACTION_UPDATES_RECEIVED.inc();
}
