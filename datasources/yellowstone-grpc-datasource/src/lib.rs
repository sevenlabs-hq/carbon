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

        let mut geyser_client = GeyserGrpcClient::build_from_shared(endpoint)
        .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?
        .x_token(x_token)
        .unwrap()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .connect()
        .await
        .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?;

        let abort_handle = tokio::spawn(async move {
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
                                    Some(UpdateOneof::Account(account_update)) => {
                                       

                                        if let Some(account_info) = account_update.account {

                                            let Ok(account_pubkey) = Pubkey::try_from(account_info.pubkey.clone()) else {
                                                log::error!("Failed to parse account_pubkey: {:?}", account_info.pubkey);
                                                continue;
                                            };
                                            
                                            let Ok(account_owner_pubkey) = Pubkey::try_from(account_info.owner.clone()) else {
                                                log::error!("Failed to parse account_owner_pubkey: {:?}", account_info.owner);
                                                continue;
                                            };

                                            let account = Account {
                                                lamports: account_info.lamports,
                                                data: account_info.data,
                                                owner: account_owner_pubkey,
                                                executable: account_info.executable,
                                                rent_epoch: account_info.rent_epoch
                                            };

                                            let update = Update::Account(AccountUpdate{
                                                pubkey: account_pubkey,
                                                account: account,
                                                slot: account_update.slot,
                                            });

                                            if let Err(e) = sender.send(update) {
                                                log::error!("Failed to send account update for pubkey {:?} at slot {}: {:?}", account_pubkey, account_update.slot, e);
                                                continue;
                                            }
                                            
                                        } else {
                                            log::error!("No account info in `UpdateOneof::Account` at slot {}", account_update.slot);
                                        }
                                    }

                                    Some(UpdateOneof::Transaction(transaction_update)) => {

                                        if let Some(transaction_info) = transaction_update.transaction {

                                            let signature = match Signature::try_from(transaction_info.signature.clone()) {
                                                Ok(sig) => sig,
                                                Err(err) => {
                                                    log::error!("Failed to parse signature: {:?}, error: {:?}", transaction_info.signature, err);
                                                    continue;
                                                }
                                            };
                                            
                                            let yellowstone_transaction = match transaction_info.transaction.clone() {
                                                Some(tx) => tx,
                                                None => {
                                                    log::error!("Failed to retrieve yellowstone_transaction");
                                                    continue;
                                                }
                                            };
                                            
                                            let yellowstone_tx_meta = match transaction_info.meta.clone() {
                                                Some(meta) => meta,
                                                None => {
                                                    log::error!("Failed to retrieve yellowstone_tx_meta");
                                                    continue;
                                                }
                                            };
                                            
                                            let versioned_transaction = match create_tx_versioned(yellowstone_transaction) {
                                                Ok(tx) => tx,
                                                Err(err) => {
                                                    log::error!("Failed to create versioned transaction: {:?}", err);
                                                    continue;
                                                }
                                            };
                                            
                                            let meta_original = match create_tx_meta(yellowstone_tx_meta) {
                                                Ok(meta) => meta,
                                                Err(err) => {
                                                    log::error!("Failed to create transaction meta: {:?}", err);
                                                    continue;
                                                }
                                            };
                                            

                                            let update = Update::Transaction(TransactionUpdate {
                                                signature: signature,
                                                transaction: versioned_transaction ,
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
                                    }

                                    Some(UpdateOneof::Ping(_)) => {
                                      _  =  subscribe_tx
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
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
