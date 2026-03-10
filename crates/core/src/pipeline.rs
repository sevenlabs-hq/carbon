use crate::block_details::{BlockDetailsPipe, BlockDetailsPipes};
use crate::datasource::{BlockDetails, DatasourceId};
use crate::filter::{Filter, FilterContext, FilterResult};
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
        metrics::{Counter, Gauge, Histogram, MetricsExporter, MetricsRegistry},
        processor::Processor,
        transaction::{TransactionPipe, TransactionPipes, TransactionProcessorInputType},
        transformers,
    },
    std::{
        convert::TryInto,
        sync::{Arc, LazyLock},
        time::Instant,
    },
    tokio_util::sync::CancellationToken,
};

static UPDATES_RECEIVED: Counter = Counter::new(
    "carbon_updates_received_total",
    "Total updates pulled from datasources",
);

static UPDATES_PROCESSED: Counter = Counter::new(
    "carbon_updates_processed_total",
    "Total updates processed by the pipeline",
);

static UPDATES_SUCCESSFUL: Counter = Counter::new(
    "carbon_updates_successful_total",
    "Updates processed without error",
);

static UPDATES_FAILED: Counter = Counter::new(
    "carbon_updates_failed_total",
    "Updates that errored during processing",
);

static UPDATES_QUEUED: Gauge = Gauge::new(
    "carbon_updates_queued",
    "Current number of updates waiting in queue",
);

static ACCOUNT_UPDATES_PROCESSED: Counter = Counter::new(
    "carbon_account_updates_processed_total",
    "Total account updates processed",
);

static TRANSACTION_UPDATES_PROCESSED: Counter = Counter::new(
    "carbon_transaction_updates_processed_total",
    "Total transaction updates processed",
);

static ACCOUNT_DELETIONS_PROCESSED: Counter = Counter::new(
    "carbon_account_deletions_processed_total",
    "Total account deletions processed",
);

static BLOCK_DETAILS_PROCESSED: Counter = Counter::new(
    "carbon_block_details_processed_total",
    "Total block details processed",
);

static PROCESSING_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "carbon_updates_process_time_nanoseconds",
        "Time taken to process updates in nanoseconds",
        vec![
            1_000.0,
            10_000.0,
            100_000.0,
            1_000_000.0,
            10_000_000.0,
            100_000_000.0,
            1_000_000_000.0,
        ],
    )
});
static PROCESSING_TIME_MILLIS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "carbon_updates_process_time_milliseconds",
        "Time taken to process updates in milliseconds",
        vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0],
    )
});

fn register_pipeline_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&UPDATES_RECEIVED);
    registry.register_counter(&UPDATES_PROCESSED);
    registry.register_counter(&UPDATES_SUCCESSFUL);
    registry.register_counter(&UPDATES_FAILED);
    registry.register_gauge(&UPDATES_QUEUED);
    registry.register_histogram(&PROCESSING_TIME_NANOS);
    registry.register_histogram(&PROCESSING_TIME_MILLIS);
    registry.register_counter(&ACCOUNT_UPDATES_PROCESSED);
    registry.register_counter(&TRANSACTION_UPDATES_PROCESSED);
    registry.register_counter(&ACCOUNT_DELETIONS_PROCESSED);
    registry.register_counter(&BLOCK_DETAILS_PROCESSED);
}

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
    pub exporters: Vec<Arc<dyn MetricsExporter>>,
    pub datasource_cancellation_token: Option<CancellationToken>,
    pub shutdown_strategy: ShutdownStrategy,
    pub channel_buffer_size: usize,
}

fn flatten_nested_instructions<'a>(
    nested_instructions: &'a NestedInstructions,
    flat: &mut Vec<&'a crate::instruction::NestedInstruction>,
) {
    for nested_instruction in nested_instructions.iter() {
        flat.push(nested_instruction);
        flatten_nested_instructions(&nested_instruction.inner_instructions, flat);
    }
}

impl Pipeline {
    fn export_metrics(&self) -> CarbonResult<()> {
        let snapshot = MetricsRegistry::global().snapshot();
        for exporter in &self.exporters {
            exporter.export(&snapshot)?;
        }
        Ok(())
    }

    fn shutdown_exporters(&self) -> CarbonResult<()> {
        for exporter in &self.exporters {
            exporter.shutdown()?;
        }
        Ok(())
    }

    pub fn builder() -> PipelineBuilder {
        log::trace!("Pipeline::builder()");
        PipelineBuilder {
            datasources: Vec::new(),
            account_pipes: Vec::new(),
            account_deletion_pipes: Vec::new(),
            block_details_pipes: Vec::new(),
            instruction_pipes: Vec::new(),
            transaction_pipes: Vec::new(),
            exporters: Vec::new(),
            datasource_cancellation_token: None,
            shutdown_strategy: ShutdownStrategy::default(),
            channel_buffer_size: DEFAULT_CHANNEL_BUFFER_SIZE,
        }
    }

    pub async fn run(&mut self) -> CarbonResult<()> {
        log::info!("starting pipeline. num_datasources: {}, num_exporters: {}, num_account_pipes: {}, num_account_deletion_pipes: {}, num_instruction_pipes: {}, num_transaction_pipes: {}",
            self.datasources.len(),
            self.exporters.len(),
            self.account_pipes.len(),
            self.account_deletion_pipes.len(),
            self.instruction_pipes.len(),
            self.transaction_pipes.len(),
        );

        log::trace!("run(self)");

        for exporter in &self.exporters {
            let exporter = Arc::clone(exporter);
            MetricsExporter::initialize(exporter)?;
        }
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

            tokio::spawn(async move {
                if let Err(e) = datasource_clone
                    .consume(
                        datasource_id,
                        sender_clone,
                        datasource_cancellation_token_clone,
                    )
                    .await
                {
                    log::error!("error consuming datasource: {e:?}");
                }
            });
        }

        drop(update_sender);

        loop {
            tokio::select! {
                _ = datasource_cancellation_token.cancelled() => {
                    log::trace!("datasource cancellation token cancelled, shutting down.");
                    self.export_metrics()?;
                    self.shutdown_exporters()?;
                    break;
                }
                _ = tokio::signal::ctrl_c() => {
                    log::trace!("received SIGINT, shutting down.");
                    datasource_cancellation_token.cancel();

                    if self.shutdown_strategy == ShutdownStrategy::Immediate {
                        log::info!("shutting down the pipeline immediately.");
                        self.export_metrics()?;
                        self.shutdown_exporters()?;
                        break;
                    } else {
                        log::info!("shutting down the pipeline after processing pending updates.");
                    }
                }
                update = update_receiver.recv() => {
                    match update {
                        Some((update, datasource_id)) => {
                            UPDATES_RECEIVED.inc();

                            let start = Instant::now();
                            let process_result = self.process(update.clone(), datasource_id.clone()).await;
                            let time_taken_nanoseconds = start.elapsed().as_nanos();
                            let time_taken_milliseconds = time_taken_nanoseconds / 1_000_000;

                            PROCESSING_TIME_NANOS.record(time_taken_nanoseconds as f64);
                            PROCESSING_TIME_MILLIS.record(time_taken_milliseconds as f64);

                            match process_result {
                                Ok(_) => {
                                    UPDATES_SUCCESSFUL.inc();
                                    log::trace!("processed update")
                                }
                                Err(error) => {
                                    log::error!("error processing update ({update:?}): {error:?}");
                                    UPDATES_FAILED.inc();
                                }
                            };

                            UPDATES_PROCESSED.inc();
                            UPDATES_QUEUED.set(update_receiver.len() as f64);
                        }
                        None => {
                            log::info!("update_receiver closed, shutting down.");
                            self.export_metrics()?;
                            self.shutdown_exporters()?;
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

                let context = FilterContext {
                    datasource_id: &datasource_id,
                };

                for pipe in self.account_pipes.iter_mut() {
                    if pipe.filters().iter().all(|filter| {
                        matches!(
                            filter.filter_account(
                                &context,
                                &account_metadata,
                                &account_update.account
                            ),
                            FilterResult::Accept
                        )
                    }) {
                        pipe.run((account_metadata.clone(), account_update.account.clone()))
                            .await?;
                    }
                }

                ACCOUNT_UPDATES_PROCESSED.inc();
            }
            Update::Transaction(transaction_update) => {
                let transaction_metadata = Arc::new((*transaction_update).clone().try_into()?);

                let instructions_with_metadata: InstructionsWithMetadata =
                    transformers::extract_instructions_with_metadata(
                        &transaction_metadata,
                        &transaction_update,
                    )?;

                let nested_instructions: NestedInstructions =
                    instructions_with_metadata.clone().into();
                let mut all_instructions = Vec::new();
                flatten_nested_instructions(&nested_instructions, &mut all_instructions);

                let context = FilterContext {
                    datasource_id: &datasource_id,
                };

                for pipe in self.instruction_pipes.iter_mut() {
                    for &nested_instruction in &all_instructions {
                        if pipe.filters().iter().all(|filter| {
                            matches!(
                                filter.filter_instruction(&context, nested_instruction),
                                FilterResult::Accept
                            )
                        }) {
                            pipe.run(nested_instruction).await?;
                        }
                    }
                }

                for pipe in self.transaction_pipes.iter_mut() {
                    if pipe.filters().iter().all(|filter| {
                        matches!(
                            filter.filter_transaction(
                                &context,
                                &transaction_metadata,
                                &nested_instructions
                            ),
                            FilterResult::Accept
                        )
                    }) {
                        pipe.run(transaction_metadata.clone(), &instructions_with_metadata)
                            .await?;
                    }
                }

                TRANSACTION_UPDATES_PROCESSED.inc();
            }
            Update::AccountDeletion(account_deletion) => {
                let context = FilterContext {
                    datasource_id: &datasource_id,
                };

                for pipe in self.account_deletion_pipes.iter_mut() {
                    if pipe.filters().iter().all(|filter| {
                        matches!(
                            filter.filter_account_deletion(&context, &account_deletion),
                            FilterResult::Accept
                        )
                    }) {
                        pipe.run(account_deletion.clone()).await?;
                    }
                }

                ACCOUNT_DELETIONS_PROCESSED.inc();
            }
            Update::BlockDetails(block_details) => {
                let context = FilterContext {
                    datasource_id: &datasource_id,
                };

                for pipe in self.block_details_pipes.iter_mut() {
                    if pipe.filters().iter().all(|filter| {
                        matches!(
                            filter.filter_block_details(&context, &block_details),
                            FilterResult::Accept
                        )
                    }) {
                        pipe.run(block_details.clone()).await?;
                    }
                }

                BLOCK_DETAILS_PROCESSED.inc();
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
    pub exporters: Vec<Arc<dyn MetricsExporter>>,
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

    pub fn account<T, P>(
        mut self,
        decoder: impl for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static,
        processor: P,
    ) -> Self
    where
        T: Send + Sync + 'static,
        P: for<'a> Processor<AccountProcessorInputType<'a, T>> + Send + Sync + 'static,
    {
        log::trace!(
            "account(self, decoder: {:?}, processor: {:?})",
            stringify!(decoder),
            stringify!(processor)
        );
        self.account_pipes.push(Box::new(AccountPipe {
            decoder: Box::new(decoder),
            processor,
            filters: vec![],
        }));
        self
    }

    pub fn account_with_filters<T, P>(
        mut self,
        decoder: impl for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static,
        processor: P,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self
    where
        T: Send + Sync + 'static,
        P: for<'a> Processor<AccountProcessorInputType<'a, T>> + Send + Sync + 'static,
    {
        log::trace!(
            "account_with_filters(self, decoder: {:?}, processor: {:?}, filters: {:?})",
            stringify!(decoder),
            stringify!(processor),
            stringify!(filters)
        );
        self.account_pipes.push(Box::new(AccountPipe {
            decoder: Box::new(decoder),
            processor,
            filters,
        }));
        self
    }

    pub fn account_deletions<P>(mut self, processor: P) -> Self
    where
        P: Processor<AccountDeletion> + Send + Sync + 'static,
    {
        log::trace!(
            "account_deletions(self, processor: {:?})",
            stringify!(processor)
        );
        self.account_deletion_pipes
            .push(Box::new(AccountDeletionPipe {
                processor,
                filters: vec![],
            }));
        self
    }

    pub fn account_deletions_with_filters<P>(
        mut self,
        processor: P,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self
    where
        P: Processor<AccountDeletion> + Send + Sync + 'static,
    {
        log::trace!(
            "account_deletions_with_filters(self, processor: {:?}, filters: {:?})",
            stringify!(processor),
            stringify!(filters)
        );
        self.account_deletion_pipes
            .push(Box::new(AccountDeletionPipe { processor, filters }));
        self
    }

    pub fn block_details<P>(mut self, processor: P) -> Self
    where
        P: Processor<BlockDetails> + Send + Sync + 'static,
    {
        log::trace!(
            "block_details(self, processor: {:?})",
            stringify!(processor)
        );
        self.block_details_pipes.push(Box::new(BlockDetailsPipe {
            processor,
            filters: vec![],
        }));
        self
    }

    pub fn block_details_with_filters<P>(
        mut self,
        processor: P,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self
    where
        P: Processor<BlockDetails> + Send + Sync + 'static,
    {
        log::trace!(
            "block_details_with_filters(self, processor: {:?}, filters: {:?})",
            stringify!(processor),
            stringify!(filters)
        );
        self.block_details_pipes
            .push(Box::new(BlockDetailsPipe { processor, filters }));
        self
    }

    pub fn instruction<T, P>(
        mut self,
        decoder: impl for<'a> InstructionDecoder<'a, InstructionType = T> + Send + Sync + 'static,
        processor: P,
    ) -> Self
    where
        T: Send + Sync + 'static,
        P: for<'a> Processor<InstructionProcessorInputType<'a, T>> + Send + Sync + 'static,
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
        T: Send + Sync + 'static,
        P: for<'a> Processor<InstructionProcessorInputType<'a, T>> + Send + Sync + 'static,
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

    pub fn transaction<T, P>(mut self, processor: P) -> Self
    where
        T: InstructionDecoderCollection + 'static,
        P: for<'a> Processor<TransactionProcessorInputType<'a, T>> + Send + Sync + 'static,
    {
        log::trace!("transaction(self, processor: {:?})", stringify!(processor));
        self.transaction_pipes
            .push(Box::new(TransactionPipe::<T, P>::new(processor, vec![])));
        self
    }

    pub fn transaction_with_filters<T, P>(
        mut self,
        processor: P,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self
    where
        T: InstructionDecoderCollection + 'static,
        P: for<'a> Processor<TransactionProcessorInputType<'a, T>> + Send + Sync + 'static,
    {
        log::trace!(
            "transaction_with_filters(self, processor: {:?}, filters: {:?})",
            stringify!(processor),
            stringify!(filters)
        );
        self.transaction_pipes
            .push(Box::new(TransactionPipe::<T, P>::new(processor, filters)));
        self
    }

    pub fn metrics(mut self, exporter: Arc<dyn MetricsExporter>) -> Self {
        log::trace!("metrics(self, exporter: {:?})", stringify!(exporter));
        self.exporters.push(exporter);
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
        register_pipeline_metrics();
        Ok(Pipeline {
            datasources: self.datasources,
            account_pipes: self.account_pipes,
            account_deletion_pipes: self.account_deletion_pipes,
            block_details_pipes: self.block_details_pipes,
            instruction_pipes: self.instruction_pipes,
            transaction_pipes: self.transaction_pipes,
            exporters: self.exporters,
            datasource_cancellation_token: self.datasource_cancellation_token,
            shutdown_strategy: self.shutdown_strategy,
            channel_buffer_size: self.channel_buffer_size,
        })
    }
}
