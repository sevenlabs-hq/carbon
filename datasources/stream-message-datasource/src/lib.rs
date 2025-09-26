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
    log::{error, warn},
    solana_pubkey::Pubkey,
    std::{collections::HashSet, sync::Arc},
    tokio::{
        select,
        sync::{
            mpsc::{Receiver, Sender},
            RwLock,
        },
    },
    tokio_util::sync::CancellationToken,
};

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
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
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
                metrics,
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
    metrics: Arc<MetricsCollection>,
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
                                    &metrics,
                                    &sender,
                                    account_deletions_tracked,
                                    id.clone()
                                ).await;
                            }

                            UnifiedMessage::Transaction(transaction_update) => {
                                send_subscribe_update_transaction_info(
                                    transaction_update,
                                    &metrics,
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
    metrics: &MetricsCollection,
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

    metrics
        .record_histogram(
            "agave_grpc_account_process_time_nanoseconds",
            start_time.elapsed().as_nanos() as f64,
        )
        .await
        .expect("Failed to record histogram");

    metrics
        .increment_counter("agave_grpc_account_updates_received", 1)
        .await
        .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));
}

async fn send_subscribe_update_transaction_info(
    transaction_info: Box<TransactionUpdate>,
    metrics: &MetricsCollection,
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

    metrics
        .record_histogram(
            "agave_grpc_transaction_process_time_nanoseconds",
            start_time.elapsed().as_nanos() as f64,
        )
        .await
        .expect("Failed to record histogram");

    metrics
        .increment_counter("agave_grpc_transaction_updates_received", 1)
        .await
        .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));
}
