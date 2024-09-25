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
    UiInstruction,
};
use std::str::FromStr;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone)]
pub struct Filter {
    pub accounts: Option<Vec<Pubkey>>,
    pub before_signature: Option<Signature>,   
    pub last_signature: Option<Signature>
} // can add transaction_types / token_mints etc

pub struct RpcTransactionCrawler {
    pub rpc_url: String,
    pub program_id: String,
    pub batch_limit: usize,
    pub polling_interval: u64,
    pub filters: Option<Filter>,
}
#[async_trait]
impl Datasource for RpcTransactionCrawler {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let rpc_url = self.rpc_url.clone();
        let program_id = self.program_id.clone();
        let batch_limit = self.batch_limit;
        let polling_interval = self.polling_interval;
        let filters = self.filters.clone();
        let sender = sender.clone();

        let abort_handle = tokio::spawn(async move {
            let rpc_client = RpcClient::new(&rpc_url);
            let program_pubkey = Pubkey::from_str(&program_id).unwrap();

            loop {
                let config = GetConfirmedSignaturesForAddress2Config {
                    limit: Some(batch_limit),
                    before: filters.as_ref().and_then(|f| f.before_signature.clone()),    
                    until: filters.as_ref().and_then(|f| f.last_signature.clone()),
                    commitment: Some(CommitmentConfig::confirmed()),
                };

                let signatures = rpc_client
                    .get_signatures_for_address_with_config(&program_pubkey, config)
                    .unwrap();

                for signature in signatures {
                    let transaction = rpc_client
                        .get_transaction(
                            &Signature::from_str(&signature.signature).unwrap(),
                            solana_transaction_status::UiTransactionEncoding::JsonParsed,
                        )
                        .unwrap();

                    let decoded_transaction = transaction.transaction.transaction.decode().unwrap();

                    let legacy_transaction = decoded_transaction.into_legacy_transaction().unwrap();

                    if let Some(ref filter) = filters {
                        if let Some(accounts) = &filter.accounts {
                            let transaction_accounts: Vec<Pubkey> =
                                legacy_transaction.message.account_keys.clone();
                            if !transaction_accounts
                                .iter()
                                .any(|account| accounts.contains(account))
                            {
                                continue;
                            }
                        }
                    }

                    let meta_original = transaction.transaction.meta.unwrap();

                    let meta_needed = TransactionStatusMeta {
                        status: meta_original.status,
                        fee: meta_original.fee,
                        pre_balances: meta_original.pre_balances,
                        post_balances: meta_original.post_balances,
                        inner_instructions: Some(
                            meta_original
                                .inner_instructions
                                .unwrap()
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
                                                        .unwrap(),
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
                        log_messages: Some(meta_original.log_messages.unwrap()),
                        pre_token_balances: Some(
                            meta_original
                                .pre_token_balances
                                .unwrap()
                                .iter()
                                .map(|tok| TransactionTokenBalance {
                                    account_index: tok.account_index,
                                    mint: tok.mint.clone(),
                                    ui_token_amount: tok.ui_token_amount.clone(),
                                    owner: tok.owner.clone().unwrap(),
                                    program_id: tok.program_id.clone().unwrap(),
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
                                .unwrap()
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
                            let loaded = meta_original.loaded_addresses.unwrap();
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
                        compute_units_consumed: Some(meta_original.compute_units_consumed.unwrap()),
                    };

                    let update = Update::Transaction(TransactionUpdate {
                        signature: Signature::from_str(&signature.signature).unwrap(),
                        transaction: transaction.transaction.transaction.decode().unwrap(),
                        meta: meta_needed,
                        is_vote: false,
                        slot: transaction.slot,
                    });

                    sender.send(update).unwrap();
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(polling_interval)).await;
            }
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}
