use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
};
use futures::StreamExt;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
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
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::JoinHandle,
};

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
        sender: &mpsc::UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            self.rpc_url.clone(),
            self.commitment.unwrap_or(CommitmentConfig::confirmed()),
        ));
        let account = self.account;
        let batch_limit = self.batch_limit;
        let polling_interval = self.polling_interval;
        let filters = self.filters.clone();
        let sender = sender.clone();
        let commitment = self.commitment;
        let max_concurrent_requests = self.max_concurrent_requests;

        let (signature_sender, signature_receiver) = mpsc::channel(1000);
        let (transaction_sender, transaction_receiver) = mpsc::channel(1000);

        let signature_fetcher = signature_fetcher(
            rpc_client.clone(),
            account,
            batch_limit,
            polling_interval,
            signature_sender,
            filters.clone(),
            commitment,
        );

        let transaction_fetcher = transaction_fetcher(
            rpc_client,
            signature_receiver,
            transaction_sender,
            commitment,
            max_concurrent_requests,
        );

        let processing_task = processing_task(transaction_receiver, sender, filters);

        let main_task = tokio::spawn(async move {
            tokio::select! {
                _ = signature_fetcher => {},
                _ = transaction_fetcher => {},
                _ = processing_task => {},
            }
        });

        Ok(main_task.abort_handle())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}

fn signature_fetcher(
    rpc_client: Arc<RpcClient>,
    account: Pubkey,
    batch_limit: usize,
    polling_interval: Duration,
    signature_sender: Sender<Signature>,
    filters: Filters,
    commitment: Option<CommitmentConfig>,
) -> JoinHandle<()> {
    let rpc_client = Arc::clone(&rpc_client);
    let account = account;
    let batch_limit = batch_limit;
    let filters = filters.clone();
    let signature_sender = signature_sender.clone();
    let polling_interval = polling_interval;
    tokio::spawn(async move {
        let mut last_fetched_signature = filters.before_signature;

        loop {
            match rpc_client
                .get_signatures_for_address_with_config(
                    &account,
                    GetConfirmedSignaturesForAddress2Config {
                        before: last_fetched_signature,
                        until: filters.until_signature,
                        limit: Some(batch_limit),
                        commitment: Some(commitment.unwrap_or(CommitmentConfig::confirmed())),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(signatures) => {
                    if signatures.is_empty() {
                        tokio::time::sleep(polling_interval).await;
                        continue;
                    }

                    for sig_info in signatures.iter() {
                        let signature = match Signature::from_str(&sig_info.signature) {
                            Ok(sig) => sig,
                            Err(e) => {
                                log::error!("Invalid signature: {:?}", e);
                                continue;
                            }
                        };

                        if let Err(e) = signature_sender.send(signature).await {
                            log::error!("Failed to send signature: {:?}", e);
                            break;
                        }
                    }

                    last_fetched_signature = signatures
                        .last()
                        .and_then(|s| Signature::from_str(&s.signature).ok());
                }
                Err(e) => {
                    log::error!("Error fetching signatures: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    })
}

fn transaction_fetcher(
    rpc_client: Arc<RpcClient>,
    signature_receiver: Receiver<Signature>,
    transaction_sender: Sender<(Signature, EncodedConfirmedTransactionWithStatusMeta)>,
    commitment: Option<CommitmentConfig>,
    max_concurrent_requests: usize,
) -> JoinHandle<()> {
    let rpc_client = Arc::clone(&rpc_client);
    let transaction_sender = transaction_sender.clone();
    let mut signature_receiver = signature_receiver;
    tokio::spawn(async move {
        let fetch_stream = async_stream::stream! {
            while let Some(signature) = signature_receiver.recv().await {
                yield signature;
            }
        };

        fetch_stream
            .map(|signature| {
                let rpc_client = Arc::clone(&rpc_client);
                async move {
                    match rpc_client
                        .get_transaction_with_config(
                            &signature,
                            RpcTransactionConfig {
                                encoding: Some(UiTransactionEncoding::Base64),
                                commitment: Some(
                                    commitment.unwrap_or(CommitmentConfig::confirmed()),
                                ),
                                max_supported_transaction_version: Some(0),
                                ..Default::default()
                            },
                        )
                        .await
                    {
                        Ok(tx) => Some((signature, tx)),
                        Err(e) => {
                            log::error!("Error fetching transaction {}: {:?}", signature, e);
                            None
                        }
                    }
                }
            })
            .buffer_unordered(max_concurrent_requests)
            .for_each(|result| async {
                if let Some((signature, fetched_transaction)) = result {
                    if let Err(e) = transaction_sender
                        .send((signature, fetched_transaction))
                        .await
                    {
                        log::error!("Failed to send transaction: {:?}", e);
                    }
                }
            })
            .await;
    })
}

fn processing_task(
    transaction_receiver: Receiver<(Signature, EncodedConfirmedTransactionWithStatusMeta)>,
    sender: mpsc::UnboundedSender<Update>,
    filters: Filters,
) -> JoinHandle<()> {
    let mut transaction_receiver = transaction_receiver;
    let sender = sender.clone();
    tokio::spawn(async move {
        while let Some((signature, fetched_transaction)) = transaction_receiver.recv().await {
            let transaction = fetched_transaction.transaction;

            let meta_original = if let Some(meta) = transaction.clone().meta {
                meta
            } else {
                log::warn!("Meta is malformed for transaction: {:?}", signature);
                continue;
            };

            if meta_original.status.is_err() {
                log::warn!(
                    "Transaction failed or encountered an error: {:?} (signature: {:?})",
                    meta_original.status,
                    signature
                );
                continue;
            }

            let Some(decoded_transaction) = transaction.transaction.decode() else {
                log::error!("Failed to decode transaction: {:?}", transaction);
                continue;
            };

            if let Some(accounts) = &filters.accounts {
                let account_set: HashSet<Pubkey> = accounts.iter().cloned().collect();

                let static_accounts = decoded_transaction.message.static_account_keys();

                let loaded_addresses =
                    meta_original
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
                                        let decoded_data =
                                            bs58::decode(compiled_ui_instruction.data.clone())
                                                .into_vec()
                                                .unwrap_or_else(|_| vec![]);
                                        InnerInstruction {
                                            instruction: CompiledInstruction {
                                                program_id_index: compiled_ui_instruction
                                                    .program_id_index,
                                                accounts: compiled_ui_instruction.accounts.clone(),
                                                data: decoded_data,
                                            },
                                            stack_height: compiled_ui_instruction.stack_height,
                                        }
                                    }
                                    _ => {
                                        log::error!("Unsupported instruction type encountered");
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
                    let loaded =
                        meta_original
                            .loaded_addresses
                            .unwrap_or_else(|| UiLoadedAddresses {
                                writable: vec![],
                                readonly: vec![],
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
                return_data: meta_original
                    .return_data
                    .map(|return_data| TransactionReturnData {
                        program_id: return_data.program_id.parse().unwrap_or_default(),
                        data: return_data.data.0.as_bytes().to_vec(),
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
                is_vote: false,
                slot: fetched_transaction.slot,
            });

            if let Err(e) = sender.send(update) {
                log::error!("Failed to send update: {:?}", e);
                continue;
            }
        }
    })
}
