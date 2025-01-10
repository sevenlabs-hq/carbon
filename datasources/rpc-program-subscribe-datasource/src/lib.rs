use async_trait::async_trait;
use bincode;
use carbon_core::{
    datasource::{AccountUpdate, Datasource, Update, UpdateType},
    error::CarbonResult,
    metrics::MetricsCollection,
};
use futures::StreamExt;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient, rpc_config::RpcProgramAccountsConfig,
};
use solana_sdk::{account::Account, pubkey::Pubkey, sysvar::clock::Clock};
use std::{
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

const MAX_RECONNECTION_ATTEMPTS: u32 = 10;
const RECONNECTION_DELAY_MS: u64 = 3000;
const MAX_MISSED_BLOCKS: u64 = 10;

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
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let mut reconnection_attempts = 0;

        loop {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested, stopping reconnection attempts");
                break;
            }

            let client = match PubsubClient::new(&self.rpc_ws_url).await {
                Ok(client) => client,
                Err(err) => {
                    log::error!("Failed to create RPC subscribe client: {}", err);
                    reconnection_attempts += 1;
                    if reconnection_attempts >= MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to create RPC subscribe client after {} attempts: {}",
                            MAX_RECONNECTION_ATTEMPTS, err
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
                    continue;
                }
            };

            let filters = self.filters.clone();
            let sender_clone = sender.clone();

            let (mut program_stream, _program_unsub) = match client
                .program_subscribe(&filters.pubkey, filters.program_subscribe_config)
                .await
            {
                Ok(subscription) => subscription,
                Err(err) => {
                    log::error!("Failed to subscribe to program updates: {:?}", err);
                    reconnection_attempts += 1;
                    if reconnection_attempts > MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to subscribe after {} attempts: {}",
                            MAX_RECONNECTION_ATTEMPTS, err
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
                    continue;
                }
            };

            let (mut clock_stream, _clock_unsub) = match client
                .program_subscribe(&solana_sdk::sysvar::clock::ID, None)
                .await
            {
                Ok(subscription) => subscription,
                Err(err) => {
                    log::error!("Failed to subscribe to Clock sysvar: {:?}", err);
                    reconnection_attempts += 1;
                    if reconnection_attempts > MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to subscribe to Clock after {} attempts: {}",
                            MAX_RECONNECTION_ATTEMPTS, err
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
                    continue;
                }
            };

            reconnection_attempts = 0;

            let mut last_slot = 0u64;
            let mut last_clock_update = Instant::now();

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancellation requested, stopping subscription...");
                        return Ok(());
                    }
                    _ = tokio::time::sleep(Duration::from_secs(5)) => {
                        if last_clock_update.elapsed() > Duration::from_secs(5) {
                            log::warn!("No Clock updates received for 5 seconds, considering connection stale and reconnecting...");
                            break;
                        }
                    }
                    clock_event = clock_stream.next() => {
                        match clock_event {
                            Some(clock_update) => {
                                last_clock_update = Instant::now();
                                if let Some(clock_data) = clock_update.value.account.decode::<Account>() {
                                    if let Ok(clock) = bincode::deserialize::<Clock>(&clock_data.data) {
                                        let current_slot = clock.slot;

                                        if last_slot > 0 && current_slot > last_slot + MAX_MISSED_BLOCKS {
                                            log::warn!("Detected large slot gap: last_slot={}, current_slot={}, gap={}",
                                                last_slot, current_slot, current_slot - last_slot);
                                            break;
                                        }
                                        last_slot = current_slot;
                                    }
                                }
                            }
                            None => {
                                log::warn!("Clock sysvar stream closed, reconnecting...");
                                break;
                            }
                        }
                    }
                    event_result = program_stream.next() => {
                        match event_result {
                            Some(acc_event) => {
                                let start_time = std::time::Instant::now();
                                let decoded_account: Account = match acc_event.value.account.decode() {
                                    Some(account_data) => account_data,
                                    None => {
                                        log::error!("Error decoding account event");
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

                                metrics
                                    .record_histogram(
                                        "program_subscribe_account_process_time_nanoseconds",
                                        start_time.elapsed().as_nanos() as f64
                                    )
                                    .await
                                    .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                metrics.increment_counter("program_subscribe_accounts_processed", 1)
                                    .await
                                    .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                if let Err(err) = sender_clone.send(update) {
                                    log::error!("Error sending account update: {:?}", err);
                                    break;
                                }
                            }
                            None => {
                                log::warn!("Program accounts stream has been closed, attempting to reconnect...");
                                break;
                            }
                        }
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
        }

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
