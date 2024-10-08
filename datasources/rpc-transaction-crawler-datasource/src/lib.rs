use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
};
use retry::{delay::Fixed, retry, OperationResult};
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_config::RpcTransactionConfig,
};
use solana_sdk::{
    bs58, commitment_config::CommitmentConfig, instruction::CompiledInstruction,
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
use tokio::sync::Semaphore;

#[derive(Debug, Clone)]
pub struct Filters {
    pub accounts: Option<Vec<Pubkey>>,
    pub before_signature: Option<Signature>,
    pub until_signature: Option<Signature>,
}

impl Filters {
    pub fn new(
        accounts: Option<Vec<Pubkey>>,
        before_signature: Option<Signature>,
        until_signature: Option<Signature>,
    ) -> Self {
        Filters {
            accounts,
            before_signature,
            until_signature,
        }
    }
}

pub struct RpcTransactionCrawler {
    pub rpc_url: String,
    pub account: Pubkey,
    pub batch_limit: usize,
    pub polling_interval: Duration,
    pub filters: Filters,
    pub commitment: Option<CommitmentConfig>,
    pub max_concurrent_requests: usize,
}

impl RpcTransactionCrawler {
    pub fn new(
        rpc_url: String,
        account: Pubkey,
        batch_limit: usize,
        polling_interval: Duration,
        filters: Filters,
        commitment: Option<CommitmentConfig>,
        max_concurrent_requests: usize,
    ) -> Self {
        RpcTransactionCrawler {
            rpc_url,
            account,
            batch_limit,
            polling_interval,
            filters,
            commitment,
            max_concurrent_requests,
        }
    }
}

#[async_trait]
impl Datasource for RpcTransactionCrawler {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let rpc_client = Arc::new(RpcClient::new(&self.rpc_url));
        let account = self.account;
        let batch_limit = self.batch_limit;
        let polling_interval = self.polling_interval;
        let filters = self.filters.clone();
        let sender = sender.clone();
        let commitment = self.commitment;
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent_requests));
        let mut last_signature: Option<Signature> = self.filters.before_signature;
        let mut newest_signature: Option<Signature> = None;

        let abort_handle = tokio::spawn(async move {
            loop {
                let rpc_client = Arc::clone(&rpc_client);

                let signatures = match rpc_client.get_signatures_for_address_with_config(
                    &account,
                    GetConfirmedSignaturesForAddress2Config {
                        limit: Some(batch_limit),
                        before: last_signature,
                        until: filters.until_signature,
                        commitment: Some(commitment.unwrap_or(CommitmentConfig::confirmed())),
                    },
                ) {
                    Ok(sigs) => sigs,
                    Err(e) => {
                        log::error!("Failed to get signatures: {:?}", e);
                        continue;
                    }
                };

                if let (Some(newest_sig), Some(last_sig)) = (newest_signature, last_signature) {
                    if newest_sig == last_sig {
                        println!("No new signatures found. Sleeping for interval...");
                        tokio::time::sleep(polling_interval).await;
                        continue;
                    }
                }

                if !signatures.is_empty() && last_signature == filters.before_signature
                    || !signatures.is_empty() && last_signature == newest_signature
                {
                    newest_signature = Some(Signature::from_str(&signatures[0].signature).unwrap());
                }

                if signatures.is_empty() {
                    last_signature = newest_signature;
                    println!("No new signatures found. Sleeping for interval...");
                    tokio::time::sleep(polling_interval).await;
                    continue;
                }

                let mut fetch_tasks = Vec::new();

                for signature in signatures {
                    let Ok(permit) = semaphore.clone().acquire_owned().await else {
                        continue;
                    };

                    let Ok(signature) = Signature::from_str(&signature.signature) else {
                        log::error!("Failed to parse signature: {:?}", signature.signature);
                        continue;
                    };

                    let rpc_client = Arc::clone(&rpc_client);

                    fetch_tasks.push(tokio::spawn(async move {
                        let fetched_transaction = retry(Fixed::from_millis(500).take(10), || {
                            match rpc_client.get_transaction_with_config(
                                &signature,
                                RpcTransactionConfig {
                                    encoding: Some(UiTransactionEncoding::Base64),
                                    commitment: Some(
                                        commitment.unwrap_or(CommitmentConfig::confirmed()),
                                    ),
                                    max_supported_transaction_version: Some(0),
                                },
                            ) {
                                Ok(tx) => OperationResult::Ok(tx),
                                Err(e) => {
                                    log::error!(
                                        "Failed to get transaction: {:?}, signature: {:?}",
                                        e,
                                        signature
                                    );
                                    OperationResult::Retry(e)
                                }
                            }
                        });

                        drop(permit);

                        fetched_transaction.ok().map(|tx| (signature, tx))
                    }));
                }

                let mut results: Vec<(Signature, EncodedConfirmedTransactionWithStatusMeta)> =
                    futures::future::join_all(fetch_tasks)
                        .await
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter_map(|value| value)
                        .collect();

                last_signature = results.last().map(|(signature, _)| signature.clone());

                results.sort_by(|a, b| a.1.slot.cmp(&b.1.slot));

                for (signature, fetched_transaction) in results {
                    let transaction = fetched_transaction.transaction;

                    let meta_original = if let Some(meta) = transaction.clone().meta {
                        meta
                    } else {
                        log::warn!("Meta is malformed for transaction: {:?}", signature);
                        continue;
                    };

                    let Some(decoded_transaction) = transaction.transaction.decode() else {
                        log::error!("Failed to decode transaction: {:?}", transaction);
                        continue;
                    };

                    if let Some(accounts) = &filters.accounts {
                        let account_set: HashSet<Pubkey> = accounts.iter().cloned().collect();

                        let static_accounts = decoded_transaction.message.static_account_keys();

                        let loaded_addresses = meta_original
                            .loaded_addresses
                            .clone()
                            .unwrap_or_else(|| UiLoadedAddresses {
                                writable: vec![],
                                readonly: vec![],
                            });

                        let all_accounts: HashSet<Pubkey> = static_accounts
                            .iter()
                            .cloned()
                            .chain(
                                loaded_addresses
                                    .writable
                                    .iter()
                                    .filter_map(|s| Pubkey::from_str(s).ok()),
                            )
                            .chain(
                                loaded_addresses
                                    .readonly
                                    .iter()
                                    .filter_map(|s| Pubkey::from_str(s).ok()),
                            )
                            .collect();

                        if !all_accounts
                            .iter()
                            .any(|account| account_set.contains(account))
                        {
                            continue;
                        }
                    }

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
                        signature: signature,
                        transaction: decoded_transaction.clone(),
                        meta: meta_needed,
                        is_vote: false,
                        slot: fetched_transaction.slot,
                    });

                    if let Err(e) = sender.send(update) {
                        log::error!("Failed to send update: {:?}", e);
                        continue;
                    }
                }

                tokio::time::sleep(polling_interval).await;
            }
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}
