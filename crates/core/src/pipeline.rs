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
use core::time;
use serde::de::DeserializeOwned;
use std::{sync::Arc, time::Instant};
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
    pub metrics: Vec<Arc<dyn Metrics>>,
    pub metrics_flush_interval: Option<u64>,
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
            metrics: Vec::new(),
            metrics_flush_interval: None,
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

        self.initialize_metrics().await?;
        let (update_sender, mut update_receiver) = tokio::sync::mpsc::unbounded_channel::<Update>();

        let datasource_cancellation_token = CancellationToken::new();

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

        let mut interval = tokio::time::interval(time::Duration::from_secs(
            self.metrics_flush_interval.unwrap_or(5),
        ));

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    log::trace!("Shutdown signal received, sending cancellation requests...");
                    datasource_cancellation_token.cancel();

                    if self.shutdown_strategy == ShutdownStrategy::Immediate {
                        log::trace!("Shutting down updates stream and processing.");
                        self.flush_metrics().await?;
                        self.shutdown_metrics().await?;
                        break;
                    } else {
                        log::trace!("Shutting down updates stream, continuing to process remaining updates.");
                    }
                }
                _ = interval.tick() => {
                    self.flush_metrics().await?;
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
                            log::trace!("All the updates were processed. Shutting down.");

                            self.flush_metrics().await?;
                            self.shutdown_metrics().await?;
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

                let instructions_with_metadata: Vec<(
                    InstructionMetadata,
                    solana_sdk::instruction::Instruction,
                )> = transformers::extract_instructions_with_metadata(
                    &transaction_metadata,
                    &transaction_update,
                )?;

                let nested_instructions =
                    transformers::nest_instructions(instructions_with_metadata);

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

    pub async fn initialize_metrics(&self) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.initialize().await?;
        }
        Ok(())
    }
    pub async fn flush_metrics(&self) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.flush().await?;
        }
        Ok(())
    }
    pub async fn shutdown_metrics(&self) -> CarbonResult<()> {
        for metrics in self.metrics.iter() {
            metrics.shutdown().await?;
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
    pub datasources: Vec<Arc<dyn Datasource + Send + Sync>>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub account_deletion_pipes: Vec<Box<dyn AccountDeletionPipes>>,
    pub instruction_pipes: Vec<Box<dyn for<'a> InstructionPipes<'a>>>,
    pub transaction_pipes: Vec<Box<dyn for<'a> TransactionPipes<'a>>>,
    pub metrics: Vec<Arc<dyn Metrics>>,
    pub metrics_flush_interval: Option<u64>,
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
            metrics: Vec::new(),
            metrics_flush_interval: None,
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
            datasources: self.datasources,
            account_pipes: self.account_pipes,
            account_deletion_pipes: self.account_deletion_pipes,
            instruction_pipes: self.instruction_pipes,
            transaction_pipes: self.transaction_pipes,
            shutdown_strategy: self.shutdown_strategy,
            metrics: self.metrics,
            metrics_flush_interval: self.metrics_flush_interval,
        })
    }
}
