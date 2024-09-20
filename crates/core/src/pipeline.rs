use serde::de::DeserializeOwned;

use crate::{
    account::{AccountDecoder, AccountMetadata, AccountPipe, AccountPipes, DecodedAccount},
    collection::InstructionDecoderCollection,
    datasource::{Datasource, Update, UpdateType},
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
        let (update_sender, mut update_receiver) = tokio::sync::mpsc::unbounded_channel::<Update>();
        let _abort_handle = self.datasource.consume(&update_sender).await?;

        if !self.account_pipes.is_empty()
            && !self
                .datasource
                .update_types()
                .contains(&UpdateType::AccountUpdate)
        {
            return Err(Error::MissingUpdateTypeInDatasource(
                UpdateType::AccountUpdate,
            ));
        }

        if !self.instruction_pipes.is_empty()
            || !self.transaction_pipes.is_empty()
                && !self
                    .datasource
                    .update_types()
                    .contains(&UpdateType::Transaction)
        {
            return Err(Error::MissingUpdateTypeInDatasource(
                UpdateType::Transaction,
            ));
        }

        loop {
            match update_receiver.try_recv() {
                Ok(update) => match self.process(update.clone()).await {
                    Ok(_) => log::trace!("processed update"),
                    Err(error) => log::error!("error processing update: {:?}", error),
                },
                Err(error) => match error {
                    tokio::sync::mpsc::error::TryRecvError::Disconnected => {
                        break;
                    }
                    tokio::sync::mpsc::error::TryRecvError::Empty => {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        continue;
                    }
                },
            }
        }

        Ok(())
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
