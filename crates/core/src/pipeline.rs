use crate::block_details::{BlockDetailsPipe, BlockDetailsPipes};
use crate::datasource::{BlockDetails, DatasourceId};
use crate::filter::Filter;
use {
    crate::{
        account::{
            AccountDecoder, AccountMetadata, AccountPipe, AccountPipes, AccountProcessorInputType,
        },
        account_deletion::{AccountDeletionPipe, AccountDeletionPipes},
        collection::InstructionDecoderCollection,
        datasource::{AccountDeletion, Datasource, Update},
        error::CarbonResult,
        instruction::{
            InstructionDecoder, InstructionPipe, InstructionPipes, InstructionProcessorInputType,
            InstructionsWithMetadata, NestedInstructions,
        },
        metrics::{Metrics, MetricsCollection},
        processor::Processor,
        schema::TransactionSchema,
        transaction::{TransactionPipe, TransactionPipes, TransactionProcessorInputType},
        transformers,
    },
    core::time,
    serde::de::DeserializeOwned,
    std::{convert::TryInto, sync::Arc, time::Instant},
    tokio_util::sync::CancellationToken,
};

#[derive(Default, PartialEq, Debug)]
pub enum ShutdownStrategy {
    Immediate,
    #[default]
    ProcessPending,
}

pub const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 1_000;

pub struct Pipeline {
    pub datasources: Vec<(DatasourceId, Arc<dyn Datasource + Send + Sync>)>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub account_deletion_pipes: Vec<Box<dyn AccountDeletionPipes>>,
    pub block_details_pipes: Vec<Box<dyn BlockDetailsPipes>>,
    pub instruction_pipes: Vec<Box<dyn for<'a> InstructionPipes<'a>>>,
    pub transaction_pipes: Vec<Box<dyn for<'a> TransactionPipes<'a>>>,
    pub metrics: Arc<MetricsCollection>,
    pub metrics_flush_interval: Option<u64>,
    pub datasource_cancellation_token: Option<CancellationToken>,
    pub shutdown_strategy: ShutdownStrategy,
    pub channel_buffer_size: usize,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        log::trace!("Pipeline::builder()");
        PipelineBuilder {
            datasources: Vec::new(),
            account_pipes: Vec::new(),
            account_deletion_pipes: Vec::new(),
            block_details_pipes: Vec::new(),
            instruction_pipes: Vec::new(),
            transaction_pipes: Vec::new(),
            metrics: MetricsCollection::default(),
            metrics_flush_interval: None,
            datasource_cancellation_token: None,
            shutdown_strategy: ShutdownStrategy::default(),
            channel_buffer_size: DEFAULT_CHANNEL_BUFFER_SIZE,
        }
    }

    pub async fn run(&mut self) -> CarbonResult<()> {
        log::info!("starting pipeline. num_datasources: {}, num_metrics: {}, num_account_pipes: {}, num_account_deletion_pipes: {}, num_instruction_pipes: {}, num_transaction_pipes: {}",
            self.datasources.len(),
            self.metrics.metrics.len(),
            self.account_pipes.len(),
            self.account_deletion_pipes.len(),
            self.instruction_pipes.len(),
            self.transaction_pipes.len(),
        );

        log::trace!("run(self)");

        self.metrics.initialize_metrics().await?;
        let (update_sender, mut update_receiver) =
            tokio::sync::mpsc::channel::<(Update, DatasourceId)>(self.channel_buffer_size);

        let datasource_cancellation_token = self
            .datasource_cancellation_token
            .clone()
            .unwrap_or_default();

        for datasource in &self.datasources {
            let datasource_cancellation_token_clone = datasource_cancellation_token.clone();
            let sender_clone = update_sender.clone();
            let datasource_clone = Arc::clone(&datasource.1);
            let datasource_id = datasource.0.clone();
            let metrics_collection = self.metrics.clone();

            tokio::spawn(async move {
                if let Err(e) = datasource_clone
                    .consume(
                        datasource_id,
                        sender_clone,
                        datasource_cancellation_token_clone,
                        metrics_collection,
                    )
                    .await
                {
                    log::error!("error consuming datasource: {e:?}");
                }
            });
        }

        drop(update_sender);

        let mut interval = tokio::time::interval(time::Duration::from_secs(
            self.metrics_flush_interval.unwrap_or(5),
        ));

        loop {
            tokio::select! {
                _ = datasource_cancellation_token.cancelled() => {
                    log::trace!("datasource cancellation token cancelled, shutting down.");
                    self.metrics.flush_metrics().await?;
                    self.metrics.shutdown_metrics().await?;
                    break;
                }
                _ = tokio::signal::ctrl_c() => {
                    log::trace!("received SIGINT, shutting down.");
                    datasource_cancellation_token.cancel();

                    if self.shutdown_strategy == ShutdownStrategy::Immediate {
                        log::info!("shutting down the pipeline immediately.");
                        self.metrics.flush_metrics().await?;
                        self.metrics.shutdown_metrics().await?;
                        break;
                    } else {
                        log::info!("shutting down the pipeline after processing pending updates.");
                    }
                }
                _ = interval.tick() => {
                    self.metrics.flush_metrics().await?;
                }
                update = update_receiver.recv() => {
                    match update {
                        Some((update, datasource_id)) => {
                            self
                                .metrics.increment_counter("updates_received", 1)
                                .await?;

                            let start = Instant::now();
                            let process_result = self.process(update.clone(), datasource_id.clone()).await;
                            let time_taken_nanoseconds = start.elapsed().as_nanos();
                            let time_taken_milliseconds = time_taken_nanoseconds / 1_000_000;

                            self
                                .metrics
                                .record_histogram("updates_process_time_nanoseconds", time_taken_nanoseconds as f64)
                                .await?;

                            self
                                .metrics
                                .record_histogram("updates_process_time_milliseconds", time_taken_milliseconds as f64)
                                .await?;

                            match process_result {
                                Ok(_) => {
                                    self
                                        .metrics.increment_counter("updates_successful", 1)
                                        .await?;

                                    log::trace!("processed update")
                                }
                                Err(error) => {
                                    log::error!("error processing update ({update:?}): {error:?}");
                                    self.metrics.increment_counter("updates_failed", 1).await?;
                                }
                            };

                            self
                                .metrics.increment_counter("updates_processed", 1)
                                .await?;

                            self
                                .metrics.update_gauge("updates_queued", update_receiver.len() as f64)
                                .await?;
                        }
                        None => {
                            log::info!("update_receiver closed, shutting down.");
                            self.metrics.flush_metrics().await?;
                            self.metrics.shutdown_metrics().await?;
                            break;
                        }
                    }
                }
            }
        }

        log::info!("pipeline shutdown complete.");

        Ok(())
    }

    async fn process(&mut self, update: Update, datasource_id: DatasourceId) -> CarbonResult<()> {
        log::trace!("process(self, update: {update:?}, datasource_id: {datasource_id:?})");
        match update {
            Update::Account(account_update) => {
                let account_metadata = AccountMetadata {
                    slot: account_update.slot,
                    pubkey: account_update.pubkey,
                    transaction_signature: account_update.transaction_signature,
                };

                for pipe in self.account_pipes.iter_mut() {
                    if pipe.filters().iter().all(|filter| {
                        filter.filter_account(
                            &datasource_id,
                            &account_metadata,
                            &account_update.account,
                        )
                    }) {
                        pipe.run(
                            (account_metadata.clone(), account_update.account.clone()),
                            self.metrics.clone(),
                        )
                        .await?;
                    }
                }

                self.metrics
                    .increment_counter("account_updates_processed", 1)
                    .await?;
            }
            Update::Transaction(transaction_update) => {
                let transaction_metadata = Arc::new((*transaction_update).clone().try_into()?);

                let instructions_with_metadata: InstructionsWithMetadata =
                    transformers::extract_instructions_with_metadata(
                        &transaction_metadata,
                        &transaction_update,
                    )?;

                let nested_instructions: NestedInstructions = instructions_with_metadata.into();

                for pipe in self.instruction_pipes.iter_mut() {
                    for nested_instruction in nested_instructions.iter() {
                        if pipe.filters().iter().all(|filter| {
                            filter.filter_instruction(&datasource_id, nested_instruction)
                        }) {
                            pipe.run(nested_instruction, self.metrics.clone()).await?;
                        }
                    }
                }

                for pipe in self.transaction_pipes.iter_mut() {
                    if pipe.filters().iter().all(|filter| {
                        filter.filter_transaction(
                            &datasource_id,
                            &transaction_metadata,
                            &nested_instructions,
                        )
                    }) {
                        pipe.run(
                            transaction_metadata.clone(),
                            &nested_instructions,
                            self.metrics.clone(),
                        )
                        .await?;
                    }
                }

                self.metrics
                    .increment_counter("transaction_updates_processed", 1)
                    .await?;
            }
            Update::AccountDeletion(account_deletion) => {
                for pipe in self.account_deletion_pipes.iter_mut() {
                    if pipe.filters().iter().all(|filter| {
                        filter.filter_account_deletion(&datasource_id, &account_deletion)
                    }) {
                        pipe.run(account_deletion.clone(), self.metrics.clone())
                            .await?;
                    }
                }

                self.metrics
                    .increment_counter("account_deletions_processed", 1)
                    .await?;
            }
            Update::BlockDetails(block_details) => {
                for pipe in self.block_details_pipes.iter_mut() {
                    if pipe
                        .filters()
                        .iter()
                        .all(|filter| filter.filter_block_details(&datasource_id, &block_details))
                    {
                        pipe.run(block_details.clone(), self.metrics.clone())
                            .await?;
                    }
                }

                self.metrics
                    .increment_counter("block_details_processed", 1)
                    .await?;
            }
        };

        Ok(())
    }
}

#[derive(Default)]
pub struct PipelineBuilder {
    pub datasources: Vec<(DatasourceId, Arc<dyn Datasource + Send + Sync>)>,
    pub account_pipes: Vec<Box<dyn AccountPipes>>,
    pub account_deletion_pipes: Vec<Box<dyn AccountDeletionPipes>>,
    pub block_details_pipes: Vec<Box<dyn BlockDetailsPipes>>,
    pub instruction_pipes: Vec<Box<dyn for<'a> InstructionPipes<'a>>>,
    pub transaction_pipes: Vec<Box<dyn for<'a> TransactionPipes<'a>>>,
    pub metrics: MetricsCollection,
    pub metrics_flush_interval: Option<u64>,
    pub datasource_cancellation_token: Option<CancellationToken>,
    pub shutdown_strategy: ShutdownStrategy,
    pub channel_buffer_size: usize,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        log::trace!("PipelineBuilder::new()");
        Self::default()
    }

    pub fn datasource(mut self, datasource: impl Datasource + 'static) -> Self {
        log::trace!("datasource(self, datasource: {:?})", stringify!(datasource));
        self.datasources
            .push((DatasourceId::new_unique(), Arc::new(datasource)));
        self
    }

    pub fn datasource_with_id(
        mut self,
        datasource: impl Datasource + 'static,
        id: DatasourceId,
    ) -> Self {
        log::trace!(
            "datasource_with_id(self, id: {:?}, datasource: {:?})",
            id,
            stringify!(datasource)
        );
        self.datasources.push((id, Arc::new(datasource)));
        self
    }

    pub fn shutdown_strategy(mut self, shutdown_strategy: ShutdownStrategy) -> Self {
        log::trace!("shutdown_strategy(self, shutdown_strategy: {shutdown_strategy:?})");
        self.shutdown_strategy = shutdown_strategy;
        self
    }

    pub fn account<T: Send + Sync + 'static>(
        mut self,
        decoder: impl for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static,
        processor: impl Processor<InputType = AccountProcessorInputType<T>> + Send + Sync + 'static,
    ) -> Self {
        log::trace!(
            "account(self, decoder: {:?}, processor: {:?})",
            stringify!(decoder),
            stringify!(processor)
        );
        self.account_pipes.push(Box::new(AccountPipe {
            decoder: Box::new(decoder),
            processor: Box::new(processor),
            filters: vec![],
        }));
        self
    }

    pub fn account_with_filters<T: Send + Sync + 'static>(
        mut self,
        decoder: impl for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static,
        processor: impl Processor<InputType = AccountProcessorInputType<T>> + Send + Sync + 'static,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self {
        log::trace!(
            "account_with_filters(self, decoder: {:?}, processor: {:?}, filters: {:?})",
            stringify!(decoder),
            stringify!(processor),
            stringify!(filters)
        );
        self.account_pipes.push(Box::new(AccountPipe {
            decoder: Box::new(decoder),
            processor: Box::new(processor),
            filters,
        }));
        self
    }

    pub fn account_deletions(
        mut self,
        processor: impl Processor<InputType = AccountDeletion> + Send + Sync + 'static,
    ) -> Self {
        log::trace!(
            "account_deletions(self, processor: {:?})",
            stringify!(processor)
        );
        self.account_deletion_pipes
            .push(Box::new(AccountDeletionPipe {
                processor: Box::new(processor),
                filters: vec![],
            }));
        self
    }

    pub fn account_deletions_with_filters(
        mut self,
        processor: impl Processor<InputType = AccountDeletion> + Send + Sync + 'static,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self {
        log::trace!(
            "account_deletions_with_filters(self, processor: {:?}, filters: {:?})",
            stringify!(processor),
            stringify!(filters)
        );
        self.account_deletion_pipes
            .push(Box::new(AccountDeletionPipe {
                processor: Box::new(processor),
                filters,
            }));
        self
    }

    pub fn block_details(
        mut self,
        processor: impl Processor<InputType = BlockDetails> + Send + Sync + 'static,
    ) -> Self {
        log::trace!(
            "block_details(self, processor: {:?})",
            stringify!(processor)
        );
        self.block_details_pipes.push(Box::new(BlockDetailsPipe {
            processor: Box::new(processor),
            filters: vec![],
        }));
        self
    }

    pub fn block_details_with_filters(
        mut self,
        processor: impl Processor<InputType = BlockDetails> + Send + Sync + 'static,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self {
        log::trace!(
            "block_details_with_filters(self, processor: {:?}, filters: {:?})",
            stringify!(processor),
            stringify!(filters)
        );
        self.block_details_pipes.push(Box::new(BlockDetailsPipe {
            processor: Box::new(processor),
            filters,
        }));
        self
    }

    pub fn instruction<T, P>(
        mut self,
        decoder: impl for<'a> InstructionDecoder<'a, InstructionType = T> + Send + Sync + 'static,
        processor: P,
    ) -> Self
    where
        T: Send + Sync + 'static,
        P: Processor<InputType = InstructionProcessorInputType<T>> + Send + Sync + 'static,
    {
        log::trace!(
            "instruction(self, decoder: {:?}, processor: {:?})",
            stringify!(decoder),
            stringify!(processor)
        );
        self.instruction_pipes.push(Box::new(InstructionPipe {
            decoder: Box::new(decoder),
            processor,
            filters: vec![],
        }));
        self
    }

    pub fn instruction_with_filters<T, P>(
        mut self,
        decoder: impl for<'a> InstructionDecoder<'a, InstructionType = T> + Send + Sync + 'static,
        processor: P,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self
    where
        T: Send + Sync + crate::deserialize::ArrangeAccounts + 'static,
        P: Processor<InputType = InstructionProcessorInputType<T>> + Send + Sync + 'static,
    {
        log::trace!(
            "instruction_with_filters(self, decoder: {:?}, processor: {:?}, filters: {:?})",
            stringify!(decoder),
            stringify!(processor),
            stringify!(filters)
        );
        self.instruction_pipes.push(Box::new(InstructionPipe {
            decoder: Box::new(decoder),
            processor,
            filters,
        }));
        self
    }

    pub fn transaction<T, U>(
        mut self,
        processor: impl Processor<InputType = TransactionProcessorInputType<T, U>>
            + Send
            + Sync
            + 'static,
        schema: Option<TransactionSchema<T>>,
    ) -> Self
    where
        T: InstructionDecoderCollection + 'static,
        U: DeserializeOwned + Send + Sync + 'static,
    {
        log::trace!(
            "transaction(self, schema: {:?}, processor: {:?})",
            stringify!(schema),
            stringify!(processor)
        );
        self.transaction_pipes
            .push(Box::new(TransactionPipe::<T, U>::new(
                schema,
                processor,
                vec![],
            )));
        self
    }

    pub fn transaction_with_filters<T, U>(
        mut self,
        processor: impl Processor<InputType = TransactionProcessorInputType<T, U>>
            + Send
            + Sync
            + 'static,
        schema: Option<TransactionSchema<T>>,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self
    where
        T: InstructionDecoderCollection + 'static,
        U: DeserializeOwned + Send + Sync + 'static,
    {
        log::trace!(
            "transaction_with_filters(self, schema: {:?}, processor: {:?}, filters: {:?})",
            stringify!(schema),
            stringify!(processor),
            stringify!(filters)
        );
        self.transaction_pipes
            .push(Box::new(TransactionPipe::<T, U>::new(
                schema, processor, filters,
            )));
        self
    }

    pub fn metrics(mut self, metrics: Arc<dyn Metrics>) -> Self {
        log::trace!("metrics(self, metrics: {:?})", stringify!(metrics));
        self.metrics.metrics.push(metrics);
        self
    }

    pub fn metrics_flush_interval(mut self, interval: u64) -> Self {
        log::trace!("metrics_flush_interval(self, interval: {interval:?})");
        self.metrics_flush_interval = Some(interval);
        self
    }

    pub fn datasource_cancellation_token(mut self, cancellation_token: CancellationToken) -> Self {
        log::trace!(
            "datasource_cancellation_token(self, cancellation_token: {cancellation_token:?})"
        );
        self.datasource_cancellation_token = Some(cancellation_token);
        self
    }

    pub fn channel_buffer_size(mut self, size: usize) -> Self {
        log::trace!("channel_buffer_size(self, size: {size:?})");
        self.channel_buffer_size = size;
        self
    }

    pub fn build(self) -> CarbonResult<Pipeline> {
        log::trace!("build(self)");
        Ok(Pipeline {
            datasources: self.datasources,
            account_pipes: self.account_pipes,
            account_deletion_pipes: self.account_deletion_pipes,
            block_details_pipes: self.block_details_pipes,
            instruction_pipes: self.instruction_pipes,
            transaction_pipes: self.transaction_pipes,
            shutdown_strategy: self.shutdown_strategy,
            metrics: Arc::new(self.metrics),
            metrics_flush_interval: self.metrics_flush_interval,
            datasource_cancellation_token: self.datasource_cancellation_token,
            channel_buffer_size: self.channel_buffer_size,
        })
    }
}
