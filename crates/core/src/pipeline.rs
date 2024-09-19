use std::str::FromStr;

use serde::de::DeserializeOwned;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::CompiledInstruction,
    message::v0::LoadedAddresses, pubkey::Pubkey, signature::Signature,
};
use solana_transaction_status::{
    InnerInstruction, InnerInstructions, Reward, TransactionStatusMeta, TransactionTokenBalance,
    UiInstruction,
};

use crate::{
    account::{AccountDecoder, AccountMetadata, AccountPipe, AccountPipes, DecodedAccount},
    collection::InstructionDecoderCollection,
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::{CarbonResult, Error},
    instruction::{
        DecodedInstruction, InstructionDecoder, InstructionMetadata, InstructionPipe,
        InstructionPipes,
    },
    processor::Processor,
    schema::TransactionSchema,
    transaction::{TransactionPipe, TransactionPipes},
    transformers,
};

pub struct Pipeline {
    pub datasource: Box<dyn Datasource>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub instruction_pipes: Vec<Box<dyn InstructionPipes>>,
    pub transaction_pipes: Vec<Box<dyn TransactionPipes>>,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder {
            datasource: None,
            account_pipes: Vec::new(),
            instruction_pipes: Vec::new(),
            transaction_pipes: Vec::new(),
        }
    }

    pub async fn run(&self) -> CarbonResult<()> {
        // let (update_sender, mut update_receiver) = tokio::sync::mpsc::unbounded_channel::<Update>();
        // let _abort_handle = self.datasource.consume(&update_sender).await?;

        // if !self.account_pipes.is_empty()
        //     && !self
        //         .datasource
        //         .update_types()
        //         .contains(&UpdateType::AccountUpdate)
        // {
        //     return Err(Error::MissingUpdateTypeInDatasource(
        //         UpdateType::AccountUpdate,
        //     ));
        // }

        // if !self.instruction_pipes.is_empty()
        //     || !self.transaction_pipes.is_empty()
        //         && !self
        //             .datasource
        //             .update_types()
        //             .contains(&UpdateType::Transaction)
        // {
        //     return Err(Error::MissingUpdateTypeInDatasource(
        //         UpdateType::Transaction,
        //     ));
        // }

        // loop {
        //     match update_receiver.try_recv() {
        //         Ok(update) => match self.process(update.clone()).await {
        //             Ok(_) => log::trace!("processed update"),
        //             Err(error) => log::error!("error processing update: {:?}", error),
        //         },
        //         Err(error) => match error {
        //             tokio::sync::mpsc::error::TryRecvError::Disconnected => {
        //                 break;
        //             }
        //             tokio::sync::mpsc::error::TryRecvError::Empty => {
        //                 tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        //                 continue;
        //             }
        //         },
        //     }
        // }

        // Ok(())

        // FOR TESTING

        let rpc_client =
            RpcClient::new_with_commitment("TODO".to_string(), CommitmentConfig::confirmed());

        let tx = rpc_client.get_transaction(&Signature::from_str("Xw9nEnKJMna6S7dfq2cnaZtUypW7MSYVWarUSEfjoRWaWP7K7ytDLkH4QD1w4jWzUbNL6FuT8DuvkKnUAcuFw6x").unwrap(),solana_transaction_status::UiTransactionEncoding::Base58).ok().unwrap();

        let meta_original = tx.transaction.meta.unwrap();

        let meta_needed = TransactionStatusMeta {
            status: meta_original.status,
            fee: meta_original.fee,
            pre_balances: meta_original.pre_balances,
            post_balances: meta_original.post_balances,
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
                                            accounts: ui_compiled_instruction.accounts.clone(),
                                            data: bs58::decode(
                                                ui_compiled_instruction.data.clone(),
                                            )
                                            .into_vec()
                                            .unwrap(),
                                        },
                                        stack_height: ui_compiled_instruction.stack_height,
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
            return_data: None,
            compute_units_consumed: Some(meta_original.compute_units_consumed.unwrap()),
        };

        let update = Update::Transaction(TransactionUpdate {
            signature: Signature::from_str("Xw9nEnKJMna6S7dfq2cnaZtUypW7MSYVWarUSEfjoRWaWP7K7ytDLkH4QD1w4jWzUbNL6FuT8DuvkKnUAcuFw6x").unwrap(),
            transaction: tx.transaction.transaction.decode().unwrap(),
            meta: meta_needed,
            is_vote: false,
            slot: tx.slot,
        });

        self.process(update.clone()).await
    }

    pub async fn process(&self, update: Update) -> CarbonResult<()> {
        match update {
            Update::Account(account_update) => {
                let account_metadata = AccountMetadata {
                    slot: account_update.slot,
                    pubkey: account_update.pubkey,
                };
                for pipe in self.account_pipes.iter() {
                    pipe.run((account_metadata.clone(), account_update.account.clone()))
                        .await?;
                }
            }
            Update::Transaction(transaction_update) => {
                let transaction_metadata =
                    transformers::extract_transaction_metadata(&transaction_update)?;

                let instructions_with_metadata = transformers::extract_instructions_with_metadata(
                    &transaction_metadata,
                    &transaction_update,
                )?;

                for ix in instructions_with_metadata.clone() {
                    println!(
                        "\n\n\n instruction: \n{:?} \n{:?} \n",
                        ix.1, ix.0.stack_height
                    );
                }

                for instruction in instructions_with_metadata.iter().cloned() {
                    println!(
                        "\n\nInstruction stack height {:?}\n",
                        instruction.0.stack_height
                    );
                    for pipe in self.instruction_pipes.iter() {
                        pipe.run(instruction.clone()).await?;
                    }
                }

                // ive changed this also to work but no idea whats going on as of right now and if its good
                let nested_instructions =
                    transformers::nest_instructions(instructions_with_metadata);

                for nest in nested_instructions.clone() {
                    println!(
                        "\n\n\n nest: \n{:?} \n{:?} \n{:?}",
                        nest.instruction, nest.metadata.stack_height, nest.inner_instructions,
                    );
                }

                for pipe in self.transaction_pipes.iter() {
                    pipe.run(nested_instructions.clone()).await?;
                }
            }
        };
        Ok(())
    }
}

pub struct PipelineBuilder {
    pub datasource: Option<Box<dyn Datasource>>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub instruction_pipes: Vec<Box<dyn InstructionPipes>>,
    pub transaction_pipes: Vec<Box<dyn TransactionPipes>>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            datasource: None,
            account_pipes: Vec::new(),
            instruction_pipes: Vec::new(),
            transaction_pipes: Vec::new(),
        }
    }

    pub fn datasource(mut self, datasource: impl Datasource + 'static) -> Self {
        self.datasource = Some(Box::new(datasource));
        self
    }

    pub fn account<T: Send + Sync + 'static>(
        mut self,
        decoder: impl AccountDecoder<AccountType = T> + Send + Sync + 'static,
        processor: impl Processor<InputType = (AccountMetadata, DecodedAccount<T>)>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        self.account_pipes.push(Box::new(AccountPipe {
            decoder: Box::new(decoder),
            processor: Box::new(processor),
        }));
        self
    }

    pub fn instruction<T: Send + Sync + 'static>(
        mut self,
        decoder: impl InstructionDecoder<InstructionType = T> + Send + Sync + 'static,
        processor: impl Processor<InputType = (InstructionMetadata, DecodedInstruction<T>)>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        self.instruction_pipes.push(Box::new(InstructionPipe {
            decoder: Box::new(decoder),
            processor: Box::new(processor),
        }));
        self
    }

    pub fn transaction<T, U>(
        mut self,
        schema: TransactionSchema<T>,
        processor: impl Processor<InputType = U> + Send + Sync + 'static,
    ) -> Self
    where
        T: InstructionDecoderCollection + 'static,
        U: DeserializeOwned + Send + Sync + 'static,
    {
        self.transaction_pipes
            .push(Box::new(TransactionPipe::<T, U>::new(schema, processor)));
        self
    }

    pub fn build(self) -> CarbonResult<Pipeline> {
        Ok(Pipeline {
            datasource: self.datasource.ok_or(Error::MissingDatasource)?,
            account_pipes: self.account_pipes,
            instruction_pipes: self.instruction_pipes,
            transaction_pipes: self.transaction_pipes,
        })
    }
}
