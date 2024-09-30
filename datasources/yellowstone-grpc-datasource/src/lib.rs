use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use futures::{sink::SinkExt, StreamExt};
use tokio::sync::mpsc::UnboundedSender;
use carbon_core::{
    datasource::{AccountUpdate, Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
};
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::Signature,
};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof,
        CommitmentLevel,
        SubscribeRequest,
        SubscribeRequestFilterAccounts,
        SubscribeRequestFilterTransactions,
        SubscribeRequestPing,
    },
    convert_from::{create_tx_meta, create_tx_versioned},
};

pub struct YellowstoneGrpcGeyserClient {
    pub endpoint: String,
    pub x_token: Option<String>,
    pub commitment: Option<CommitmentLevel>,
    pub account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
    pub transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
}

impl YellowstoneGrpcGeyserClient {
    pub fn new(
        endpoint: String,
        x_token: Option<String>,
        commitment: Option<CommitmentLevel>,
        account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
        transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
    ) -> Self {
        YellowstoneGrpcGeyserClient {
            endpoint,
            x_token,
            commitment,
            account_filters,
            transaction_filters,
        }
    }
}

#[async_trait]
impl Datasource for YellowstoneGrpcGeyserClient {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let sender = sender.clone();
        let endpoint = self.endpoint.clone();
        let x_token = self.x_token.clone();
        let commitment = self.commitment;
        let account_filters = self.account_filters.clone();
        let transaction_filters = self.transaction_filters.clone();

        let abort_handle = tokio::spawn(async move {
            let geyser_client_build_result = GeyserGrpcClient::build_from_shared(endpoint);

            if let Err(err) = geyser_client_build_result {
                log::error!("Error building Geyser client: {}", err);
                return;
            }

            let geyser_client_connection_result = geyser_client_build_result
                .unwrap() // safe to unwrap
                .x_token(x_token)
                .unwrap()
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(10))
                .connect()
                .await
                .map_err(|err| {
                    log::error!("Error connecting to Geyser server: {}", err);
                    err
                });

            if let Err(err) = geyser_client_connection_result {
                log::error!("Failed to connect to Geyser server: {}", err);
                return;
            }

            let mut geyser_client = geyser_client_connection_result.unwrap(); // safe to unwrap

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
                match geyser_client
                    .subscribe_with_request(Some(subscribe_request.clone()))
                    .await
                {
                    Ok((mut subscribe_tx, mut stream)) => {
                        while let Some(message) = stream.next().await {
                            match message {
                                Ok(msg) => match msg.update_oneof {
                                    Some(UpdateOneof::Account(account)) => {
                                        let account_slot = account.slot;

                                        if let Some(account_info) = account.account {

                                            let account_pubkey = Pubkey::try_from(account_info.pubkey.clone()).unwrap_or_default();

                                            let account_owner_pubkey = Pubkey::try_from(account_info.owner.clone()).unwrap_or_default();

                                            let account = Account {
                                                lamports: account_info.lamports,
                                                data:account_info.data,
                                                owner:account_owner_pubkey,
                                                executable:account_info.executable,
                                                rent_epoch: account_info.rent_epoch
                                            };

                                            let update = Update::Account(AccountUpdate{
                                                pubkey:account_pubkey,
                                                account:account,
                                                slot:account_slot,
                                            });

                                            if let Err(e) = sender.send(update) {
                                                log::error!("Failed to send account update for pubkey {:?} at slot {}: {:?}", account_pubkey, account_slot, e);
                                                continue;
                                            }
                                            
                                        } else {
                                            log::error!("No account info in `UpdateOneof::Account` at slot {}", account_slot);
                                        }
                                    }

                                    Some(UpdateOneof::Transaction(transaction)) => {
                                        
                                        let transaction_slot = transaction.slot;

                                        if let Some(transaction_info) = transaction.transaction {

                                            let signature = Signature::try_from(transaction_info.signature.clone())
                                            .unwrap_or_default();
                                        
                                            let yellowstone_transaction = transaction_info.transaction.unwrap_or_default();

                                            let yellowstone_tx_meta = transaction_info.meta.unwrap_or_default();

                                            let versioned_transaction = create_tx_versioned(yellowstone_transaction).unwrap_or_default();

                                            let meta_original = create_tx_meta(yellowstone_tx_meta).unwrap_or_default();

                                            let update = Update::Transaction(TransactionUpdate {
                                                signature: signature,
                                                transaction: versioned_transaction ,
                                                meta: meta_original,
                                                is_vote: transaction_info.is_vote,
                                                slot: transaction_slot,
                                            });
                        
                                            if let Err(e) = sender.send(update) {
                                                log::error!("Failed to send transaction update with signature {:?} at slot {}: {:?}", signature, transaction_slot, e);
                                                continue;
                                            }
                                            
                                        } else {
                                            log::error!("No transaction info in `UpdateOneof::Transaction` at slot {}", transaction_slot);
                                        }
                                    }

                                    Some(UpdateOneof::Ping(_)) => {
                                        subscribe_tx
                                            .send(SubscribeRequest {
                                                ping: Some(SubscribeRequestPing { id: 1 }),
                                                ..Default::default()
                                            })
                                            .await.unwrap_or_default();
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
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
