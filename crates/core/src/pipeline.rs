use core::time;
use std::{sync::Arc, time::Instant};

use serde::de::DeserializeOwned;

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
    metrics::Metrics,
    processor::Processor,
    schema::TransactionSchema,
    transaction::{TransactionPipe, TransactionPipes},
    transformers,
};

pub struct Pipeline {
    pub datasource: Box<dyn Datasource>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub account_deletion_pipes: Vec<Box<dyn AccountDeletionPipes>>,
    pub instruction_pipes: Vec<Box<dyn for<'a> InstructionPipes<'a>>>,
    pub transaction_pipes: Vec<Box<dyn for<'a> TransactionPipes<'a>>>,
    pub metrics: Vec<Arc<dyn Metrics>>,
    pub metrics_flush_interval: Option<u64>,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder {
            datasource: None,
            account_pipes: Vec::new(),
            account_deletion_pipes: Vec::new(),
            instruction_pipes: Vec::new(),
            transaction_pipes: Vec::new(),
            metrics: Vec::new(),
            metrics_flush_interval: None,
        }
    }

    pub async fn run(&mut self) -> CarbonResult<()> {
        self.initialize().await?;

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

        if (!self.instruction_pipes.is_empty() || !self.transaction_pipes.is_empty())
            && !self
                .datasource
                .update_types()
                .contains(&UpdateType::Transaction)
        {
            return Err(Error::MissingUpdateTypeInDatasource(
                UpdateType::Transaction,
            ));
        }

        let mut interval = tokio::time::interval(time::Duration::from_secs(
            self.metrics_flush_interval.unwrap_or(5),
        ));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.flush().await?;
                }
                update = update_receiver.recv() => {
                    match update {
                        Some(update) => {
                            self
                                .increment_counter("updates_received", 1)
                                .await?;

                            let start = Instant::now();
                            let process_result = self.process(update.clone()).await;
                            let time_taken = start.elapsed().as_millis();


                            self
                                .record_histogram("updates_processing_times", time_taken as f64)
                                .await?;

                            match process_result {
                                Ok(_) => {
                                    self
                                        .increment_counter("updates_successful", 1)
                                        .await?;

                                    log::trace!("processed update")
                                }
                                Err(error) => {
                                    self.increment_counter("updates_failed", 1).await?;

                                    log::error!("error processing update: {:?}", error)
                                }
                            };

                            self
                                .increment_counter("updates_processed", 1)
                                .await?;

                            self
                                .update_gauge("updates_queued", update_receiver.len() as f64)
                                .await?;
                        }

                        None => {
                            self.increment_counter("updates_failed", 1).await?;
                            log::error!("error processing update");

                            break
                        }
                    }

                }
            }
        }

        Ok(())
    }

    pub async fn process(&mut self, update: Update) -> CarbonResult<()> {
        match update {
            Update::Account(account_update) => {
                let account_metadata = AccountMetadata {
                    slot: account_update.slot,
                    pubkey: account_update.pubkey,
                };

                for pipe in self.account_pipes.iter_mut() {
                    pipe.run(
                        (account_metadata.clone(), account_update.account.clone()),
                        self.metrics.clone(),
                    )
                    .await?;
                }

                self.increment_counter("account_updates_processed", 1)
                    .await?;
            }
            Update::Transaction(transaction_update) => {
                let transaction_metadata =
                    transformers::extract_transaction_metadata(&transaction_update)?;

                let instructions_with_metadata = transformers::extract_instructions_with_metadata(
                    &transaction_metadata,
                    &transaction_update,
                )?;

                let nested_instructions =
                    transformers::nest_instructions(instructions_with_metadata);

                // TODO: Check if this or other way around
                for pipe in self.instruction_pipes.iter_mut() {
                    for nested_instruction in nested_instructions.iter().cloned() {
                        pipe.run(&nested_instruction, self.metrics.clone()).await?;
                    }
                }

                for pipe in self.transaction_pipes.iter_mut() {
                    pipe.run(&nested_instructions, self.metrics.clone()).await?;
                }

                self.increment_counter("transaction_updates_processed", 1)
                    .await?;
            }
            Update::AccountDeletion(account_deletion) => {
                for pipe in self.account_deletion_pipes.iter_mut() {
                    pipe.run(account_deletion.clone(), self.metrics.clone())
                        .await?;
                }

                self.increment_counter("account_deletions_processed", 1)
                    .await?;
            }
        };

        Ok(())
    }

    pub async fn initialize(&self) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.initialize_metrics().await?;
        }
        Ok(())
    }
    pub async fn flush(&self) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.flush_metrics().await?;
        }
        Ok(())
    }
    pub async fn shutdown(&self) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.shutdown_metrics().await?;
        }
        Ok(())
    }

    pub async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.update_gauge(name, value).await?;
        }
        Ok(())
    }
    pub async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.increment_counter(name, value).await?;
        }
        Ok(())
    }
    pub async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.record_histogram(name, value).await?;
        }
        Ok(())
    }
}

pub struct PipelineBuilder {
    pub datasource: Option<Box<dyn Datasource>>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub account_deletion_pipes: Vec<Box<dyn AccountDeletionPipes>>,
    pub instruction_pipes: Vec<Box<dyn for<'a> InstructionPipes<'a>>>,
    pub transaction_pipes: Vec<Box<dyn for<'a> TransactionPipes<'a>>>,
    pub metrics: Vec<Arc<dyn Metrics>>,
    pub metrics_flush_interval: Option<u64>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            datasource: None,
            account_pipes: Vec::new(),
            account_deletion_pipes: Vec::new(),
            instruction_pipes: Vec::new(),
            transaction_pipes: Vec::new(),
            metrics: Vec::new(),
            metrics_flush_interval: None,
        }
    }

    pub fn datasource(mut self, datasource: impl Datasource + 'static) -> Self {
        self.datasource = Some(Box::new(datasource));
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

    pub fn metrics(mut self, metrics: Arc<dyn Metrics>) -> Self {
        self.metrics.push(metrics);
        self
    }

    pub fn metrics_flush_interval(mut self, interval: u64) -> Self {
        self.metrics_flush_interval = Some(interval);
        self
    }

    pub fn build(self) -> CarbonResult<Pipeline> {
        Ok(Pipeline {
            datasource: self.datasource.ok_or(Error::MissingDatasource)?,
            account_pipes: self.account_pipes,
            account_deletion_pipes: self.account_deletion_pipes,
            instruction_pipes: self.instruction_pipes,
            transaction_pipes: self.transaction_pipes,
            metrics: self.metrics,
            metrics_flush_interval: self.metrics_flush_interval,
        })
    }
}
