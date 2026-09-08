//! Pipeline orchestrator — central runtime that wires datasources,
//! pipes, filters, and metrics into a single `run()` loop.
//!
//! # Components
//!
//! - [`Pipeline`] — built type that owns all datasources, pipes, and exporters.
//!   Driven by [`Pipeline::run`].
//! - [`PipelineBuilder`] — fluent constructor returning `Pipeline` via
//!   `.build()`. Every framework user starts here.
//! - [`ShutdownStrategy`] — `Immediate` (drop in-flight on ctrl-C) vs
//!   `ProcessPending` (drain the channel before exit).
//!
//! # Flow
//!
//! 1. `run()` spawns one tokio task and creates one MPSC channel per datasource.
//! 2. It reads channels in turn and calls every registered pipe whose
//!    update type matches and whose filters return `true`.
//! 3. Each pipe decodes the payload (where applicable) and invokes its
//!    `Processor`.
//! 4. Crate-wide metrics (received / processed / successful / failed / queued /
//!    processing-time histograms) are updated per iteration.
//! 5. Shutdown via the supplied `CancellationToken` or ctrl-C; behaviour
//!    governed by [`ShutdownStrategy`].

use {
    crate::{
        account::AccountDecoder,
        account_deletion::{AccountDeletionPipe, AccountDeletionPipes},
        block_details::{BlockDetailsPipe, BlockDetailsPipes},
        collection::InstructionDecoderCollection,
        datasource::{
            queue::{next_update, QueuedUpdate},
            receipt::UpdateReceiptError,
            Datasource, DatasourceContext, DatasourceOptions, DynDatasource, ShutdownSignal,
        },
        error::{CarbonResult, Error},
        filter::Filters,
        id::{Id, IdError},
        instruction::{
            extract_instructions_with_metadata, InstructionDecoder, InstructionsWithMetadata,
            NestedInstruction, NestedInstructions,
        },
        metrics::{Counter, Gauge, Histogram, MetricsExporter, MetricsRegistry},
        processor::Processor,
        route::{
            AccountProcessorInput, AccountRoute, DecodedRouteOptions, DynAccountRoute,
            DynInstructionRoute, InstructionProcessorInput, InstructionRoute, RouteContext,
            TransactionProcessorInput,
        },
        transaction::{TransactionPipe, TransactionPipes},
        update::{AccountClosureUpdate, AccountUpdate, BlockUpdate, TransactionUpdate, Update},
    },
    std::{
        collections::HashSet,
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

/// Shutdown semantics on ctrl-C or external cancellation.
///
/// - `Immediate` — cancel datasources, flush metrics, exit; in-flight updates
///   may be dropped.
/// - `ProcessPending` — cancel datasources, then drain the channel through the
///   registered pipes before exiting. Default.
#[derive(Default, PartialEq, Debug)]
pub enum ShutdownStrategy {
    Immediate,
    #[default]
    ProcessPending,
}

/// Default capacity of each datasource's MPSC channel to the
/// pipeline loop. Override with [`PipelineBuilder::channel_buffer_size`].
pub const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 1_000;

/// Invalid pipeline or registration IDs.
#[derive(Debug, thiserror::Error)]
pub enum PipelineBuildError {
    #[error("invalid pipeline ID: {source}")]
    InvalidPipelineId { source: IdError },
    #[error("invalid datasource ID at registration {registration_index}: {source}")]
    InvalidDatasourceId {
        registration_index: usize,
        source: IdError,
    },
    #[error("invalid route ID {id:?}: {source}")]
    InvalidRouteId { id: String, source: IdError },
    #[error("duplicate datasource ID: {id}")]
    DuplicateDatasourceId { id: Id },
    #[error("duplicate route ID: {id}")]
    DuplicateRouteId { id: Id },
}

fn build_routes<P: ?Sized>(
    routes: Vec<(String, Box<P>)>,
    ids: &mut HashSet<Id>,
) -> Result<Vec<(Id, Box<P>)>, PipelineBuildError> {
    routes
        .into_iter()
        .map(|(name, pipe)| {
            let id = Id::new(name.clone())
                .map_err(|source| PipelineBuildError::InvalidRouteId { id: name, source })?;
            if !ids.insert(id.clone()) {
                return Err(PipelineBuildError::DuplicateRouteId { id });
            }
            Ok((id, pipe))
        })
        .collect()
}

/// Built pipeline ready to execute. Construct via [`Pipeline::builder`].
///
/// Owns every datasource, pipe, and exporter for the lifetime of
/// [`run`](Self::run). Construct through the builder.
pub struct Pipeline {
    pub id: Id,
    datasources: Vec<(Id, Box<dyn DynDatasource>)>,
    account_routes: Vec<(Id, Box<dyn DynAccountRoute>)>,
    pub account_deletion_pipes: Vec<(Id, Box<dyn AccountDeletionPipes>)>,
    pub block_details_pipes: Vec<(Id, Box<dyn BlockDetailsPipes>)>,
    instruction_routes: Vec<(Id, Box<dyn DynInstructionRoute>)>,
    pub transaction_pipes: Vec<(Id, Box<dyn TransactionPipes>)>,
    pub exporters: Vec<Arc<dyn MetricsExporter>>,
    pub datasource_cancellation_token: Option<CancellationToken>,
    pub shutdown_strategy: ShutdownStrategy,
    pub channel_buffer_size: usize,
}

impl Pipeline {
    pub fn builder(id: impl Into<String>) -> PipelineBuilder {
        PipelineBuilder::new(id)
    }

    pub async fn run(mut self) -> CarbonResult<()> {
        log::info!("starting pipeline. num_datasources: {}, num_exporters: {}, num_account_routes: {}, num_account_deletion_pipes: {}, num_instruction_routes: {}, num_transaction_pipes: {}",
            self.datasources.len(),
            self.exporters.len(),
            self.account_routes.len(),
            self.account_deletion_pipes.len(),
            self.instruction_routes.len(),
            self.transaction_pipes.len(),
        );

        for exporter in &self.exporters {
            let exporter = Arc::clone(exporter);
            MetricsExporter::initialize(exporter)?;
        }
        let datasource_cancellation_token = self
            .datasource_cancellation_token
            .clone()
            .unwrap_or_default();

        let options = DatasourceOptions::default().queue_capacity(self.channel_buffer_size);
        let mut queues = Vec::with_capacity(self.datasources.len());
        for (datasource_id, datasource) in self.datasources.drain(..) {
            let (sender, receiver) = options
                .channel()
                .ok_or(Error::InvalidQueueCapacity(self.channel_buffer_size))?;
            let context = DatasourceContext::new(
                self.id.clone(),
                datasource_id.clone(),
                sender,
                options.overflow_policy,
                ShutdownSignal::new(datasource_cancellation_token.clone()),
            );
            queues.push((datasource_id.clone(), receiver));

            tokio::spawn(async move {
                if let Err(error) = datasource.run(context).await {
                    log::error!("datasource {datasource_id} failed: {error:?}");
                }
            });
        }

        let mut next_source = 0;
        loop {
            tokio::select! {
                _ = datasource_cancellation_token.cancelled() => {
                    self.export_metrics()?;
                    self.shutdown_exporters()?;
                    break;
                }
                _ = tokio::signal::ctrl_c() => {
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
                update = next_update(&mut queues, &mut next_source) => {
                    match update {
                        Some((datasource_id, QueuedUpdate { update, receipt_sender })) => {
                            UPDATES_RECEIVED.inc();

                            let start = Instant::now();
                            let process_result = self.process(update.clone(), datasource_id.clone()).await;
                            let time_taken_nanoseconds = start.elapsed().as_nanos();
                            let time_taken_milliseconds = time_taken_nanoseconds / 1_000_000;

                            PROCESSING_TIME_NANOS.record(time_taken_nanoseconds as f64);
                            PROCESSING_TIME_MILLIS.record(time_taken_milliseconds as f64);

                            match process_result {
                                Ok(_) => {
                                    receipt_sender.send(Ok(()));
                                    UPDATES_SUCCESSFUL.inc();
                                }
                                Err(error) => {
                                    receipt_sender.send(Err(UpdateReceiptError::Failed));
                                    log::error!("error processing update ({update:?}): {error:?}");
                                    UPDATES_FAILED.inc();
                                }
                            };

                            UPDATES_PROCESSED.inc();
                            UPDATES_QUEUED.set(
                                queues.iter().map(|(_, receiver)| receiver.len()).sum::<usize>() as f64,
                            );
                        }
                        None => {
                            log::info!("all datasource queues drained, shutting down.");
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

    async fn process(&mut self, update: Update, datasource_id: Id) -> CarbonResult<()> {
        match update {
            Update::Account(update) => {
                for (route_id, route) in &mut self.account_routes {
                    route
                        .run(
                            &RouteContext::new(&self.id, &datasource_id, route_id),
                            &update,
                        )
                        .await?;
                }
                ACCOUNT_UPDATES_PROCESSED.inc();
            }
            Update::AccountClosure(update) => {
                for (route_id, pipe) in &mut self.account_deletion_pipes {
                    pipe.run(
                        &RouteContext::new(&self.id, &datasource_id, route_id),
                        &update,
                    )
                    .await?;
                }
                ACCOUNT_DELETIONS_PROCESSED.inc();
            }
            Update::Block(update) => {
                for (route_id, pipe) in &mut self.block_details_pipes {
                    pipe.run(
                        &RouteContext::new(&self.id, &datasource_id, route_id),
                        &update,
                    )
                    .await?;
                }
                BLOCK_DETAILS_PROCESSED.inc();
            }
            Update::Transaction(update) => {
                let metadata = Arc::new(update.clone().try_into()?);
                let instructions: InstructionsWithMetadata =
                    extract_instructions_with_metadata(&metadata, &update)?;
                let instructions: NestedInstructions = instructions.try_into()?;
                let mut all_instructions = Vec::new();
                Self::flatten_nested_instructions(&instructions, &mut all_instructions);

                for (route_id, route) in &mut self.instruction_routes {
                    let context = RouteContext::new(&self.id, &datasource_id, route_id);
                    for &instruction in &all_instructions {
                        route.run(&context, instruction).await?;
                    }
                }
                for (route_id, pipe) in &mut self.transaction_pipes {
                    pipe.run(
                        &RouteContext::new(&self.id, &datasource_id, route_id),
                        &update,
                        &all_instructions,
                    )
                    .await?;
                }
                TRANSACTION_UPDATES_PROCESSED.inc();
            }
        }
        Ok(())
    }

    fn flatten_nested_instructions<'a>(
        nested_instructions: &'a NestedInstructions,
        flat: &mut Vec<&'a crate::instruction::NestedInstruction>,
    ) {
        for nested_instruction in nested_instructions.iter() {
            flat.push(nested_instruction);
            Self::flatten_nested_instructions(&nested_instruction.inner_instructions, flat);
        }
    }
}

pub struct PipelineBuilder {
    pub id: String,
    datasources: Vec<(String, Box<dyn DynDatasource>)>,
    account_routes: Vec<(String, Box<dyn DynAccountRoute>)>,
    pub account_deletion_pipes: Vec<(String, Box<dyn AccountDeletionPipes>)>,
    pub block_details_pipes: Vec<(String, Box<dyn BlockDetailsPipes>)>,
    instruction_routes: Vec<(String, Box<dyn DynInstructionRoute>)>,
    pub transaction_pipes: Vec<(String, Box<dyn TransactionPipes>)>,
    pub exporters: Vec<Arc<dyn MetricsExporter>>,
    pub datasource_cancellation_token: Option<CancellationToken>,
    pub shutdown_strategy: ShutdownStrategy,
    pub channel_buffer_size: usize,
}

impl PipelineBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            datasources: Vec::new(),
            account_routes: Vec::new(),
            account_deletion_pipes: Vec::new(),
            block_details_pipes: Vec::new(),
            instruction_routes: Vec::new(),
            transaction_pipes: Vec::new(),
            exporters: Vec::new(),
            datasource_cancellation_token: None,
            shutdown_strategy: ShutdownStrategy::default(),
            channel_buffer_size: DEFAULT_CHANNEL_BUFFER_SIZE,
        }
    }
    pub fn datasource(
        mut self,
        id: impl Into<String>,
        datasource: impl Datasource + 'static,
    ) -> Self {
        self.datasources.push((id.into(), Box::new(datasource)));
        self
    }

    pub fn shutdown_strategy(mut self, shutdown_strategy: ShutdownStrategy) -> Self {
        self.shutdown_strategy = shutdown_strategy;
        self
    }

    pub fn account<D, P>(self, route_id: impl Into<String>, decoder: D, processor: P) -> Self
    where
        D: AccountDecoder + Send + 'static,
        D::AccountType: Send + Sync + 'static,
        P: for<'a> Processor<AccountProcessorInput<'a, D::AccountType>> + 'static,
    {
        self.account_with_options(route_id, decoder, processor, DecodedRouteOptions::default())
    }

    pub fn account_with_options<D, P>(
        mut self,
        route_id: impl Into<String>,
        decoder: D,
        processor: P,
        options: DecodedRouteOptions<AccountUpdate>,
    ) -> Self
    where
        D: AccountDecoder + Send + 'static,
        D::AccountType: Send + Sync + 'static,
        P: for<'a> Processor<AccountProcessorInput<'a, D::AccountType>> + 'static,
    {
        self.account_routes.push((
            route_id.into(),
            Box::new(AccountRoute::new(decoder, processor, options)),
        ));
        self
    }

    pub fn account_deletions<P>(mut self, route_id: impl Into<String>, processor: P) -> Self
    where
        P: Processor<AccountClosureUpdate> + 'static,
    {
        self.account_deletion_pipes.push((
            route_id.into(),
            Box::new(AccountDeletionPipe::new(processor, Filters::new())),
        ));
        self
    }

    pub fn account_deletions_with_filters<P>(
        mut self,
        route_id: impl Into<String>,
        processor: P,
        filters: Filters<AccountClosureUpdate>,
    ) -> Self
    where
        P: Processor<AccountClosureUpdate> + 'static,
    {
        self.account_deletion_pipes.push((
            route_id.into(),
            Box::new(AccountDeletionPipe::new(processor, filters)),
        ));
        self
    }

    pub fn block_details<P>(mut self, route_id: impl Into<String>, processor: P) -> Self
    where
        P: Processor<BlockUpdate> + 'static,
    {
        self.block_details_pipes.push((
            route_id.into(),
            Box::new(BlockDetailsPipe::new(processor, Filters::new())),
        ));
        self
    }

    pub fn block_details_with_filters<P>(
        mut self,
        route_id: impl Into<String>,
        processor: P,
        filters: Filters<BlockUpdate>,
    ) -> Self
    where
        P: Processor<BlockUpdate> + 'static,
    {
        self.block_details_pipes.push((
            route_id.into(),
            Box::new(BlockDetailsPipe::new(processor, filters)),
        ));
        self
    }

    pub fn instruction<D, P>(self, route_id: impl Into<String>, decoder: D, processor: P) -> Self
    where
        D: InstructionDecoder + Send + 'static,
        D::InstructionType: Send + Sync + 'static,
        P: for<'a> Processor<InstructionProcessorInput<'a, D::InstructionType>> + 'static,
    {
        self.instruction_with_options(route_id, decoder, processor, DecodedRouteOptions::default())
    }

    pub fn instruction_with_options<D, P>(
        mut self,
        route_id: impl Into<String>,
        decoder: D,
        processor: P,
        options: DecodedRouteOptions<NestedInstruction>,
    ) -> Self
    where
        D: InstructionDecoder + Send + 'static,
        D::InstructionType: Send + Sync + 'static,
        P: for<'a> Processor<InstructionProcessorInput<'a, D::InstructionType>> + 'static,
    {
        self.instruction_routes.push((
            route_id.into(),
            Box::new(InstructionRoute::new(decoder, processor, options)),
        ));
        self
    }

    pub fn transaction<T, P>(mut self, route_id: impl Into<String>, processor: P) -> Self
    where
        T: InstructionDecoderCollection + Send + Sync + 'static,
        P: for<'a> Processor<TransactionProcessorInput<'a, T>> + 'static,
    {
        self.transaction_pipes.push((
            route_id.into(),
            Box::new(TransactionPipe::<T, P>::new(processor, Filters::new())),
        ));
        self
    }

    pub fn transaction_with_filters<T, P>(
        mut self,
        route_id: impl Into<String>,
        processor: P,
        filters: Filters<TransactionUpdate>,
    ) -> Self
    where
        T: InstructionDecoderCollection + Send + Sync + 'static,
        P: for<'a> Processor<TransactionProcessorInput<'a, T>> + 'static,
    {
        self.transaction_pipes.push((
            route_id.into(),
            Box::new(TransactionPipe::<T, P>::new(processor, filters)),
        ));
        self
    }

    pub fn metrics(mut self, exporter: Arc<dyn MetricsExporter>) -> Self {
        self.exporters.push(exporter);
        self
    }

    pub fn datasource_cancellation_token(mut self, cancellation_token: CancellationToken) -> Self {
        self.datasource_cancellation_token = Some(cancellation_token);
        self
    }

    pub fn channel_buffer_size(mut self, size: usize) -> Self {
        self.channel_buffer_size = size;
        self
    }

    pub fn build(self) -> Result<Pipeline, PipelineBuildError> {
        let id =
            Id::new(self.id).map_err(|source| PipelineBuildError::InvalidPipelineId { source })?;
        let mut source_ids = HashSet::new();
        let mut datasources = Vec::with_capacity(self.datasources.len());
        for (registration_index, (name, datasource)) in self.datasources.into_iter().enumerate() {
            let id = Id::new(name).map_err(|source| PipelineBuildError::InvalidDatasourceId {
                registration_index,
                source,
            })?;
            if !source_ids.insert(id.clone()) {
                return Err(PipelineBuildError::DuplicateDatasourceId { id });
            }
            datasources.push((id, datasource));
        }
        let mut route_ids = HashSet::new();
        let pipeline = Pipeline {
            id,
            datasources,
            account_routes: build_routes(self.account_routes, &mut route_ids)?,
            account_deletion_pipes: build_routes(self.account_deletion_pipes, &mut route_ids)?,
            block_details_pipes: build_routes(self.block_details_pipes, &mut route_ids)?,
            instruction_routes: build_routes(self.instruction_routes, &mut route_ids)?,
            transaction_pipes: build_routes(self.transaction_pipes, &mut route_ids)?,
            exporters: self.exporters,
            datasource_cancellation_token: self.datasource_cancellation_token,
            shutdown_strategy: self.shutdown_strategy,
            channel_buffer_size: self.channel_buffer_size,
        };
        register_pipeline_metrics();
        #[cfg(feature = "postgres")]
        crate::postgres::processors::register_postgres_metrics();
        Ok(pipeline)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            error::{BoxError, Error},
            processor::ProcessorResult,
            update::{AccountUpdate, TransactionUpdate},
        },
        solana_account::Account,
        solana_hash::Hash,
        solana_instruction::Instruction,
        solana_message::{
            compiled_instruction::CompiledInstruction, legacy::Message, MessageHeader,
            VersionedMessage,
        },
        solana_pubkey::Pubkey,
        solana_signature::Signature,
        solana_transaction::versioned::VersionedTransaction,
        solana_transaction_status::{InnerInstruction, InnerInstructions, TransactionStatusMeta},
        std::{cell::Cell, sync::Mutex},
    };

    struct Source;

    impl Datasource for Source {
        async fn run(self, _context: DatasourceContext) -> Result<(), BoxError> {
            Ok(())
        }
    }

    struct ReceiptSource {
        grouped: bool,
        result: tokio::sync::oneshot::Sender<Result<(), UpdateReceiptError>>,
    }

    impl Datasource for ReceiptSource {
        async fn run(self, mut context: DatasourceContext) -> Result<(), BoxError> {
            assert_eq!(context.pipeline_id().as_str(), "pipeline");
            let receipt = if self.grouped {
                let mut group = context.begin_group();
                group.emit(BlockUpdate::new(1).into()).await?;
                group.emit(BlockUpdate::new(2).into()).await?;
                group.seal()
            } else {
                context.emit(BlockUpdate::new(1).into()).await?.unwrap()
            };
            self.result.send(receipt.processed().await).unwrap();
            Ok(())
        }
    }

    struct Decoder;

    impl AccountDecoder for Decoder {
        type AccountType = u8;
        fn decode_account(
            &self,
            _pubkey: &Pubkey,
            _account: &Account,
        ) -> Result<Option<u8>, BoxError> {
            Ok(Some(7))
        }
    }

    impl InstructionDecoder for Decoder {
        type InstructionType = u8;
        fn decode_instruction(&self, instruction: &Instruction) -> Result<Option<u8>, BoxError> {
            Ok(instruction.data.first().copied())
        }
    }

    struct Collection;

    impl InstructionDecoderCollection for Collection {
        fn decode_instruction(_instruction: &Instruction) -> Result<Option<Self>, BoxError> {
            Ok(Some(Self))
        }
    }

    type Seen = Arc<Mutex<Vec<(String, String, String, usize)>>>;

    struct Recorder {
        calls: Cell<usize>,
        seen: Seen,
    }

    impl Recorder {
        fn new(seen: Seen) -> Self {
            Self {
                calls: Cell::new(0),
                seen,
            }
        }
    }

    impl<T: Sync> Processor<T> for Recorder {
        async fn process(&mut self, context: &RouteContext<'_>, _value: &T) -> ProcessorResult {
            self.calls.set(self.calls.get() + 1);
            tokio::task::yield_now().await;
            self.seen.lock().unwrap().push((
                context.pipeline_id().to_string(),
                context.datasource_id().to_string(),
                context.route_id().to_string(),
                self.calls.get(),
            ));
            Ok(())
        }
    }

    fn recorder() -> Recorder {
        Recorder::new(Arc::new(Mutex::new(Vec::new())))
    }

    struct RouteObserver {
        filtered: Recorder,
        committed: Recorder,
    }

    impl<T: Sync> crate::filter::Filter<T> for RouteObserver {
        async fn filter(
            &mut self,
            context: &RouteContext<'_>,
            value: &T,
        ) -> Result<bool, BoxError> {
            self.filtered.process(context, value).await?;
            Ok(true)
        }

        async fn commit(
            &mut self,
            context: &RouteContext<'_>,
            value: &T,
            result: &ProcessorResult,
        ) -> Result<(), BoxError> {
            assert!(result.is_ok());
            self.committed.process(context, value).await
        }
    }

    #[test]
    fn builder_rejects_blank_ids_and_duplicates() {
        assert!(matches!(
            Pipeline::builder(" ").build(),
            Err(PipelineBuildError::InvalidPipelineId { .. })
        ));
        assert!(matches!(
            Pipeline::builder("pipeline")
                .datasource(" ", Source)
                .build(),
            Err(PipelineBuildError::InvalidDatasourceId {
                registration_index: 0,
                ..
            })
        ));
        assert!(matches!(
            Pipeline::builder("pipeline")
                .datasource("source", Source)
                .datasource("source", Source)
                .build(),
            Err(PipelineBuildError::DuplicateDatasourceId { .. })
        ));
        for builder in [
            Pipeline::builder("pipeline").account(" ", Decoder, recorder()),
            Pipeline::builder("pipeline").account_with_options(
                " ",
                Decoder,
                recorder(),
                DecodedRouteOptions::default(),
            ),
            Pipeline::builder("pipeline").instruction(" ", Decoder, recorder()),
            Pipeline::builder("pipeline").instruction_with_options(
                " ",
                Decoder,
                recorder(),
                DecodedRouteOptions::default(),
            ),
            Pipeline::builder("pipeline").transaction::<Collection, _>(" ", recorder()),
            Pipeline::builder("pipeline").transaction_with_filters::<Collection, _>(
                " ",
                recorder(),
                Filters::new(),
            ),
            Pipeline::builder("pipeline").account_deletions(" ", recorder()),
            Pipeline::builder("pipeline").account_deletions_with_filters(
                " ",
                recorder(),
                Filters::new(),
            ),
            Pipeline::builder("pipeline").block_details(" ", recorder()),
            Pipeline::builder("pipeline").block_details_with_filters(
                " ",
                recorder(),
                Filters::new(),
            ),
        ] {
            assert!(matches!(
                builder.build(),
                Err(PipelineBuildError::InvalidRouteId { .. })
            ));
        }
        assert!(matches!(
            Pipeline::builder("pipeline")
                .account("same", Decoder, recorder())
                .instruction("same", Decoder, recorder())
                .build(),
            Err(PipelineBuildError::DuplicateRouteId { .. })
        ));
        assert!(matches!(
            Pipeline::builder("pipeline")
                .block_details("same", recorder())
                .block_details("same", recorder())
                .build(),
            Err(PipelineBuildError::DuplicateRouteId { .. })
        ));
        // Names are unique within their category, not across categories.
        assert!(Pipeline::builder("same")
            .datasource("same", Source)
            .block_details("same", recorder())
            .build()
            .is_ok());
    }

    #[test]
    fn pipeline_future_is_send_with_a_non_sync_processor() {
        fn assert_send<T: Send>(_: T) {}

        let pipeline = Pipeline::builder("pipeline")
            .block_details("blocks", recorder())
            .build()
            .unwrap();
        assert_send(pipeline.run());
    }

    #[tokio::test]
    async fn run_settles_individual_and_grouped_receipts() {
        for fail in [false, true] {
            let seen = Arc::new(Mutex::new(Vec::new()));
            let mut builder = Pipeline::builder("pipeline").channel_buffer_size(1);
            let mut results = Vec::new();
            for (name, grouped) in [("single", false), ("group", true)] {
                let (result, receiver) = tokio::sync::oneshot::channel();
                builder = builder.datasource(name, ReceiptSource { grouped, result });
                results.push(receiver);
            }
            builder = if fail {
                builder.block_details("blocks", Failing)
            } else {
                builder.block_details("blocks", Recorder::new(seen.clone()))
            };
            tokio::time::timeout(
                std::time::Duration::from_secs(5),
                builder.build().unwrap().run(),
            )
            .await
            .unwrap()
            .unwrap();

            for result in results {
                assert_eq!(
                    result.await.unwrap(),
                    if fail {
                        Err(UpdateReceiptError::Failed)
                    } else {
                        Ok(())
                    }
                );
            }
            if !fail {
                let seen = seen.lock().unwrap();
                assert_eq!(seen.iter().filter(|call| call.1 == "single").count(), 1);
                assert_eq!(seen.iter().filter(|call| call.1 == "group").count(), 2);
            }
        }
    }

    #[tokio::test]
    async fn run_rejects_invalid_queue_capacity_without_panicking() {
        for capacity in [0, usize::MAX] {
            let result = Pipeline::builder("pipeline")
                .datasource("source", Source)
                .channel_buffer_size(capacity)
                .build()
                .unwrap()
                .run()
                .await;
            assert!(matches!(result, Err(Error::InvalidQueueCapacity(value)) if value == capacity));
        }
    }

    #[tokio::test]
    async fn every_route_receives_registered_ids_and_keeps_mutable_processor_state() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let filtered = Arc::new(Mutex::new(Vec::new()));
        let committed = Arc::new(Mutex::new(Vec::new()));
        fn filters<T: Sync>(filtered: &Seen, committed: &Seen) -> Filters<T> {
            let mut filters = Filters::new();
            filters.push(RouteObserver {
                filtered: Recorder::new(filtered.clone()),
                committed: Recorder::new(committed.clone()),
            });
            filters
        }
        let mut pipeline = Pipeline::builder(" indexer ")
            .datasource("source", Source)
            .account_with_options(
                "accounts",
                Decoder,
                Recorder::new(seen.clone()),
                DecodedRouteOptions::default().filter(RouteObserver {
                    filtered: Recorder::new(filtered.clone()),
                    committed: Recorder::new(committed.clone()),
                }),
            )
            .instruction_with_options(
                "instructions",
                Decoder,
                Recorder::new(seen.clone()),
                DecodedRouteOptions::default().filter(RouteObserver {
                    filtered: Recorder::new(filtered.clone()),
                    committed: Recorder::new(committed.clone()),
                }),
            )
            .transaction_with_filters::<Collection, _>(
                "transactions",
                Recorder::new(seen.clone()),
                filters(&filtered, &committed),
            )
            .account_deletions_with_filters(
                "closures",
                Recorder::new(seen.clone()),
                filters(&filtered, &committed),
            )
            .block_details_with_filters(
                "blocks",
                Recorder::new(seen.clone()),
                filters(&filtered, &committed),
            )
            .build()
            .unwrap();
        let source = pipeline.datasources[0].0.clone();
        let account = AccountUpdate::new(
            Pubkey::new_unique(),
            Account {
                lamports: 1,
                ..Default::default()
            },
            7,
        );
        pipeline
            .process(account.clone().into(), source.clone())
            .await
            .unwrap();
        pipeline
            .process(account.into(), source.clone())
            .await
            .unwrap();

        let instruction = CompiledInstruction {
            program_id_index: 1,
            accounts: vec![0],
            data: vec![7],
        };
        let transaction = TransactionUpdate::new(
            VersionedTransaction {
                signatures: vec![Signature::default()],
                message: VersionedMessage::Legacy(Message {
                    header: MessageHeader {
                        num_required_signatures: 1,
                        num_readonly_signed_accounts: 0,
                        num_readonly_unsigned_accounts: 1,
                    },
                    account_keys: vec![Pubkey::new_unique(), Pubkey::new_unique()],
                    recent_blockhash: Hash::default(),
                    instructions: vec![instruction.clone()],
                }),
            },
            TransactionStatusMeta {
                inner_instructions: Some(vec![InnerInstructions {
                    index: 0,
                    instructions: vec![InnerInstruction {
                        instruction,
                        stack_height: Some(2),
                    }],
                }]),
                ..Default::default()
            },
            7,
        )
        .unwrap();
        pipeline
            .process(transaction.into(), source.clone())
            .await
            .unwrap();
        pipeline
            .process(
                AccountClosureUpdate::new(Pubkey::new_unique(), Account::default(), 7)
                    .unwrap()
                    .into(),
                source.clone(),
            )
            .await
            .unwrap();
        pipeline
            .process(BlockUpdate::new(7).into(), source)
            .await
            .unwrap();

        let seen = seen.lock().unwrap();
        assert_eq!(*filtered.lock().unwrap(), *seen);
        assert_eq!(*committed.lock().unwrap(), *seen);
        assert!(seen
            .iter()
            .all(|(pipeline, source, _, _)| pipeline == " indexer " && source == "source"));
        let calls: Vec<_> = seen
            .iter()
            .map(|(_, _, route, count)| (route.as_str(), *count))
            .collect();
        assert_eq!(
            calls,
            vec![
                ("accounts", 1),
                ("accounts", 2),
                ("instructions", 1),
                ("instructions", 2),
                ("transactions", 1),
                ("closures", 1),
                ("blocks", 1)
            ]
        );
    }

    #[derive(Debug, thiserror::Error)]
    #[error("rejected: {0}")]
    struct Rejected(u8);

    struct Failing;

    impl Processor<BlockUpdate> for Failing {
        async fn process(
            &mut self,
            context: &RouteContext<'_>,
            _value: &BlockUpdate,
        ) -> ProcessorResult {
            assert_eq!(context.route_id().as_str(), "blocks");
            Err(Box::new(Rejected(7)))
        }
    }

    #[tokio::test]
    async fn processor_errors_preserve_the_original_source() {
        let mut pipeline = Pipeline::builder("pipeline")
            .datasource("source", Source)
            .block_details("blocks", Failing)
            .build()
            .unwrap();
        let source = pipeline.datasources[0].0.clone();
        let error = pipeline
            .process(BlockUpdate::new(7).into(), source)
            .await
            .unwrap_err();
        assert_eq!(
            std::error::Error::source(&error)
                .unwrap()
                .downcast_ref::<Rejected>()
                .unwrap()
                .0,
            7
        );
        let Error::Processor(error) = error else {
            panic!("expected processor error")
        };
        assert_eq!(error.downcast_ref::<Rejected>().unwrap().0, 7);
    }
}
