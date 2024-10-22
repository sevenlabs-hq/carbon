use async_trait::async_trait;
use carbon_core::{
    datasource::{
        AccountDeletion, AccountUpdate, Datasource, TransactionUpdate, Update, UpdateType,
    },
    error::CarbonResult,
    metrics::MetricsCollection,
};
use futures::StreamExt;
use helius::types::{Cluster, RpcTransactionsConfig};
use helius::Helius;
use solana_sdk::{
    account::Account, bs58, instruction::CompiledInstruction, message::v0::LoadedAddresses,
    pubkey::Pubkey, signature::Signature, transaction_context::TransactionReturnData,
};
use solana_transaction_status::{
    option_serializer::OptionSerializer, InnerInstruction, InnerInstructions, Reward,
    TransactionStatusMeta, TransactionTokenBalance, UiInstruction, UiLoadedAddresses,
};
use std::{collections::HashSet, str::FromStr, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, RwLock};
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
    pub account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
}

impl HeliusWebsocket {
    pub fn new(
        api_key: String,
        filters: Filters,
        account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
    ) -> Self {
        Self {
            api_key,
            filters,
            account_deletions_tracked,
        }
    }
}
#[async_trait]
impl Datasource for HeliusWebsocket {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
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

        let account_deletions_tracked = Arc::clone(&self.account_deletions_tracked);
        let filters = self.filters.clone();
        let cancellation_token = cancellation_token.clone();
        let sender = sender.clone();
        let helius = Arc::new(helius);

        tokio::spawn(async move {
            let mut handles = vec![];

            // Accounts subscriptions
            if !filters.accounts.is_empty() {
                for account in filters.accounts {
                    let cancellation_token = cancellation_token.clone();
                    let sender_clone = sender.clone();
                    let helius_clone = Arc::clone(&helius);
                    let account_deletions_tracked = Arc::clone(&account_deletions_tracked);
                    let metrics = metrics.clone();

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
                                event_result = stream.next() => {
                                    match event_result {
                                        Some(acc_event) => {
                                            let start_time = std::time::Instant::now();
                                            let decoded_account: Account = match acc_event.value.decode() {
                                                Some(account_data) => account_data,
                                                None => {
                                                    log::error!("Error decoding Helius WS Account event");
                                                    continue;
                                                }
                                            };

                                            if decoded_account.lamports == 0 && decoded_account.data.is_empty() && decoded_account.owner == solana_sdk::system_program::ID {
                                                let accounts_tracked =
                                                    account_deletions_tracked.read().await;
                                                if !accounts_tracked.is_empty() {
                                                    if accounts_tracked.contains(&account) {
                                                        let account_deletion = AccountDeletion {
                                                            pubkey: account.clone(),
                                                            slot: acc_event.context.slot,
                                                        };

                                                        metrics.record_histogram("helius_atlas_ws_account_deletion_process_time_milliseconds", start_time.elapsed().as_millis() as f64).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                                        metrics.increment_counter("helius_atlas_ws_account_deletions_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));


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
                                                    pubkey: account,
                                                    account: decoded_account,
                                                    slot: acc_event.context.slot,
                                                });

                                                metrics.record_histogram("helius_atlas_ws_account_process_time_milliseconds", start_time.elapsed().as_millis() as f64).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                                metrics.increment_counter("helius_atlas_ws_account_updates_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));


                                                if let Err(err) = sender_clone.send(update) {
                                                    log::error!("Error sending account update: {:?}", err);
                                                    break;
                                                }
                                            }
                                        },
                                        None => {
                                            log::info!("Helius WS Accounts stream has been closed");
                                            break;
                                        }
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

                    let (mut stream, _unsub) = match ws.transaction_subscribe(config.clone()).await
                    {
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
                            event_result = stream.next() => {
                                match event_result {
                                    Some(tx_event) => {
                                        let start_time = std::time::Instant::now();
                                        let encoded_transaction_with_status_meta = tx_event.transaction;
                                        let signature_str = tx_event.signature;
                                        let Ok(signature) = Signature::from_str(&signature_str) else {
                                            log::error!("Error getting Signature from string");
                                            continue;
                                        };

                                        let meta_original = if let Some(meta) = encoded_transaction_with_status_meta.clone().meta {
                                            meta
                                        } else {
                                            log::warn!("Meta is malformed for transaction: {:?}", signature_str);
                                            continue;
                                        };

                                        if meta_original.status.is_err() {
                                            continue;
                                        }

                                        let Some(decoded_transaction) = encoded_transaction_with_status_meta.transaction.decode() else {
                                            log::error!("Failed to decode transaction: {:?}", encoded_transaction_with_status_meta);
                                            continue;
                                        };

                                        let meta_needed = TransactionStatusMeta {
                                            status: meta_original.status,
                                            fee: meta_original.fee,
                                            pre_balances: meta_original.pre_balances,
                                            post_balances: meta_original.post_balances,
                                            inner_instructions: Some(
                                                meta_original
                                                    .inner_instructions
                                                    .unwrap_or_else(|| vec![])
                                                    .iter()
                                                    .map(|inner_instruction_group| InnerInstructions {
                                                        index: inner_instruction_group.index,
                                                        instructions: inner_instruction_group
                                                            .instructions
                                                            .iter()
                                                            .map(|ui_instruction| match ui_instruction {
                                                                UiInstruction::Compiled(compiled_ui_instruction) => {
                                                                    let decoded_data = bs58::decode(
                                                                        compiled_ui_instruction.data.clone(),
                                                                    )
                                                                    .into_vec()
                                                                    .unwrap_or_else(|_| vec![]);
                                                                    InnerInstruction {
                                                                        instruction: CompiledInstruction {
                                                                            program_id_index: compiled_ui_instruction
                                                                                .program_id_index,
                                                                            accounts: compiled_ui_instruction
                                                                                .accounts
                                                                                .clone(),
                                                                            data: decoded_data,
                                                                        },
                                                                        stack_height: compiled_ui_instruction
                                                                            .stack_height,
                                                                    }
                                                                }
                                                                _ => {
                                                                    log::error!(
                                                                        "Unsupported instruction type encountered"
                                                                    );
                                                                    InnerInstruction {
                                                                        instruction: CompiledInstruction {
                                                                            program_id_index: 0,
                                                                            accounts: vec![],
                                                                            data: vec![],
                                                                        },
                                                                        stack_height: None,
                                                                    }
                                                                }
                                                            })
                                                            .collect::<Vec<InnerInstruction>>(),
                                                    })
                                                    .collect::<Vec<InnerInstructions>>(),
                                            ),
                                            log_messages: Some(meta_original.log_messages.unwrap_or_else(|| vec![])),
                                            pre_token_balances: Some(
                                                meta_original
                                                    .pre_token_balances
                                                    .unwrap_or_else(|| vec![])
                                                    .iter()
                                                    .filter_map(|transaction_token_balance| {
                                                        if let (
                                                            OptionSerializer::Some(owner),
                                                            OptionSerializer::Some(program_id),
                                                        ) = (
                                                            transaction_token_balance.owner.as_ref(),
                                                            transaction_token_balance.program_id.as_ref(),
                                                        ) {
                                                            Some(TransactionTokenBalance {
                                                                account_index: transaction_token_balance.account_index,
                                                                mint: transaction_token_balance.mint.clone(),
                                                                ui_token_amount: transaction_token_balance
                                                                    .ui_token_amount
                                                                    .clone(),
                                                                owner: owner.to_string(),
                                                                program_id: program_id.to_string(),
                                                            })
                                                        } else {
                                                            None
                                                        }
                                                    })
                                                    .collect::<Vec<TransactionTokenBalance>>(),
                                            ),
                                            post_token_balances: Some(
                                                meta_original
                                                    .post_token_balances
                                                    .unwrap_or_else(|| vec![])
                                                    .iter()
                                                    .filter_map(|transaction_token_balance| {
                                                        if let (
                                                            OptionSerializer::Some(owner),
                                                            OptionSerializer::Some(program_id),
                                                        ) = (
                                                            transaction_token_balance.owner.as_ref(),
                                                            transaction_token_balance.program_id.as_ref(),
                                                        ) {
                                                            Some(TransactionTokenBalance {
                                                                account_index: transaction_token_balance.account_index,
                                                                mint: transaction_token_balance.mint.clone(),
                                                                ui_token_amount: transaction_token_balance
                                                                    .ui_token_amount
                                                                    .clone(),
                                                                owner: owner.to_string(),
                                                                program_id: program_id.to_string(),
                                                            })
                                                        } else {
                                                            None
                                                        }
                                                    })
                                                    .collect::<Vec<TransactionTokenBalance>>(),
                                            ),
                                            rewards: Some(
                                                meta_original
                                                    .rewards
                                                    .unwrap_or_else(|| vec![])
                                                    .iter()
                                                    .map(|rewards| Reward {
                                                        pubkey: rewards.pubkey.clone(),
                                                        lamports: rewards.lamports,
                                                        post_balance: rewards.post_balance,
                                                        reward_type: rewards.reward_type,
                                                        commission: rewards.commission,
                                                    })
                                                    .collect::<Vec<Reward>>(),
                                            ),
                                            loaded_addresses: {
                                                let loaded = meta_original.loaded_addresses.unwrap_or_else(|| {
                                                    UiLoadedAddresses {
                                                        writable: vec![],
                                                        readonly: vec![],
                                                    }
                                                });
                                                LoadedAddresses {
                                                    writable: loaded
                                                        .writable
                                                        .iter()
                                                        .map(|w| Pubkey::from_str(&w).unwrap_or_default())
                                                        .collect::<Vec<Pubkey>>(),
                                                    readonly: loaded
                                                        .readonly
                                                        .iter()
                                                        .map(|r| Pubkey::from_str(&r).unwrap_or_default())
                                                        .collect::<Vec<Pubkey>>(),
                                                }
                                            },
                                            return_data: meta_original.return_data.map(|return_data| {
                                                TransactionReturnData {
                                                    program_id: return_data.program_id.parse().unwrap_or_default(),
                                                    data: return_data.data.0.as_bytes().to_vec(),
                                                }
                                            }),
                                            compute_units_consumed: meta_original
                                                .compute_units_consumed
                                                .map(|compute_unit_consumed| compute_unit_consumed)
                                                .or(None),
                                        };

                                        let update = Update::Transaction(TransactionUpdate {
                                            signature,
                                            transaction: decoded_transaction.clone(),
                                            meta: meta_needed,
                                            is_vote: config.filter.vote.is_some_and(|is_vote| is_vote == true),
                                            slot: tx_event.slot,
                                        });

                                        metrics
                                                .record_histogram(
                                                    "helius_atlas_ws_transaction_process_time_milliseconds",
                                                    start_time.elapsed().as_millis() as f64
                                                )
                                                .await
                                                .unwrap_or_else(|value| log::error!("Error recording metric: {}", value));

                                        metrics.increment_counter("helius_atlas_ws_transaction_updates_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {}", value));


                                        if let Err(err) = sender_clone.send(update) {
                                            log::error!("Error sending transaction update: {:?}", err);
                                            break;
                                        }
                                    },
                                    None => {
                                        log::info!("Helius WS Accounts stream has been closed");
                                        break;
                                    }
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
        vec![
            UpdateType::Transaction,
            UpdateType::AccountUpdate,
            UpdateType::AccountDeletion,
        ]
    }
}
