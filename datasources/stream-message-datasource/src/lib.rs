use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{
            AccountUpdate, Datasource, DatasourceId, TransactionUpdate, Update, UpdateType,
        },
        error::CarbonResult,
        metrics::{Counter, Histogram, MetricsRegistry},
    },
    log::{error, warn},
    std::sync::LazyLock,
    tokio::{
        select,
        sync::mpsc::{Receiver, Sender},
    },
    tokio_util::sync::CancellationToken,
};

static ACCOUNT_PROCESS_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "agave_grpc_account_process_time_nanoseconds",
        "Time to process account in nanoseconds",
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
    "agave_grpc_account_updates_received_total",
    "Account updates received from stream message",
);
static ACCOUNT_DELETION_PROCESS_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "agave_grpc_account_deletion_process_time_nanoseconds",
        "Time to process account deletions in nanoseconds",
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
static ACCOUNT_DELETIONS_RECEIVED: Counter = Counter::new(
    "agave_grpc_account_deletions_received_total",
    "Account deletions received from stream message",
);
static TRANSACTION_PROCESS_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "agave_grpc_transaction_process_time_nanoseconds",
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
static TRANSACTION_UPDATES_RECEIVED: Counter = Counter::new(
    "agave_grpc_transaction_updates_received_total",
    "Transaction updates received from stream message",
);

fn register_stream_message_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&ACCOUNT_UPDATES_RECEIVED);
    registry.register_counter(&ACCOUNT_DELETIONS_RECEIVED);
    registry.register_counter(&TRANSACTION_UPDATES_RECEIVED);
    registry.register_histogram(&ACCOUNT_PROCESS_TIME_NANOS);
    registry.register_histogram(&ACCOUNT_DELETION_PROCESS_TIME_NANOS);
    registry.register_histogram(&TRANSACTION_PROCESS_TIME_NANOS);
}

pub enum UnifiedMessage {
    Account(AccountUpdate),
    Transaction(Box<TransactionUpdate>),
}

#[derive(Debug)]
pub struct StreamMessageClient {
    receiver: std::sync::Mutex<Option<Receiver<UnifiedMessage>>>,
}

impl StreamMessageClient {
    pub fn new(receiver: Receiver<UnifiedMessage>) -> Self {
        Self {
            receiver: std::sync::Mutex::new(Some(receiver)),
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

        let id = id.clone();

        tokio::spawn(async move {
            handle_message_stream(receiver, cancellation_token, sender, id).await;
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
    id: DatasourceId,
) {
    let start_time = std::time::Instant::now();
    let account_pubkey = account_update.pubkey;
    let update = account_update.into_update();
    let is_deletion = matches!(&update, Update::AccountDeletion(_));

    if let Err(e) = sender.send((update, id.clone())).await {
        log::error!(
            "Failed to send account event for pubkey {:?}, sender capacity {:?} / max_capacity: {:?} : {:?},",
            account_pubkey,
            sender.capacity(),
            sender.max_capacity(),
            e
        );
    }

    if is_deletion {
        ACCOUNT_DELETION_PROCESS_TIME_NANOS.record(start_time.elapsed().as_nanos() as f64);
        ACCOUNT_DELETIONS_RECEIVED.inc();
    } else {
        ACCOUNT_PROCESS_TIME_NANOS.record(start_time.elapsed().as_nanos() as f64);
        ACCOUNT_UPDATES_RECEIVED.inc();
    }
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

    TRANSACTION_PROCESS_TIME_NANOS.record(start_time.elapsed().as_nanos() as f64);
    TRANSACTION_UPDATES_RECEIVED.inc();
}
