use std::{default, sync::Arc};

use crate::{
    account::{AccountDecoder, AccountMetadata, AccountPipe, AccountPipes, DecodedAccount},
    account_deletion::{AccountDeletionPipe, AccountDeletionPipes},
    collection::InstructionDecoderCollection,
    datasource::{AccountDeletion, Datasource, Update, UpdateType},
    error::{CarbonResult, Error},
    instruction::{
        DecodedInstruction, InstructionDecoder, InstructionMetadata, InstructionPipe,
        InstructionPipes, NestedInstruction,
    },
    processor::Processor,
    schema::TransactionSchema,
    transaction::{TransactionPipe, TransactionPipes},
    transformers,
};
use serde::de::DeserializeOwned;
use tokio_util::sync::CancellationToken;

#[derive(Default, PartialEq)]
pub enum ShutdownStrategy {
    /// Stop the whole pipeline immediately.
    Immediate,
    /// Terminate the datasource(s) and finish processing all pending updates.
    #[default]
    ProcessPending,
}

pub struct Pipeline {
    pub datasources: Vec<Arc<dyn Datasource + Send + Sync>>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub account_deletion_pipes: Vec<Box<dyn AccountDeletionPipes>>,
    pub instruction_pipes: Vec<Box<dyn for<'a> InstructionPipes<'a>>>,
    pub transaction_pipes: Vec<Box<dyn for<'a> TransactionPipes<'a>>>,
    pub shutdown_strategy: ShutdownStrategy,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder {
            datasources: Vec::new(),
            account_pipes: Vec::new(),
            account_deletion_pipes: Vec::new(),
            instruction_pipes: Vec::new(),
            transaction_pipes: Vec::new(),
            shutdown_strategy: ShutdownStrategy::default(),
        }
    }

    pub async fn run(&mut self) -> CarbonResult<()> {
        let update_types: Vec<UpdateType> = self
            .datasources
            .iter()
            .map(|datasource| datasource.update_types())
            .flatten()
            .collect();

        if !self.account_pipes.is_empty() && !update_types.contains(&UpdateType::AccountUpdate) {
            return Err(Error::MissingUpdateTypeInDatasource(
                UpdateType::AccountUpdate,
            ));
        }

        if !self.account_deletion_pipes.is_empty()
            && !update_types.contains(&UpdateType::AccountDeletion)
        {
            return Err(Error::MissingUpdateTypeInDatasource(
                UpdateType::AccountDeletion,
            ));
        }

        if (!self.instruction_pipes.is_empty() || !self.transaction_pipes.is_empty())
            && !update_types.contains(&UpdateType::Transaction)
        {
            return Err(Error::MissingUpdateTypeInDatasource(
                UpdateType::Transaction,
            ));
        }

        let (update_sender, mut update_receiver) = tokio::sync::mpsc::unbounded_channel::<Update>();

        let datasource_cancellation_token = CancellationToken::new();
        let processing_cancellation_token = CancellationToken::new();

        for datasource in &self.datasources {
            let datasource_cancellation_token_clone = datasource_cancellation_token.clone();
            let sender_clone = update_sender.clone();
            let datasource_clone = Arc::clone(datasource);

            tokio::spawn(async move {
                if let Err(e) = datasource_clone
                    .consume(&sender_clone, datasource_cancellation_token_clone)
                    .await
                {
                    log::error!("Datasource consume error: {:?}", e);
                }
            });
        }

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    log::trace!("Shutdown signal received, sending cancellation requests and dropping the update sender...");
                    datasource_cancellation_token.cancel();
                    if self.shutdown_strategy != ShutdownStrategy::ProcessPending {
                        processing_cancellation_token.cancel();
                    }
                    drop(update_sender);
                    break;
                }
                recv_result = update_receiver.recv() => {
                    match recv_result {
                        Some(update) => {
                            if let Err(err) = self.process(update).await {
                                log::error!("Error processing datasource update: {:?}", err)
                            }
                        }
                        None => {
                            log::trace!("Pipeline update sender has been closed.");
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn process(&mut self, update: Update) -> CarbonResult<()> {
        match update {
            Update::Account(account_update) => {
                let account_metadata = AccountMetadata {
                    slot: account_update.slot,
                    pubkey: account_update.pubkey,
                };
                for pipe in self.account_pipes.iter_mut() {
                    pipe.run((account_metadata.clone(), account_update.account.clone()))
                        .await?;
                }
            }
            Update::Transaction(transaction_update) => {
                let transaction_metadata =
                    transformers::extract_transaction_metadata(&transaction_update)?;

                let instructions_with_metadata: Vec<(
                    InstructionMetadata,
                    solana_sdk::instruction::Instruction,
                )> = transformers::extract_instructions_with_metadata(
                    &transaction_metadata,
                    &transaction_update,
                )?;

                let nested_instructions =
                    transformers::nest_instructions(instructions_with_metadata);

                // TODO: Check if this or other way around
                for pipe in self.instruction_pipes.iter_mut() {
                    for nested_instruction in nested_instructions.iter().cloned() {
                        pipe.run(&nested_instruction).await?;
                    }
                }

                for pipe in self.transaction_pipes.iter_mut() {
                    pipe.run(&nested_instructions).await?;
                }
            }

            Update::AccountDeletion(account_deletion) => {
                for pipe in self.account_deletion_pipes.iter_mut() {
                    pipe.run(account_deletion.clone()).await?;
                }
            }
        };
        Ok(())
    }
}

pub struct PipelineBuilder {
    pub datasources: Vec<Arc<dyn Datasource + Send + Sync>>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub account_deletion_pipes: Vec<Box<dyn AccountDeletionPipes>>,
    pub instruction_pipes: Vec<Box<dyn for<'a> InstructionPipes<'a>>>,
    pub transaction_pipes: Vec<Box<dyn for<'a> TransactionPipes<'a>>>,
    pub shutdown_strategy: ShutdownStrategy,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            datasources: Vec::new(),
            account_pipes: Vec::new(),
            account_deletion_pipes: Vec::new(),
            instruction_pipes: Vec::new(),
            transaction_pipes: Vec::new(),
            shutdown_strategy: ShutdownStrategy::default(),
        }
    }

    pub fn datasource(mut self, datasource: impl Datasource + Send + Sync + 'static) -> Self {
        self.datasources.push(Arc::new(datasource));
        self
    }

    pub fn shutdown_strategy(mut self, shutdown_strategy: ShutdownStrategy) -> Self {
        self.shutdown_strategy = shutdown_strategy;
        self
    }

    pub fn account<T: Send + Sync + 'static>(
        mut self,
        decoder: impl for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static,
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

    pub fn account_deletions(
        mut self,
        processor: impl Processor<InputType = AccountDeletion> + Send + Sync + 'static,
    ) -> Self {
        self.account_deletion_pipes
            .push(Box::new(AccountDeletionPipe {
                processor: Box::new(processor),
            }));
        self
    }

    pub fn instruction<T: Send + Sync + 'static>(
        mut self,
        decoder: impl for<'a> InstructionDecoder<'a, InstructionType = T> + Send + Sync + 'static,
        processor: impl Processor<
                InputType = (
                    InstructionMetadata,
                    DecodedInstruction<T>,
                    Vec<NestedInstruction>,
                ),
            > + Send
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
            datasources: self.datasources,
            account_pipes: self.account_pipes,
            account_deletion_pipes: self.account_deletion_pipes,
            instruction_pipes: self.instruction_pipes,
            transaction_pipes: self.transaction_pipes,
            shutdown_strategy: self.shutdown_strategy,
        })
    }
}
