use async_trait::async_trait;
use carbon_core::{
    datasource::{AccountUpdate, Datasource, Update, UpdateType},
    error::CarbonResult,
};
use futures::StreamExt;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient, rpc_config::RpcProgramAccountsConfig,
};
use solana_sdk::{account::Account, pubkey::Pubkey};
use std::str::FromStr;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Filters {
    pub pubkey: Pubkey,
    pub program_subscribe_config: Option<RpcProgramAccountsConfig>,
}

impl Filters {
    pub fn new(pubkey: Pubkey, program_subscribe_config: Option<RpcProgramAccountsConfig>) -> Self {
        Filters {
            pubkey,
            program_subscribe_config,
        }
    }
}

pub struct RpcProgramSubscribe {
    pub rpc_ws_url: String,
    pub filters: Filters,
}

impl RpcProgramSubscribe {
    pub fn new(rpc_ws_url: String, filters: Filters) -> Self {
        Self {
            rpc_ws_url,
            filters,
        }
    }
}

#[async_trait]
impl Datasource for RpcProgramSubscribe {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        let client = PubsubClient::new(&self.rpc_ws_url).await.map_err(|err| {
            carbon_core::error::Error::Custom(format!(
                "Failed to create an RPC subscribe client: {err}"
            ))
        })?;

        let filters = self.filters.clone();
        let sender = sender.clone();

        tokio::spawn(async move {
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
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancelling RPC program subscription...");
                        break;
                    }
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
                            None => {
                                log::info!("Program accounts stream has been closed");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
