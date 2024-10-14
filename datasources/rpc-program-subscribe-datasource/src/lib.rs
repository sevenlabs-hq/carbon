use async_trait::async_trait;
use carbon_core::{
    datasource::{AccountDeletion, AccountUpdate, Datasource, Update, UpdateType},
    error::CarbonResult,
};
use futures::StreamExt;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::RpcProgramAccountsConfig,
};
use solana_sdk::{account::Account, pubkey::Pubkey};
use std::{collections::HashSet, str::FromStr, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, RwLock};

#[derive(Debug, Clone)]
pub struct Filters {
    pub pubkey: Pubkey,
    pub program_subscribe_config: Option<RpcProgramAccountsConfig>,
}

impl Filters {
    pub fn new(
        pubkey: Pubkey,
        program_subscribe_config: Option<RpcProgramAccountsConfig>,
    ) -> CarbonResult<Self> {
        Ok(Filters {
            pubkey,
            program_subscribe_config,
        })
    }
}

pub struct RpcProgramSubscribe {
    pub rpc_url: String,
    pub filters: Filters,
    pub account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
}

impl RpcProgramSubscribe {
    pub fn new(rpc_url: String, filters: Filters, account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,) -> Self {
        Self { rpc_url, filters, account_deletions_tracked }
    }
}

#[async_trait]
impl Datasource for RpcProgramSubscribe {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let client = PubsubClient::new(&self.rpc_url).await.map_err(|err| {
            carbon_core::error::Error::Custom(format!(
                "Failed to create an RPC subscribe client: {err}"
            ))
        })?;

        let account_deletions_tracked = Arc::clone(&self.account_deletions_tracked);
        let filters = self.filters.clone();
        let sender = sender.clone();

        let abort_handle = tokio::spawn(async move {
            let sender_clone = sender.clone();
            let (mut stream, _unsub) = match client
                .program_subscribe(&filters.pubkey, filters.program_subscribe_config)
                .await
            {
                Ok(subscription) => subscription,
                Err(err) => {
                    log::error!("Failed to subscribe to blocks updates: {:?}", err);
                    return;
                }
            };

            loop {
                tokio::select! {
                    // TODO:
                    // _ = cancellation_token.cancelled() => {
                    //     log::info!("Cancelling RPC program subscription...");
                    //     break;
                    // }
                    event_result = stream.next() => {
                        match event_result {
                            Some(acc_event) => {
                                    let decoded_account: Account = match acc_event.value.account.decode() {
                                        Some(account_data) => account_data,
                                        None => {
                                            log::error!("Error decoding Helius WS Account event");
                                            continue;
                                        }
                                    }; 

                                    let Ok(account_pubkey) = Pubkey::from_str(&acc_event.value.pubkey) else {
                                        log::error!("Error parsing account pubkey. Value: {}", &acc_event.value.pubkey);
                                        continue;
                                    };

                                    if decoded_account.lamports == 0 && decoded_account.data.is_empty() && decoded_account.owner == solana_sdk::system_program::ID {
                                        let accounts_tracked =
                                            account_deletions_tracked.read().await;
                                        if !accounts_tracked.is_empty() {
                                            if accounts_tracked.contains(&account_pubkey) {
                                                let account_deletion = AccountDeletion {
                                                    pubkey: account_pubkey,
                                                    slot: acc_event.context.slot,
                                                };
                                                if let Err(err) = sender_clone.send(
                                                    Update::AccountDeletion(account_deletion),
                                                ) {
                                                    log::error!("Error sending account update: {:?}", err);
                                                    break;
                                                }
                                            }
                                        }
                                    } else {
                                        let update = Update::Account(AccountUpdate {
                                            pubkey: account_pubkey,
                                            account: decoded_account,
                                            slot: acc_event.context.slot,
                                        });

                                        if let Err(err) = sender_clone.send(update) {
                                            log::error!("Error sending account update: {:?}", err);
                                            break;
                                        }
                                    }
                            }
                            None => {
                                log::info!("Program accounts stream has been closed");
                                break;
                            }
                        }
                    }
                }
            }
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountDeletion, UpdateType::AccountDeletion]
    }
}