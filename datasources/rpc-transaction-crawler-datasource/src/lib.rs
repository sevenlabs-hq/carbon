use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
};
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_sdk::{
    bs58, commitment_config::CommitmentConfig, instruction::CompiledInstruction,
    message::v0::LoadedAddresses, pubkey::Pubkey, signature::Signature,
    transaction_context::TransactionReturnData,
};
use solana_transaction_status::{
    InnerInstruction, InnerInstructions, Reward, TransactionStatusMeta, TransactionTokenBalance,
    UiInstruction, UiLoadedAddresses,
};
use std::{str::FromStr, time::Duration};
use tokio::sync::mpsc::UnboundedSender;

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
        let account = self.account.clone();
        let batch_limit = self.batch_limit;
        let polling_interval = self.polling_interval;
        let filters = self.filters.clone();
        let sender = sender.clone();

        let abort_handle = tokio::spawn(async move {
            loop {
                let config = GetConfirmedSignaturesForAddress2Config {
                    limit: Some(batch_limit),
                    before: filters.before_signature.clone(),
                    until: filters.until_signature.clone(),
                    commitment: Some(CommitmentConfig::confirmed()),
                };

                let signatures =
                    match rpc_client.get_signatures_for_address_with_config(&account, config) {
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

                    let fetched_transaction = match rpc_client.get_transaction(
                        &signature,
                        solana_transaction_status::UiTransactionEncoding::JsonParsed,
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

                    let Some(legacy_transaction) =
                        decoded_transaction.clone().into_legacy_transaction()
                    else {
                        log::error!("Failed to convert to legacy transaction: {:?}", transaction);
                        continue;
                    }; // Still need to test this part to make sure this still keeps all necessary address lookup tables (test with jupiter swaps to know if all accounts are kept)

                    if let Some(accounts) = &filters.accounts {
                        let transaction_accounts: Vec<Pubkey> =
                            legacy_transaction.message.account_keys.clone();
                        if !transaction_accounts
                            .iter()
                            .any(|account| accounts.contains(account))
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
                                .map(|tok| TransactionTokenBalance {
                                    account_index: tok.account_index,
                                    mint: tok.mint.clone(),
                                    ui_token_amount: tok.ui_token_amount.clone(),
                                    owner: tok.owner.clone().unwrap(),
                                    program_id: tok.program_id.clone().unwrap(), // I'm not sure how to handle those similar parts, as there's no option to use unwrap_or_default(). We can not set fallback default values for those right ?
                                })
                                .collect::<Vec<TransactionTokenBalance>>(),
                        ),
                        post_token_balances: Some(
                            meta_original
                                .post_token_balances
                                .unwrap()
                                .iter()
                                .map(|ptb| TransactionTokenBalance {
                                    account_index: ptb.account_index,
                                    mint: ptb.mint.clone(),
                                    ui_token_amount: ptb.ui_token_amount.clone(),
                                    owner: ptb.owner.clone().unwrap(),
                                    program_id: ptb.program_id.clone().unwrap(),
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
                                    .map(|w| Pubkey::from_str(&w).unwrap())
                                    .collect::<Vec<Pubkey>>(),
                                readonly: loaded
                                    .readonly
                                    .iter()
                                    .map(|r| Pubkey::from_str(&r).unwrap())
                                    .collect::<Vec<Pubkey>>(),
                            }
                        },
                        return_data: meta_original.return_data.map(|ui_data| {
                            TransactionReturnData {
                                program_id: ui_data.program_id.parse().unwrap(),
                                data: ui_data.data.0.as_bytes().to_vec(),
                            }
                        }),
                        compute_units_consumed: meta_original
                            .compute_units_consumed
                            .map(|cu| cu)
                            .or(None),
                    };

                    let update = Update::Transaction(TransactionUpdate {
                        signature: signature.clone(),
                        transaction: decoded_transaction,
                        meta: meta_needed,
                        is_vote: false,
                        slot: fetched_transaction.slot,
                    });

                    sender.send(update).unwrap();
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
