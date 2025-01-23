use async_trait::async_trait;
use carbon_core::datasource::AccountDeletion;
use carbon_core::metrics::MetricsCollection;
use carbon_core::{
    datasource::{AccountUpdate, Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
};
use futures::{sink::SinkExt, StreamExt};
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signature};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, RwLock};
use tokio_util::sync::CancellationToken;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::tonic::transport::ClientTlsConfig;
use yellowstone_grpc_proto::{
    convert_from::{create_tx_meta, create_tx_versioned},
    geyser::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions, SubscribeRequestPing,
    },
};

#[derive(Debug)]
pub struct YellowstoneGrpcGeyserClient {
    pub endpoint: String,
    pub x_token: Option<String>,
    pub commitment: Option<CommitmentLevel>,
    pub account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
    pub transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
    pub account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
}

impl YellowstoneGrpcGeyserClient {
    pub fn new(
        endpoint: String,
        x_token: Option<String>,
        commitment: Option<CommitmentLevel>,
        account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
        transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
        account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
    ) -> Self {
        YellowstoneGrpcGeyserClient {
            endpoint,
            x_token,
            commitment,
            account_filters,
            transaction_filters,
            account_deletions_tracked,
        }
    }
}

#[async_trait]
impl Datasource for YellowstoneGrpcGeyserClient {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let sender = sender.clone();
        let endpoint = self.endpoint.clone();
        let x_token = self.x_token.clone();
        let commitment = self.commitment;
        let account_filters = self.account_filters.clone();
        let transaction_filters = self.transaction_filters.clone();
        let account_deletions_tracked = self.account_deletions_tracked.clone();

        let mut geyser_client = GeyserGrpcClient::build_from_shared(endpoint)
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?
            .x_token(x_token)
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(15))
            .tls_config(ClientTlsConfig::new().with_enabled_roots())
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
                blocks: HashMap::new(),
                blocks_meta: HashMap::new(),
                commitment: commitment.map(|x| x as i32),
                accounts_data_slice: vec![],
                ping: None,
            };

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
                                    match message {
                                        Ok(msg) => match msg.update_oneof {
                                            Some(UpdateOneof::Account(account_update)) => {
                                                let start_time = std::time::Instant::now();

                                                metrics.increment_counter("yellowstone_grpc_account_updates_received", 1).await.unwrap();


                                                if let Some(account_info) = account_update.account {
                                                    let Ok(account_pubkey) =
                                                        Pubkey::try_from(account_info.pubkey)
                                                    else {
                                                        continue;
                                                    };

                                                    let Ok(account_owner_pubkey) =
                                                        Pubkey::try_from(account_info.owner)
                                                    else {
                                                        continue;
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
                                                        && account_owner_pubkey
                                                            == solana_sdk::system_program::ID
                                                    {
                                                        let accounts =
                                                            account_deletions_tracked.read().await;
                                                        if accounts.contains(&account_pubkey) {
                                                            let account_deletion = AccountDeletion {
                                                                pubkey: account_pubkey,
                                                                slot: account_update.slot,
                                                            };
                                                            if let Err(e) = sender.send(
                                                                Update::AccountDeletion(account_deletion),
                                                            ) {
                                                                log::error!("Failed to send account deletion update for pubkey {:?} at slot {}: {:?}", account_pubkey, account_update.slot, e);
                                                            }
                                                        }
                                                    } else {
                                                        let update = Update::Account(AccountUpdate {
                                                            pubkey: account_pubkey,
                                                            account,
                                                            slot: account_update.slot,
                                                        });

                                                        if let Err(e) = sender.send(update) {
                                                            log::error!("Failed to send account update for pubkey {:?} at slot {}: {:?}", account_pubkey, account_update.slot, e);
                                                        }
                                                    }

                                                    metrics
                                                            .record_histogram(
                                                                "yellowstone_grpc_account_process_time_nanoseconds",
                                                                start_time.elapsed().as_nanos() as f64
                                                            )
                                                            .await
                                                            .unwrap();

                                                    metrics.increment_counter("yellowstone_grpc_account_updates_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                                } else {
                                                    log::error!("No account info in UpdateOneof::Account at slot {}", account_update.slot);
                                                }
                                            }

                                            Some(UpdateOneof::Transaction(transaction_update)) => {
                                                let start_time = std::time::Instant::now();

                                                if let Some(transaction_info) =
                                                    transaction_update.transaction
                                                {
                                                    let Ok(signature) =
                                                        Signature::try_from(transaction_info.signature)
                                                    else {
                                                        continue;
                                                    };
                                                    let Some(yellowstone_transaction) =
                                                        transaction_info.transaction
                                                    else {
                                                        continue;
                                                    };
                                                    let Some(yellowstone_tx_meta) = transaction_info.meta
                                                    else {
                                                        continue;
                                                    };
                                                    let Ok(versioned_transaction) =
                                                        create_tx_versioned(yellowstone_transaction)
                                                    else {
                                                        continue;
                                                    };
                                                    let meta_original = match create_tx_meta(
                                                        yellowstone_tx_meta,
                                                    ) {
                                                        Ok(meta) => meta,
                                                        Err(err) => {
                                                            log::error!(
                                                                "Failed to create transaction meta: {:?}",
                                                                err
                                                            );
                                                            continue;
                                                        }
                                                    };
                                                    let update = Update::Transaction(TransactionUpdate {
                                                        signature: signature,
                                                        transaction: versioned_transaction,
                                                        meta: meta_original,
                                                        is_vote: transaction_info.is_vote,
                                                        slot: transaction_update.slot,
                                                    });
                                                    if let Err(e) = sender.send(update) {
                                                        log::error!("Failed to send transaction update with signature {:?} at slot {}: {:?}", signature, transaction_update.slot, e);
                                                        continue;
                                                    }
                                                } else {
                                                    log::error!("No transaction info in `UpdateOneof::Transaction` at slot {}", transaction_update.slot);
                                                }

                                                metrics
                                                        .record_histogram(
                                                            "yellowstone_grpc_transaction_process_time_nanoseconds",
                                                            start_time.elapsed().as_nanos() as f64
                                                        )
                                                        .await
                                                        .unwrap();

                                                metrics.increment_counter("yellowstone_grpc_transaction_updates_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                            }

                                            Some(UpdateOneof::Ping(_)) => {
                                                _ = subscribe_tx
                                                    .send(SubscribeRequest {
                                                        ping: Some(SubscribeRequestPing { id: 1 }),
                                                        ..Default::default()
                                                    })
                                                    .await
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
