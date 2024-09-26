use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
};
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
    option_serializer::OptionSerializer, InnerInstruction, InnerInstructions, Reward,
    TransactionStatusMeta, TransactionTokenBalance, UiInstruction, UiLoadedAddresses,
    UiTransactionEncoding,
};
use std::{collections::HashSet, str::FromStr, time::Duration};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone)]
pub struct Filters {
    pub accounts: Option<Vec<Pubkey>>,
    pub before_signature: Option<Signature>,
    pub until_signature: Option<Signature>,
    pub commitment: Option<CommitmentConfig>,
}

impl Filters {
    pub fn new(
        accounts: Option<Vec<Pubkey>>,
        before_signature: Option<Signature>,
        until_signature: Option<Signature>,
        commitment: Option<CommitmentConfig>,
    ) -> Self {
        Filters {
            accounts,
            before_signature,
            until_signature,
            commitment,
        }
    }
}

pub struct RpcTransactionCrawler {
    pub rpc_url: String,
    pub account: Pubkey,
    pub batch_limit: usize,
    pub polling_interval: Duration,
    pub filters: Filters,
}

impl RpcTransactionCrawler {
    pub fn new(
        rpc_url: String,
        account: Pubkey,
        batch_limit: usize,
        polling_interval: Duration,
        filters: Filters,
    ) -> Self {
        RpcTransactionCrawler {
            rpc_url,
            account,
            batch_limit,
            polling_interval,
            filters,
        }
    }
}

#[async_trait]
impl Datasource for RpcTransactionCrawler {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let rpc_client = RpcClient::new(&self.rpc_url);
        let account = self.account;
        let batch_limit = self.batch_limit;
        let polling_interval = self.polling_interval;
        let filters = self.filters.clone();
        let sender = sender.clone();

        let abort_handle = tokio::spawn(async move {
            loop {
                let signatures = match rpc_client.get_signatures_for_address_with_config(
                    &account,
                    GetConfirmedSignaturesForAddress2Config {
                        limit: Some(batch_limit),
                        before: filters.before_signature,
                        until: filters.until_signature,
                        commitment: Some(
                            filters.commitment.unwrap_or(CommitmentConfig::confirmed()),
                        ),
                    },
                ) {
                    Ok(sigs) => sigs,
                    Err(e) => {
                        log::error!("Failed to get signatures: {:?}", e);
                        continue;
                    }
                };

                for signature in signatures {
                    let Ok(signature) = Signature::from_str(&signature.signature) else {
                        log::error!("Failed to parse signature: {:?}", signature.signature);
                        continue;
                    };

                    let fetched_transaction = match rpc_client.get_transaction_with_config(
                        &signature,
                        RpcTransactionConfig {
                            encoding: Some(UiTransactionEncoding::Base64),
                            commitment: None,
                            max_supported_transaction_version: Some(0),
                        },
                    ) {
                        Ok(tx) => tx,
                        Err(e) => {
                            log::error!("Failed to get transaction: {:?}", e);
                            continue;
                        }
                    };

                    let transaction = fetched_transaction.transaction;

                    let Some(decoded_transaction) = transaction.transaction.decode() else {
                        log::error!("Failed to decode transaction: {:?}", transaction);
                        continue;
                    };

                    if let Some(accounts) = &filters.accounts {
                        let static_accounts = decoded_transaction.message.static_account_keys();

                        let account_set: HashSet<Pubkey> = accounts.iter().cloned().collect();

                        if !static_accounts
                            .iter()
                            .any(|account| account_set.contains(account))
                        {
                            continue;
                        }
                    }

                    let meta_original = if let Some(meta) = transaction.meta {
                        meta
                    } else {
                        log::warn!("Meta is malformed for transaction: {:?}", signature);
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
                                .map(|iixs| InnerInstructions {
                                    index: iixs.index,
                                    instructions: iixs
                                        .instructions
                                        .iter()
                                        .map(|iix| match iix {
                                            UiInstruction::Compiled(ui_compiled_instruction) => {
                                                InnerInstruction {
                                                    instruction: CompiledInstruction {
                                                        program_id_index: ui_compiled_instruction
                                                            .program_id_index,
                                                        accounts: ui_compiled_instruction
                                                            .accounts
                                                            .clone(),
                                                        data: bs58::decode(
                                                            ui_compiled_instruction.data.clone(),
                                                        )
                                                        .into_vec()
                                                        .unwrap_or_else(|_| vec![]),
                                                    },
                                                    stack_height: ui_compiled_instruction
                                                        .stack_height,
                                                }
                                            }
                                            _ => {
                                                panic!("unimplemented instruction type");
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
                                .filter_map(|tok| {
                                    if let (
                                        OptionSerializer::Some(owner),
                                        OptionSerializer::Some(program_id),
                                    ) = (tok.owner.as_ref(), tok.program_id.as_ref())
                                    {
                                        Some(TransactionTokenBalance {
                                            account_index: tok.account_index,
                                            mint: tok.mint.clone(),
                                            ui_token_amount: tok.ui_token_amount.clone(),
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
                                .filter_map(|ptb| {
                                    if let (
                                        OptionSerializer::Some(owner),
                                        OptionSerializer::Some(program_id),
                                    ) = (ptb.owner.as_ref(), ptb.program_id.as_ref())
                                    {
                                        Some(TransactionTokenBalance {
                                            account_index: ptb.account_index,
                                            mint: ptb.mint.clone(),
                                            ui_token_amount: ptb.ui_token_amount.clone(),
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
                        return_data: meta_original.return_data.map(|ui_data| {
                            TransactionReturnData {
                                program_id: ui_data.program_id.parse().unwrap_or_default(),
                                data: ui_data.data.0.as_bytes().to_vec(),
                            }
                        }),
                        compute_units_consumed: meta_original
                            .compute_units_consumed
                            .map(|cu| cu)
                            .or(None),
                    };

                    let update = Update::Transaction(TransactionUpdate {
                        signature: signature,
                        transaction: decoded_transaction,
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
