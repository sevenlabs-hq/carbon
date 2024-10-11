use async_trait::async_trait;
use carbon_core::{
    datasource::{AccountUpdate, Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
};
use futures::StreamExt;
use helius::types::{
    Cluster, RpcTransactionsConfig, TransactionSubscribeFilter, TransactionSubscribeOptions,
};
use helius::Helius;
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_config::RpcTransactionConfig,
};
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::CompiledInstruction,
    message::v0::LoadedAddresses, pubkey::Pubkey, signature::Signature,
    transaction_context::TransactionReturnData,
};
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta,
    InnerInstruction, InnerInstructions, Reward, TransactionStatusMeta, TransactionTokenBalance,
    UiInstruction, UiLoadedAddresses, UiTransactionEncoding,
};
use std::{collections::HashSet, str::FromStr, sync::Arc, time::Duration};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Filters {
    pub accounts: Vec<Pubkey>,
    pub transactions: Option<RpcTransactionsConfig>,
}

impl Filters {
    pub fn new(
        accounts: Vec<Pubkey>,
        transactions: Option<RpcTransactionsConfig>,
    ) -> CarbonResult<Self> {
        if accounts.is_empty() && transactions.is_none() {
            return CarbonResult::Err(carbon_core::error::Error::Custom(format!("Error creating Filters for the Helius WebSocket: accounts and transactions can't be both empty")));
        };

        Ok(Filters {
            accounts,
            transactions,
        })
    }
}

pub struct HeliusWebsocket {
    pub api_key: String,
    pub filters: Filters,
}

impl HeliusWebsocket {
    pub fn new(api_key: String, filters: Filters) -> Self {
        Self { api_key, filters }
    }
}

#[async_trait]
impl Datasource for HeliusWebsocket {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        if self.filters.accounts.is_empty() && self.filters.transactions.is_none() {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Filters can't be empty.".to_string(),
            ));
        }

        let helius = Helius::new_with_ws(&self.api_key, Cluster::MainnetBeta)
            .await
            .map_err(|err| {
                carbon_core::error::Error::Custom(format!("Error creating Helius client: {}", err))
            })?;

        let filters = self.filters.clone();
        let sender = sender.clone();
        let cancellation_token = cancellation_token.clone();
        let helius = Arc::new(helius);

        tokio::spawn(async move {
            let mut handles = vec![];

            // Accounts subscriptions
            if !filters.accounts.is_empty() {
                for account in filters.accounts {
                    let cancellation_token = cancellation_token.clone();
                    let sender_clone = sender.clone();
                    let helius_clone = Arc::clone(&helius);

                    let handle = tokio::spawn(async move {
                        let ws = match helius_clone.ws() {
                            Some(ws) => ws,
                            None => {
                                log::error!("Helius Websocket not available");
                                return;
                            }
                        };

                        let (mut stream, _unsub) = match ws.account_subscribe(&account, None).await
                        {
                            Ok(subscription) => subscription,
                            Err(err) => {
                                log::error!(
                                    "Failed to subscribe to account {}: {:?}",
                                    account,
                                    err
                                );
                                return;
                            }
                        };

                        loop {
                            tokio::select! {
                                _ = cancellation_token.cancelled() => {
                                    log::info!("Cancelling Helius WS accounts subscription...");
                                    break;
                                }
                                event = stream.next() => {
                                    if let Some(event) = event {
                                        let update = Update::Account(AccountUpdate {
                                            // TODO
                                        });
                                        if let Err(err) = sender_clone.send(update) {
                                            log::error!("Error sending account update: {:?}", err);
                                            break;
                                        }
                                    } else {
                                        log::info!("Helius WS Accounts stream has been closed".);
                                        break;
                                    }
                                }
                            }
                        }
                    });

                    handles.push(handle);
                }
            }

            // Transactions subscription
            if let Some(config) = filters.transactions {
                let cancellation_token = cancellation_token.clone();
                let sender_clone = sender.clone();
                let helius_clone = Arc::clone(&helius);

                let handle = tokio::spawn(async move {
                    let ws = match helius_clone.ws() {
                        Some(ws) => ws,
                        None => {
                            log::error!("Helius Websocket not available");
                            return;
                        }
                    };

                    let (mut stream, _unsub) = match ws.transaction_subscribe(config).await {
                        Ok(subscription) => subscription,
                        Err(err) => {
                            log::error!("Failed to subscribe to transactions: {:?}", err);
                            return;
                        }
                    };

                    loop {
                        tokio::select! {
                            _ = cancellation_token.cancelled() => {
                                log::info!("Cancelling Helius WS transaction subscription...");
                                break;
                            }
                            event = stream.next() => {
                                if let Some(event) = event {
                                    let update = Update::Transaction(TransactionUpdate {
                                        //TODO
                                    });
                                    if let Err(err) = sender_clone.send(update) {
                                        log::error!("Error sending transaction update: {:?}", err);
                                        break;
                                    }
                                } else {
                                    log::info!("Helius WS Transactions stream has been closed.");
                                    break;
                                }
                            }
                        }
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                if let Err(e) = handle.await {
                    log::error!("Helius WS Task failed: {:?}", e);
                }
            }
        });

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction, UpdateType::AccountUpdate]
    }
}
