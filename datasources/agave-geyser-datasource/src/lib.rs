use log::warn;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{
            AccountDeletion, AccountUpdate, Datasource, TransactionUpdate, Update, UpdateType,
        },
        error::CarbonResult,
        metrics::MetricsCollection,
    },
    solana_pubkey::Pubkey,
    std::{
        collections::HashSet,
        sync::Arc,
    },
    tokio::sync::{mpsc::Sender, RwLock},
    tokio_util::sync::CancellationToken,
};

pub enum AgaveGeyserMessage {
    Account(AccountUpdate),
    Transaction(Box<TransactionUpdate>)
}

#[derive(Debug)]
pub struct AgaveGeyserClient {
    receiver: Mutex<Option<Receiver<AgaveGeyserMessage>>>,
    account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
}

impl AgaveGeyserClient {
    pub fn new(
        account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
        receiver: Receiver<AgaveGeyserMessage>,
    ) -> Self {
        Self {
            receiver: Mutex::new(Some(receiver)),
            account_deletions_tracked,
        }
    }
}

#[async_trait]
impl Datasource for AgaveGeyserClient {
    async fn consume(
        &self,
        sender: Sender<Update>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {

        let mut receiver_lock = self.receiver.lock().await;
        let receiver = receiver_lock
            .take()
            .expect("Receiver has already been taken");
        let account_deletions_tracked = Arc::clone(&self.account_deletions_tracked);
      
        tokio::spawn(async move {
            handle_geyser_stream(
                receiver,
                cancellation_token,
                metrics,
                sender,
                &account_deletions_tracked,
            ).await;
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

pub async fn handle_geyser_stream(
    mut receiver: Receiver<AgaveGeyserMessage>,
    cancellation_token: CancellationToken,
    metrics: Arc<MetricsCollection>,
    sender: Sender<Update>,
    account_deletions_tracked: &RwLock<HashSet<Pubkey>>,
) {
    while !cancellation_token.is_cancelled() {
        select! {
            biased;

            _ = cancellation_token.cancelled() => {
                break;
            }

            maybe_msg = receiver.recv() => {
                match maybe_msg {
                    Some(msg) => {
                        match msg {
                            AgaveGeyserMessage::Account(account_info) => {
                               send_subscribe_account_update_info(
                                            account_info,
                                            &metrics,
                                            &sender,
                                            &account_deletions_tracked,
                                        ).await;
                            }

                            AgaveGeyserMessage::Transaction(transaction_update) => {
                                send_subscribe_update_transaction_info(
                                    transaction_update,
                                    &metrics,
                                    &sender,
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
    sender: &Sender<Update>,
    account_deletions_tracked: &RwLock<HashSet<Pubkey>>,
) {
    let start_time = std::time::Instant::now();
    let account = &account_update.account;
    let account_pubkey = account_update.pubkey;

    if account.lamports == 0
        && account.data.is_empty()
        && account.owner == solana_program::system_program::ID
    {
        let accounts = account_deletions_tracked.read().await;
        if accounts.contains(&account_pubkey) {
            let account_deletion = AccountDeletion {
                pubkey: account_pubkey,
                slot: None,
            };
            if let Err(e) = sender.try_send(Update::AccountDeletion(account_deletion)) {
                log::error!(
                    "Failed to send account deletion update for pubkey {:?} : {:?}",
                    account_pubkey,
                    e
                );
            }
        }
    } else {
        if let Err(e) = sender.try_send(Update::Account(account_update)) {
            log::error!(
                "Failed to send account update for pubkey {:?} : {:?}",
                account_pubkey,
                e
            );
        }
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
        .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));
   
}

async fn send_subscribe_update_transaction_info<'a>(
    transaction_info: Box<TransactionUpdate>,
    metrics: &MetricsCollection,
    sender: &Sender<Update>,
) {
    let start_time = std::time::Instant::now();

    let signature = &transaction_info.signature.clone();
    
    if let Err(e) = sender.try_send(Update::Transaction(transaction_info)) {
        log::error!(
            "Failed to send transaction update with signature {:?} : {:?}",
            signature,
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
            .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));
   
}
