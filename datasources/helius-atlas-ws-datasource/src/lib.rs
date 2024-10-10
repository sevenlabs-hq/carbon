use carbon_core::{
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
};
use futures::{
    future::{AbortHandle, Abortable},
    StreamExt,
};
use helius::error::Result;
use helius::types::{
    Cluster, RpcTransactionsConfig, TransactionSubscribeFilter, TransactionSubscribeOptions,
};
use helius::Helius;
use retry::{delay::Fixed, retry, OperationResult};
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

        Filters {
            accounts,
            transactions,
        }
    }
}

pub struct HeliusWebsocket {
    pub apy_key: String,
    pub filters: Filters,
}

impl HeliusWebsocket {
    pub fn new(api_key: String, filters: Filters) -> Self {
        RpcTransactionCrawler { api_key, filters }
    }
}

#[async_trait]
impl Datasource for HeliusWebsocket {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let helius = Helius::new_with_ws(&self.api_key, Cluster::MainnetBeta)
            .await
            .map_err(|err| {
                carbon_core::error::Error::Custom(format!(
                    "Error creating Helius client: {}",
                    err.to_string()
                ))
            })?;

        let filters = self.filters.clone();
        let sender = sender.clone();

        let ws = helius.ws().ok_or_else(|| {
            carbon_core::error::Error::Custom("Websocket not available".to_string())
        })?;

        let (abort_handle, abort_registration) = AbortHandle::new_pair();

        let abortable_task = Abortable::new(
            async move {
                let mut handles = vec![];

                // Accounts subscriptions
                if !filters.accounts.is_empty() {
                    for account in filters.accounts {
                        let (mut stream, _unsub) =
                            ws.account_subscribe(&account, None).await.unwrap();
                        let sender_clone = sender.clone();
                        let handle = tokio::spawn(async move {
                            while let Some(event) = stream.next().await {
                                let update = Update::Account {
                                    // TODO
                                };
                                if sender_clone.send(update).is_err() {
                                    break;
                                }
                            }
                        });
                        handles.push(handle);
                    }
                }

                // Txs subscription
                if let Some(config) = filters.transactions {
                    let (mut stream, _unsub) = ws.transaction_subscribe(config).await.unwrap();
                    let sender_clone = sender.clone();
                    let handle = tokio::spawn(async move {
                        while let Some(event) = stream.next().await {
                            let update = Update::Transaction(TransactionUpdate {
                                // TODO
                            });
                            if sender_clone.send(update).is_err() {
                                break;
                            }
                        }
                    });
                    handles.push(handle);
                }

                for handle in handles {
                    let _ = handle.await;
                }
            },
            abort_registration,
        );

        tokio::spawn(abortable_task);

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction, UpdateType::Account]
    }
}
