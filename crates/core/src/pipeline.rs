//! Pipeline configuration and execution.

use {
    crate::{
        account::AccountDecoder,
        collection::InstructionDecoderCollection,
        datasource::{
            queue::next_update, receipt::UpdateReceiptError, Datasource, DatasourceContext,
            DatasourceOptions, DynDatasource, ShutdownSignal,
        },
        error::{BoxError, Error},
        id::{Id, IdError},
        instruction::{
            extract_instructions_with_metadata, InstructionDecoder, NestedInstruction,
            NestedInstructions, TransformError,
        },
        processor::Processor,
        route::{
            AccountClosureRoute, AccountProcessorInput, AccountRoute, BlockRoute,
            DecodedRouteOptions, DynAccountClosureRoute, DynAccountRoute, DynBlockRoute,
            DynInstructionRoute, DynTransactionRoute, InstructionProcessorInput, InstructionRoute,
            RouteContext, RouteOptions, TransactionProcessorInput, TransactionRoute,
        },
        update::{AccountClosureUpdate, AccountUpdate, BlockUpdate, TransactionUpdate, Update},
    },
    std::{collections::HashSet, sync::Arc, time::Duration},
    tokio::{
        sync::{mpsc, watch},
        task::{AbortHandle, JoinError, JoinHandle, JoinSet},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("datasource {datasource_id} failed: {source}")]
    Datasource { datasource_id: Id, source: BoxError },
    #[error("invalid transaction from datasource {datasource_id}: {source}")]
    Transform {
        datasource_id: Id,
        source: TransformError,
    },
    #[error("decoding failed in route {route_id} for datasource {datasource_id}: {source}")]
    Decode {
        datasource_id: Id,
        route_id: Id,
        source: BoxError,
    },
    #[error("filter failed in route {route_id} for datasource {datasource_id}: {source}")]
    Filter {
        datasource_id: Id,
        route_id: Id,
        source: BoxError,
    },
    #[error("processor failed in route {route_id} for datasource {datasource_id}: {source}")]
    Processor {
        datasource_id: Id,
        route_id: Id,
        source: BoxError,
    },
    #[error("filter commit failed in route {route_id} for datasource {datasource_id}: {source}")]
    FilterCommit {
        datasource_id: Id,
        route_id: Id,
        source: BoxError,
    },
    #[error("pipeline task panicked: {message:?}")]
    Panicked { message: Option<String> },
    #[error("pipeline shutdown timed out after {timeout:?}")]
    ShutdownTimedOut { timeout: Duration },
    #[error("pipeline failed: {source}")]
    Framework { source: BoxError },
}

impl PipelineError {
    fn route(error: Error, context: &RouteContext<'_>) -> Self {
        let datasource_id = context.datasource_id().clone();
        let route_id = context.route_id().clone();
        match error {
            Error::Decode(source) => Self::Decode {
                datasource_id,
                route_id,
                source,
            },
            Error::Filter(source) => Self::Filter {
                datasource_id,
                route_id,
                source,
            },
            Error::Processor(source) => Self::Processor {
                datasource_id,
                route_id,
                source,
            },
            Error::FilterCommit(source) => Self::FilterCommit {
                datasource_id,
                route_id,
                source,
            },
            source => Self::Framework {
                source: Box::new(source),
            },
        }
    }
}

impl From<JoinError> for PipelineError {
    fn from(error: JoinError) -> Self {
        if !error.is_panic() {
            return Self::Framework {
                source: Box::new(error),
            };
        }
        let payload = error.into_panic();
        let message = payload.downcast_ref::<String>().cloned().or_else(|| {
            payload
                .downcast_ref::<&str>()
                .map(|message| message.to_string())
        });
        Self::Panicked { message }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShutdownMode {
    Drain,
    Drop,
}

#[derive(Debug, thiserror::Error)]
#[error("pipeline has stopped")]
pub struct PipelineStopped;

/// Requests shutdown without owning the running pipeline.
#[derive(Clone)]
pub struct PipelineControl {
    shutdown: watch::Sender<Option<ShutdownMode>>,
    task: AbortHandle,
    tasks: Arc<[AbortHandle]>,
}

impl PipelineControl {
    pub fn shutdown(&self, mode: ShutdownMode) -> Result<(), PipelineStopped> {
        if self.task.is_finished() {
            return Err(PipelineStopped);
        }
        self.shutdown.send_if_modified(|current| {
            if *current == Some(ShutdownMode::Drop) || *current == Some(mode) {
                return false;
            }
            *current = Some(mode);
            true
        });
        Ok(())
    }

    pub fn abort(&self) -> Result<(), PipelineStopped> {
        self.shutdown(ShutdownMode::Drain)?;
        for task in self.tasks.iter() {
            task.abort();
        }
        Ok(())
    }
}

/// Owns the running pipeline and waits for its completion.
#[must_use]
pub struct PipelineHandle {
    task: JoinHandle<Result<(), PipelineError>>,
    control: PipelineControl,
}

impl PipelineHandle {
    pub fn control(&self) -> PipelineControl {
        self.control.clone()
    }

    pub async fn shutdown(self, mode: ShutdownMode) -> Result<(), PipelineError> {
        // If already stopped, wait still returns the pipeline's result.
        let _ = self.control.shutdown(mode);
        self.wait().await
    }

    pub async fn wait(mut self) -> Result<(), PipelineError> {
        (&mut self.task).await?
    }

    pub async fn abort(self) -> Result<(), PipelineError> {
        let _ = self.control.abort();
        self.wait().await
    }
}

impl Drop for PipelineHandle {
    fn drop(&mut self) {
        let _ = self.control.abort();
    }
}

/// Invalid pipeline configuration.
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
    #[error("invalid queue capacity {capacity} for datasource {datasource_id}")]
    InvalidQueueCapacity { datasource_id: Id, capacity: usize },
    #[error("shutdown timeout must be greater than zero")]
    ZeroShutdownTimeout,
}

fn build_routes<P: ?Sized>(
    routes: Vec<(String, Box<P>)>,
    ids: &mut HashSet<Id>,
) -> Result<Vec<(Id, Box<P>)>, PipelineBuildError> {
    routes
        .into_iter()
        .map(|(name, route)| {
            let id = Id::new(name.clone())
                .map_err(|source| PipelineBuildError::InvalidRouteId { id: name, source })?;
            if !ids.insert(id.clone()) {
                return Err(PipelineBuildError::DuplicateRouteId { id });
            }
            Ok((id, route))
        })
        .collect()
}

/// Validated configuration owning the registered datasources and routes.
pub struct Pipeline {
    id: Id,
    datasources: Vec<(Id, Box<dyn DynDatasource>, DatasourceOptions)>,
    account_routes: Vec<(Id, Box<dyn DynAccountRoute>)>,
    account_closure_routes: Vec<(Id, Box<dyn DynAccountClosureRoute>)>,
    block_routes: Vec<(Id, Box<dyn DynBlockRoute>)>,
    instruction_routes: Vec<(Id, Box<dyn DynInstructionRoute>)>,
    transaction_routes: Vec<(Id, Box<dyn DynTransactionRoute>)>,
    shutdown_timeout: Option<Duration>,
}

impl Pipeline {
    pub fn builder(id: impl Into<String>) -> PipelineBuilder {
        PipelineBuilder::new(id)
    }

    pub async fn start(mut self) -> Result<PipelineHandle, PipelineError> {
        let (shutdown, receiver) = watch::channel(None);
        let signal = ShutdownSignal::new(receiver);
        let mut tasks = JoinSet::new();
        let mut task_handles = Vec::with_capacity(self.datasources.len() + 1);
        let mut queues = Vec::with_capacity(self.datasources.len());
        for (datasource_id, datasource, options) in self.datasources.drain(..) {
            let (sender, receiver) = mpsc::channel(options.queue_capacity);
            let context = DatasourceContext::new(
                self.id.clone(),
                datasource_id.clone(),
                sender,
                options.overflow_policy,
                signal.clone(),
            );
            queues.push((datasource_id.clone(), receiver));
            task_handles.push(tasks.spawn(async move {
                datasource
                    .run(context)
                    .await
                    .map_err(|source| PipelineError::Datasource {
                        datasource_id,
                        source,
                    })
            }));
        }

        let processing_shutdown = signal.clone();
        let shutdown_timeout = self.shutdown_timeout;
        task_handles.push(tasks.spawn(async move {
            let mut next_source = 0;
            let mut draining = false;
            loop {
                let next = tokio::select! {
                    biased;
                    _ = processing_shutdown.requested(), if !draining => {
                        for (_, receiver) in &mut queues {
                            receiver.close();
                        }
                        draining = true;
                        continue;
                    }
                    next = next_update(&mut queues, &mut next_source) => next,
                };
                let Some((datasource_id, queued)) = next else {
                    break;
                };
                if processing_shutdown.mode() == Some(ShutdownMode::Drop) {
                    queued.receipt_sender.send(Err(UpdateReceiptError::Dropped));
                    continue;
                }
                let result: Result<(), PipelineError> = async {
                    match &queued.update {
                        Update::Account(update) => {
                            for (route_id, route) in &mut self.account_routes {
                                let context = RouteContext::new(&self.id, &datasource_id, route_id);
                                route
                                    .run(&context, update)
                                    .await
                                    .map_err(|error| PipelineError::route(error, &context))?;
                            }
                        }
                        Update::AccountClosure(update) => {
                            for (route_id, route) in &mut self.account_closure_routes {
                                let context = RouteContext::new(&self.id, &datasource_id, route_id);
                                route
                                    .run(&context, update)
                                    .await
                                    .map_err(|error| PipelineError::route(error, &context))?;
                            }
                        }
                        Update::Block(update) => {
                            for (route_id, route) in &mut self.block_routes {
                                let context = RouteContext::new(&self.id, &datasource_id, route_id);
                                route
                                    .run(&context, update)
                                    .await
                                    .map_err(|error| PipelineError::route(error, &context))?;
                            }
                        }
                        Update::Transaction(update) => {
                            let metadata =
                                Arc::new(update.clone().try_into().map_err(|source| {
                                    PipelineError::Framework {
                                        source: Box::new(source),
                                    }
                                })?);
                            let instructions =
                                extract_instructions_with_metadata(&metadata, update)
                                    .and_then(NestedInstructions::try_from)
                                    .map_err(|source| PipelineError::Transform {
                                        datasource_id: datasource_id.clone(),
                                        source,
                                    })?;
                            let mut pending: Vec<_> = instructions.iter().rev().collect();
                            let mut all_instructions = Vec::new();
                            while let Some(instruction) = pending.pop() {
                                all_instructions.push(instruction);
                                pending.extend(instruction.inner_instructions.iter().rev());
                            }
                            for instruction in &all_instructions {
                                for (route_id, route) in &mut self.instruction_routes {
                                    let context =
                                        RouteContext::new(&self.id, &datasource_id, route_id);
                                    route
                                        .run(&context, instruction)
                                        .await
                                        .map_err(|error| PipelineError::route(error, &context))?;
                                }
                            }
                            for (route_id, route) in &mut self.transaction_routes {
                                let context = RouteContext::new(&self.id, &datasource_id, route_id);
                                route
                                    .run(&context, update, &all_instructions)
                                    .await
                                    .map_err(|error| PipelineError::route(error, &context))?;
                            }
                        }
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = result {
                    queued.receipt_sender.send(Err(UpdateReceiptError::Failed));
                    return Err(error);
                }
                queued.receipt_sender.send(Ok(()));
            }
            Ok(())
        }));

        let coordinator_shutdown = shutdown.clone();
        let task = tokio::spawn(async move {
            let mut failure = None;
            let mut timed_out = false;
            let timeout = async {
                match shutdown_timeout {
                    Some(timeout) => {
                        signal.requested().await;
                        tokio::time::sleep(timeout).await;
                        timeout
                    }
                    None => std::future::pending().await,
                }
            };
            tokio::pin!(timeout);

            loop {
                let result = tokio::select! {
                    biased;
                    result = tasks.join_next() => match result {
                        Some(Ok(result)) => result,
                        Some(Err(error)) if error.is_cancelled() => continue,
                        Some(Err(error)) => Err(PipelineError::from(error)),
                        None => break,
                    },
                    timeout = &mut timeout, if !timed_out => {
                        tasks.abort_all();
                        timed_out = true;
                        Err(PipelineError::ShutdownTimedOut { timeout })
                    }
                };
                if let Err(error) = result {
                    coordinator_shutdown.send_modify(|mode| {
                        mode.get_or_insert(ShutdownMode::Drain);
                    });
                    if failure.is_none() {
                        failure = Some(error);
                    } else {
                        log::error!("additional pipeline failure: {error}");
                    }
                }
            }
            failure.map_or(Ok(()), Err)
        });
        let control = PipelineControl {
            shutdown,
            task: task.abort_handle(),
            tasks: task_handles.into(),
        };
        Ok(PipelineHandle { task, control })
    }

    pub async fn run(self) -> Result<(), PipelineError> {
        self.start().await?.wait().await
    }
}

/// Registers datasources and routes before validating their configuration.
pub struct PipelineBuilder {
    id: String,
    datasources: Vec<(String, Box<dyn DynDatasource>, DatasourceOptions)>,
    account_routes: Vec<(String, Box<dyn DynAccountRoute>)>,
    account_closure_routes: Vec<(String, Box<dyn DynAccountClosureRoute>)>,
    block_routes: Vec<(String, Box<dyn DynBlockRoute>)>,
    instruction_routes: Vec<(String, Box<dyn DynInstructionRoute>)>,
    transaction_routes: Vec<(String, Box<dyn DynTransactionRoute>)>,
    shutdown_timeout: Option<Duration>,
}

impl PipelineBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            datasources: Vec::new(),
            account_routes: Vec::new(),
            account_closure_routes: Vec::new(),
            block_routes: Vec::new(),
            instruction_routes: Vec::new(),
            transaction_routes: Vec::new(),
            shutdown_timeout: None,
        }
    }

    pub fn datasource(self, id: impl Into<String>, datasource: impl Datasource) -> Self {
        self.datasource_with_options(id, datasource, DatasourceOptions::default())
    }

    pub fn datasource_with_options(
        mut self,
        id: impl Into<String>,
        datasource: impl Datasource,
        options: DatasourceOptions,
    ) -> Self {
        self.datasources
            .push((id.into(), Box::new(datasource), options));
        self
    }

    /// Limits orderly shutdown before escalating to abort. Unset by default.
    pub fn shutdown_timeout(mut self, timeout: Duration) -> Self {
        self.shutdown_timeout = Some(timeout);
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

    pub fn account_closure<P>(self, route_id: impl Into<String>, processor: P) -> Self
    where
        P: Processor<AccountClosureUpdate> + 'static,
    {
        self.account_closure_with_options(route_id, processor, RouteOptions::default())
    }

    pub fn account_closure_with_options<P>(
        mut self,
        route_id: impl Into<String>,
        processor: P,
        options: RouteOptions<AccountClosureUpdate>,
    ) -> Self
    where
        P: Processor<AccountClosureUpdate> + 'static,
    {
        self.account_closure_routes.push((
            route_id.into(),
            Box::new(AccountClosureRoute::new(processor, options)),
        ));
        self
    }

    pub fn block_details<P>(self, route_id: impl Into<String>, processor: P) -> Self
    where
        P: Processor<BlockUpdate> + 'static,
    {
        self.block_details_with_options(route_id, processor, RouteOptions::default())
    }

    pub fn block_details_with_options<P>(
        mut self,
        route_id: impl Into<String>,
        processor: P,
        options: RouteOptions<BlockUpdate>,
    ) -> Self
    where
        P: Processor<BlockUpdate> + 'static,
    {
        self.block_routes.push((
            route_id.into(),
            Box::new(BlockRoute::new(processor, options)),
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

    pub fn transaction<C, P>(self, route_id: impl Into<String>, processor: P) -> Self
    where
        C: InstructionDecoderCollection + Send + Sync + 'static,
        P: for<'a> Processor<TransactionProcessorInput<'a, C>> + 'static,
    {
        self.transaction_with_options::<C, P>(route_id, processor, DecodedRouteOptions::default())
    }

    pub fn transaction_with_options<C, P>(
        mut self,
        route_id: impl Into<String>,
        processor: P,
        options: DecodedRouteOptions<TransactionUpdate>,
    ) -> Self
    where
        C: InstructionDecoderCollection + Send + Sync + 'static,
        P: for<'a> Processor<TransactionProcessorInput<'a, C>> + 'static,
    {
        self.transaction_routes.push((
            route_id.into(),
            Box::new(TransactionRoute::<C, P>::new(processor, options)),
        ));
        self
    }

    /// Validates configuration without creating channels or starting work.
    pub fn build(self) -> Result<Pipeline, PipelineBuildError> {
        let id =
            Id::new(self.id).map_err(|source| PipelineBuildError::InvalidPipelineId { source })?;
        if self
            .shutdown_timeout
            .is_some_and(|timeout| timeout.is_zero())
        {
            return Err(PipelineBuildError::ZeroShutdownTimeout);
        }
        let mut source_ids = HashSet::new();
        let mut datasources = Vec::with_capacity(self.datasources.len());
        for (registration_index, (name, datasource, options)) in
            self.datasources.into_iter().enumerate()
        {
            let id = Id::new(name).map_err(|source| PipelineBuildError::InvalidDatasourceId {
                registration_index,
                source,
            })?;
            if !source_ids.insert(id.clone()) {
                return Err(PipelineBuildError::DuplicateDatasourceId { id });
            }
            if !options.capacity_is_valid() {
                return Err(PipelineBuildError::InvalidQueueCapacity {
                    datasource_id: id,
                    capacity: options.queue_capacity,
                });
            }
            datasources.push((id, datasource, options));
        }
        let mut route_ids = HashSet::new();
        Ok(Pipeline {
            id,
            datasources,
            account_routes: build_routes(self.account_routes, &mut route_ids)?,
            account_closure_routes: build_routes(self.account_closure_routes, &mut route_ids)?,
            block_routes: build_routes(self.block_routes, &mut route_ids)?,
            instruction_routes: build_routes(self.instruction_routes, &mut route_ids)?,
            transaction_routes: build_routes(self.transaction_routes, &mut route_ids)?,
            shutdown_timeout: self.shutdown_timeout,
        })
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            datasource::{DatasourceContext, OverflowPolicy},
            error::BoxError,
            processor::ProcessorResult,
            route::RouteContext,
        },
        solana_account::Account,
        solana_instruction::Instruction,
        solana_pubkey::Pubkey,
        std::cell::Cell,
    };

    struct Source;

    impl Datasource for Source {
        async fn run(self, _context: DatasourceContext) -> Result<(), BoxError> {
            panic!("building a pipeline must not run a datasource")
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

    struct Recorder(Cell<usize>);

    impl<T: Sync> Processor<T> for Recorder {
        async fn process(&mut self, _context: &RouteContext<'_>, _value: &T) -> ProcessorResult {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
    }

    fn recorder() -> Recorder {
        Recorder(Cell::new(0))
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
            Pipeline::builder("pipeline").transaction_with_options::<Collection, _>(
                " ",
                recorder(),
                DecodedRouteOptions::default(),
            ),
            Pipeline::builder("pipeline").account_closure(" ", recorder()),
            Pipeline::builder("pipeline").account_closure_with_options(
                " ",
                recorder(),
                RouteOptions::default(),
            ),
            Pipeline::builder("pipeline").block_details(" ", recorder()),
            Pipeline::builder("pipeline").block_details_with_options(
                " ",
                recorder(),
                RouteOptions::default(),
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
    fn build_owns_configuration_without_a_runtime() {
        fn assert_send<T: Send>(_: &T) {}

        let options = DatasourceOptions::default()
            .queue_capacity(32)
            .overflow_policy(OverflowPolicy::Drop);
        let timeout = Duration::from_secs(5);
        let pipeline = Pipeline::builder("pipeline")
            .datasource("live", Source)
            .datasource_with_options("history", Source, options)
            .account("accounts", Decoder, recorder())
            .account_closure("closures", recorder())
            .block_details("blocks", recorder())
            .instruction("instructions", Decoder, recorder())
            .transaction::<Collection, _>("transactions", recorder())
            .shutdown_timeout(timeout)
            .build()
            .unwrap();

        assert_send(&pipeline);
        assert_eq!(pipeline.id.as_str(), "pipeline");
        assert_eq!(pipeline.datasources.len(), 2);
        assert_eq!(pipeline.datasources[0].0.as_str(), "live");
        assert_eq!(pipeline.datasources[0].2, DatasourceOptions::default());
        assert_eq!(pipeline.datasources[1].0.as_str(), "history");
        assert_eq!(pipeline.datasources[1].2, options);
        assert_eq!(pipeline.account_routes[0].0.as_str(), "accounts");
        assert_eq!(pipeline.account_closure_routes[0].0.as_str(), "closures");
        assert_eq!(pipeline.block_routes[0].0.as_str(), "blocks");
        assert_eq!(pipeline.instruction_routes[0].0.as_str(), "instructions");
        assert_eq!(pipeline.transaction_routes[0].0.as_str(), "transactions");
        assert_eq!(pipeline.shutdown_timeout, Some(timeout));
        assert_eq!(
            Pipeline::builder("empty").build().unwrap().shutdown_timeout,
            None
        );
    }

    #[test]
    fn build_rejects_invalid_source_options_and_timeout() {
        for capacity in [0, tokio::sync::Semaphore::MAX_PERMITS + 1, usize::MAX] {
            let result = Pipeline::builder("pipeline")
                .datasource_with_options(
                    "source",
                    Source,
                    DatasourceOptions::default().queue_capacity(capacity),
                )
                .build();
            assert!(matches!(
                result,
                Err(PipelineBuildError::InvalidQueueCapacity { datasource_id, capacity: value })
                    if datasource_id.as_str() == "source" && value == capacity
            ));
        }
        for capacity in [1, tokio::sync::Semaphore::MAX_PERMITS] {
            assert!(Pipeline::builder("pipeline")
                .datasource_with_options(
                    "source",
                    Source,
                    DatasourceOptions::default().queue_capacity(capacity),
                )
                .build()
                .is_ok());
        }
        assert!(matches!(
            Pipeline::builder("pipeline")
                .shutdown_timeout(Duration::ZERO)
                .build(),
            Err(PipelineBuildError::ZeroShutdownTimeout)
        ));
    }

    struct FiniteSource {
        updates: Vec<Update>,
        receipts: tokio::sync::oneshot::Sender<Vec<crate::datasource::receipt::UpdateReceipt>>,
    }

    impl Datasource for FiniteSource {
        async fn run(self, mut context: DatasourceContext) -> Result<(), BoxError> {
            let mut receipts = Vec::new();
            for update in self.updates {
                receipts.push(context.emit(update).await?.unwrap());
            }
            self.receipts.send(receipts).unwrap();
            Ok(())
        }
    }

    struct Observer {
        calls: Cell<usize>,
        seen: Arc<std::sync::Mutex<Vec<(String, usize)>>>,
    }

    impl<T: Sync> Processor<T> for Observer {
        async fn process(&mut self, context: &RouteContext<'_>, _value: &T) -> ProcessorResult {
            assert_eq!(context.pipeline_id().as_str(), "indexer");
            assert_eq!(context.datasource_id().as_str(), "source");
            self.calls.set(self.calls.get() + 1);
            self.seen
                .lock()
                .unwrap()
                .push((context.route_id().to_string(), self.calls.get()));
            tokio::task::yield_now().await;
            Ok(())
        }
    }

    #[tokio::test]
    async fn finite_source_drains_every_route_and_settles_receipts() {
        use {
            solana_message::{
                compiled_instruction::CompiledInstruction, legacy::Message, VersionedMessage,
            },
            solana_signature::Signature,
            solana_transaction::versioned::VersionedTransaction,
            solana_transaction_status::{
                InnerInstruction, InnerInstructions, TransactionStatusMeta,
            },
        };

        let instruction = CompiledInstruction {
            program_id_index: 0,
            accounts: vec![0],
            data: vec![7],
        };
        let transaction = TransactionUpdate::new(
            VersionedTransaction {
                signatures: vec![Signature::default()],
                message: VersionedMessage::Legacy(Message {
                    header: solana_message::MessageHeader {
                        num_required_signatures: 1,
                        ..Default::default()
                    },
                    account_keys: vec![Pubkey::new_unique()],
                    instructions: vec![instruction.clone()],
                    ..Message::default()
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
                ..TransactionStatusMeta::default()
            },
            1,
        )
        .unwrap();
        let updates = vec![
            AccountUpdate::new(
                Pubkey::new_unique(),
                Account {
                    lamports: 1,
                    ..Account::default()
                },
                1,
            )
            .into(),
            AccountClosureUpdate::new(Pubkey::new_unique(), Account::default(), 1)
                .unwrap()
                .into(),
            BlockUpdate::new(1).into(),
            transaction.into(),
        ];
        let (receipts, received) = tokio::sync::oneshot::channel();
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let observer = || Observer {
            calls: Cell::new(0),
            seen: seen.clone(),
        };
        let pipeline = Pipeline::builder("indexer")
            .datasource_with_options(
                "source",
                FiniteSource { updates, receipts },
                DatasourceOptions::default().queue_capacity(1),
            )
            .account("accounts", Decoder, observer())
            .account_closure("closures", observer())
            .block_details("blocks", observer())
            .instruction("first", Decoder, observer())
            .instruction("second", Decoder, observer())
            .transaction::<Collection, _>("transactions", observer())
            .build()
            .unwrap();

        tokio::time::timeout(Duration::from_secs(1), pipeline.run())
            .await
            .unwrap()
            .unwrap();

        let receipts = received.await.unwrap();
        assert_eq!(receipts.len(), 4);
        for receipt in receipts {
            assert_eq!(receipt.processed().await, Ok(()));
        }
        assert_eq!(
            *seen.lock().unwrap(),
            [
                ("accounts", 1),
                ("closures", 1),
                ("blocks", 1),
                ("first", 1),
                ("second", 1),
                ("first", 2),
                ("second", 2),
                ("transactions", 1),
            ]
            .map(|(id, count)| (id.to_owned(), count))
        );
    }

    struct AwaitingSource {
        grouped: bool,
        finished: tokio::sync::oneshot::Sender<()>,
    }

    impl Datasource for AwaitingSource {
        async fn run(self, mut context: DatasourceContext) -> Result<(), BoxError> {
            let receipt = if self.grouped {
                let mut group = context.begin_group();
                group.emit(BlockUpdate::new(1).into()).await?;
                group.emit(BlockUpdate::new(2).into()).await?;
                group.seal()
            } else {
                context.emit(BlockUpdate::new(1).into()).await?.unwrap()
            };
            receipt.processed().await?;
            self.finished.send(()).unwrap();
            Ok(())
        }
    }

    #[tokio::test]
    async fn sources_can_await_receipts_before_wait_is_called() {
        let mut builder = Pipeline::builder("pipeline").block_details("blocks", recorder());
        let mut finished_sources = Vec::new();
        for (name, grouped) in [("single", false), ("group", true)] {
            let (finished, received) = tokio::sync::oneshot::channel();
            finished_sources.push(received);
            builder = builder.datasource_with_options(
                name,
                AwaitingSource { grouped, finished },
                DatasourceOptions::default().queue_capacity(1),
            );
        }
        let handle = builder.build().unwrap().start().await.unwrap();
        for finished in finished_sources {
            tokio::time::timeout(Duration::from_secs(1), finished)
                .await
                .unwrap()
                .unwrap();
        }
        handle.wait().await.unwrap();
    }

    struct WaitingProcessor {
        entered: Option<tokio::sync::oneshot::Sender<()>>,
        release: tokio::sync::oneshot::Receiver<()>,
    }

    impl Processor<BlockUpdate> for WaitingProcessor {
        async fn process(
            &mut self,
            _context: &RouteContext<'_>,
            _update: &BlockUpdate,
        ) -> ProcessorResult {
            if let Some(entered) = self.entered.take() {
                entered.send(()).unwrap();
                (&mut self.release).await?;
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn wait_keeps_running_after_source_exit_until_processing_finishes() {
        use std::{
            future::{poll_fn, Future},
            pin::pin,
            task::Poll,
        };

        let (receipts, received) = tokio::sync::oneshot::channel();
        let (entered, processing) = tokio::sync::oneshot::channel();
        let (release, released) = tokio::sync::oneshot::channel();
        let handle = Pipeline::builder("pipeline")
            .datasource(
                "source",
                FiniteSource {
                    updates: vec![BlockUpdate::new(1).into(), BlockUpdate::new(2).into()],
                    receipts,
                },
            )
            .block_details(
                "blocks",
                WaitingProcessor {
                    entered: Some(entered),
                    release: released,
                },
            )
            .build()
            .unwrap()
            .start()
            .await
            .unwrap();

        let receipts = received.await.unwrap();
        processing.await.unwrap();
        let mut wait = pin!(handle.wait());
        poll_fn(|cx| {
            assert!(wait.as_mut().poll(cx).is_pending());
            Poll::Ready(())
        })
        .await;
        release.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(1), wait)
            .await
            .unwrap()
            .unwrap();
        for receipt in receipts {
            assert_eq!(receipt.processed().await, Ok(()));
        }
    }

    struct DrainSource {
        ready: tokio::sync::oneshot::Sender<()>,
        stopped: tokio::sync::oneshot::Sender<crate::datasource::EmitError>,
        processed: tokio::sync::oneshot::Sender<()>,
        finish: tokio::sync::oneshot::Receiver<()>,
    }

    impl Datasource for DrainSource {
        async fn run(self, mut context: DatasourceContext) -> Result<(), BoxError> {
            let first = context.emit(BlockUpdate::new(1).into()).await?.unwrap();
            let second = context.emit(BlockUpdate::new(2).into()).await?.unwrap();
            self.ready.send(()).unwrap();
            let error = context.emit(BlockUpdate::new(3).into()).await.unwrap_err();
            self.stopped.send(error).unwrap();
            assert!(matches!(
                context.emit(BlockUpdate::new(4).into()).await,
                Err(crate::datasource::EmitError::ShuttingDown)
            ));
            first.processed().await?;
            second.processed().await?;
            self.processed.send(()).unwrap();
            self.finish.await?;
            Ok(())
        }
    }

    #[tokio::test]
    async fn drain_stops_emission_finishes_accepted_work_and_joins_the_source() {
        use std::{
            future::{poll_fn, Future},
            pin::pin,
            task::Poll,
        };

        let (ready, accepted) = tokio::sync::oneshot::channel();
        let (stopped, rejected) = tokio::sync::oneshot::channel();
        let (processed, receipts) = tokio::sync::oneshot::channel();
        let (finish, finishing) = tokio::sync::oneshot::channel();
        let (entered, processing) = tokio::sync::oneshot::channel();
        let (release, released) = tokio::sync::oneshot::channel();
        let handle = Pipeline::builder("pipeline")
            .datasource_with_options(
                "source",
                DrainSource {
                    ready,
                    stopped,
                    processed,
                    finish: finishing,
                },
                DatasourceOptions::default().queue_capacity(1),
            )
            .block_details(
                "blocks",
                WaitingProcessor {
                    entered: Some(entered),
                    release: released,
                },
            )
            .build()
            .unwrap()
            .start()
            .await
            .unwrap();

        tokio::time::timeout(Duration::from_secs(1), accepted)
            .await
            .unwrap()
            .unwrap();
        processing.await.unwrap();
        let control = handle.control();
        control.shutdown(ShutdownMode::Drain).unwrap();
        control.clone().shutdown(ShutdownMode::Drain).unwrap();
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), rejected)
                .await
                .unwrap()
                .unwrap(),
            crate::datasource::EmitError::ShuttingDown
        );

        let mut drain = pin!(handle.shutdown(ShutdownMode::Drain));
        poll_fn(|cx| {
            assert!(drain.as_mut().poll(cx).is_pending());
            Poll::Ready(())
        })
        .await;
        release.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(1), receipts)
            .await
            .unwrap()
            .unwrap();
        poll_fn(|cx| {
            assert!(drain.as_mut().poll(cx).is_pending());
            Poll::Ready(())
        })
        .await;
        finish.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(1), drain)
            .await
            .unwrap()
            .unwrap();
        assert!(control.shutdown(ShutdownMode::Drain).is_err());
    }

    impl crate::filter::Filter<BlockUpdate> for Observer {
        async fn filter(
            &mut self,
            _context: &RouteContext<'_>,
            _update: &BlockUpdate,
        ) -> Result<bool, BoxError> {
            Ok(true)
        }

        async fn commit(
            &mut self,
            context: &RouteContext<'_>,
            update: &BlockUpdate,
            result: &ProcessorResult,
        ) -> Result<(), BoxError> {
            assert!(result.is_ok());
            self.seen.lock().unwrap().push((
                format!("{} commit", context.route_id()),
                update.slot() as usize,
            ));
            Ok(())
        }
    }

    #[tokio::test]
    async fn drop_finishes_the_current_update_and_discards_the_rest() {
        for escalate in [false, true] {
            let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
            let observer = || Observer {
                calls: Cell::new(0),
                seen: seen.clone(),
            };
            let (receipts, received) = tokio::sync::oneshot::channel();
            let (entered, processing) = tokio::sync::oneshot::channel();
            let (release, released) = tokio::sync::oneshot::channel();
            let handle = Pipeline::builder("indexer")
                .datasource(
                    "source",
                    FiniteSource {
                        updates: vec![BlockUpdate::new(1).into(), BlockUpdate::new(2).into()],
                        receipts,
                    },
                )
                .block_details_with_options(
                    "first",
                    WaitingProcessor {
                        entered: Some(entered),
                        release: released,
                    },
                    RouteOptions::default().filter(observer()),
                )
                .block_details_with_options(
                    "last",
                    observer(),
                    RouteOptions::default().filter(observer()),
                )
                .build()
                .unwrap()
                .start()
                .await
                .unwrap();

            let mut receipts = received.await.unwrap().into_iter();
            processing.await.unwrap();
            let control = handle.control();
            if escalate {
                control.shutdown(ShutdownMode::Drain).unwrap();
            }
            control.shutdown(ShutdownMode::Drop).unwrap();
            control.clone().shutdown(ShutdownMode::Drop).unwrap();
            control.shutdown(ShutdownMode::Drain).unwrap();
            assert!(seen.lock().unwrap().is_empty());
            release.send(()).unwrap();

            tokio::time::timeout(Duration::from_secs(1), handle.wait())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(receipts.next().unwrap().processed().await, Ok(()));
            assert_eq!(
                receipts.next().unwrap().processed().await,
                Err(UpdateReceiptError::Dropped)
            );
            assert_eq!(
                *seen.lock().unwrap(),
                vec![
                    ("first commit".into(), 1),
                    ("last".into(), 1),
                    ("last commit".into(), 1)
                ]
            );
            assert!(control.shutdown(ShutdownMode::Drop).is_err());
        }
    }

    struct IdleSource(tokio::sync::oneshot::Sender<()>);

    impl Datasource for IdleSource {
        async fn run(self, context: DatasourceContext) -> Result<(), BoxError> {
            self.0.send(()).unwrap();
            context.shutdown().requested().await;
            Ok(())
        }
    }

    #[tokio::test]
    async fn shutdown_wakes_idle_sources_and_empty_receivers() {
        for mode in [ShutdownMode::Drain, ShutdownMode::Drop] {
            let (started, ready) = tokio::sync::oneshot::channel();
            let handle = Pipeline::builder("pipeline")
                .datasource("idle", IdleSource(started))
                .build()
                .unwrap()
                .start()
                .await
                .unwrap();
            ready.await.unwrap();
            tokio::time::timeout(Duration::from_secs(1), handle.shutdown(mode))
                .await
                .unwrap()
                .unwrap();
        }
    }

    struct GroupedDrainSource(tokio::sync::oneshot::Sender<()>);

    impl Datasource for GroupedDrainSource {
        async fn run(self, mut context: DatasourceContext) -> Result<(), BoxError> {
            let shutdown = context.shutdown().clone();
            let mut group = context.begin_group();
            group.emit(BlockUpdate::new(1).into()).await?;
            group.emit(BlockUpdate::new(2).into()).await?;
            self.0.send(()).unwrap();
            shutdown.requested().await;
            group.seal().processed().await?;
            Ok(())
        }
    }

    #[tokio::test]
    async fn a_source_can_seal_and_await_its_group_during_drain() {
        let (started, ready) = tokio::sync::oneshot::channel();
        let handle = Pipeline::builder("pipeline")
            .datasource("source", GroupedDrainSource(started))
            .block_details("blocks", recorder())
            .build()
            .unwrap()
            .start()
            .await
            .unwrap();
        ready.await.unwrap();
        tokio::time::timeout(Duration::from_secs(1), handle.shutdown(ShutdownMode::Drain))
            .await
            .unwrap()
            .unwrap();
    }

    #[tokio::test]
    async fn abort_before_tasks_start_does_not_run_the_source() {
        Pipeline::builder("pipeline")
            .datasource("source", Source)
            .build()
            .unwrap()
            .start()
            .await
            .unwrap()
            .abort()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn abort_and_handle_drop_cancel_tasks_and_settle_receipts() {
        for drop_handle in [false, true] {
            for grouped in [false, true] {
                let (receipts, received) = tokio::sync::oneshot::channel();
                let (mut source_release, source_wait) = tokio::sync::oneshot::channel();
                let (entered, processing) = tokio::sync::oneshot::channel();
                let (mut release, released) = tokio::sync::oneshot::channel();
                let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
                let observer = || Observer {
                    calls: Cell::new(0),
                    seen: seen.clone(),
                };
                let handle = Pipeline::builder("indexer")
                    .datasource(
                        "source",
                        HeldSource {
                            receipts,
                            release: source_wait,
                            grouped,
                        },
                    )
                    .block_details_with_options(
                        "first",
                        WaitingProcessor {
                            entered: Some(entered),
                            release: released,
                        },
                        RouteOptions::default().filter(observer()),
                    )
                    .block_details("last", observer())
                    .build()
                    .unwrap()
                    .start()
                    .await
                    .unwrap();
                let receipts = received.await.unwrap();
                processing.await.unwrap();
                let control = handle.control();

                if drop_handle {
                    drop(handle);
                    tokio::time::timeout(Duration::from_secs(1), async {
                        source_release.closed().await;
                        release.closed().await;
                        while !control.task.is_finished() {
                            tokio::task::yield_now().await;
                        }
                    })
                    .await
                    .unwrap();
                } else {
                    control.abort().unwrap();
                    control.clone().abort().unwrap();
                    control.shutdown(ShutdownMode::Drop).unwrap();
                    tokio::time::timeout(Duration::from_secs(1), handle.abort())
                        .await
                        .unwrap()
                        .unwrap();
                }

                assert!(source_release.is_closed());
                assert!(release.is_closed());
                assert!(seen.lock().unwrap().is_empty());
                for receipt in receipts {
                    assert_eq!(receipt.processed().await, Err(UpdateReceiptError::Aborted));
                }
                assert!(control.abort().is_err());
                assert!(control.shutdown(ShutdownMode::Drain).is_err());
            }
        }
    }

    struct HeldSource {
        receipts: tokio::sync::oneshot::Sender<Vec<crate::datasource::receipt::UpdateReceipt>>,
        release: tokio::sync::oneshot::Receiver<()>,
        grouped: bool,
    }

    impl Datasource for HeldSource {
        async fn run(self, mut context: DatasourceContext) -> Result<(), BoxError> {
            let receipts = if self.grouped {
                let mut group = context.begin_group();
                group.emit(BlockUpdate::new(1).into()).await?;
                group.emit(BlockUpdate::new(2).into()).await?;
                vec![group.seal()]
            } else {
                vec![
                    context.emit(BlockUpdate::new(1).into()).await?.unwrap(),
                    context.emit(BlockUpdate::new(2).into()).await?.unwrap(),
                ]
            };
            self.receipts.send(receipts).unwrap();
            self.release.await?;
            Ok(())
        }
    }

    struct EscapingSource(tokio::sync::oneshot::Sender<DatasourceContext>);

    impl Datasource for EscapingSource {
        async fn run(self, context: DatasourceContext) -> Result<(), BoxError> {
            // Deliberately violate ownership to verify that abort revokes emission.
            assert!(self.0.send(context).is_ok());
            std::future::pending().await
        }
    }

    #[tokio::test]
    async fn cancelling_wait_revokes_emission_and_leaves_cleanup_running() {
        use std::{
            future::{poll_fn, Future},
            pin::pin,
            task::Poll,
        };

        let (context, received) = tokio::sync::oneshot::channel();
        let handle = Pipeline::builder("pipeline")
            .datasource("source", EscapingSource(context))
            .build()
            .unwrap()
            .start()
            .await
            .unwrap();
        let mut context = received.await.unwrap();
        let control = handle.control();
        {
            let mut wait = pin!(handle.wait());
            poll_fn(|cx| {
                assert!(wait.as_mut().poll(cx).is_pending());
                Poll::Ready(())
            })
            .await;
        }
        // No scheduler yield is needed before admission is revoked.
        assert!(context.shutdown().is_requested());
        assert!(matches!(
            context.emit(BlockUpdate::new(1).into()).await,
            Err(crate::datasource::EmitError::ShuttingDown)
        ));
        tokio::time::timeout(Duration::from_secs(1), async {
            while !control.task.is_finished() {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert!(control.abort().is_err());
    }

    #[tokio::test]
    async fn timeout_starts_at_shutdown_and_aborts_unfinished_work() {
        for mode in [ShutdownMode::Drain, ShutdownMode::Drop] {
            let timeout = Duration::from_millis(20);
            let (receipts, received) = tokio::sync::oneshot::channel();
            let (source_release, source_wait) = tokio::sync::oneshot::channel();
            let (entered, processing) = tokio::sync::oneshot::channel();
            let (release, released) = tokio::sync::oneshot::channel();
            let handle = Pipeline::builder("pipeline")
                .datasource(
                    "source",
                    HeldSource {
                        receipts,
                        release: source_wait,
                        grouped: false,
                    },
                )
                .block_details(
                    "blocks",
                    WaitingProcessor {
                        entered: Some(entered),
                        release: released,
                    },
                )
                .shutdown_timeout(timeout)
                .build()
                .unwrap()
                .start()
                .await
                .unwrap();
            let receipts = received.await.unwrap();
            processing.await.unwrap();
            let control = handle.control();
            tokio::time::sleep(timeout * 2).await;
            assert!(!control.task.is_finished());
            control.shutdown(mode).unwrap();
            assert!(matches!(
                tokio::time::timeout(Duration::from_secs(1), handle.wait()).await.unwrap(),
                Err(PipelineError::ShutdownTimedOut { timeout: elapsed }) if elapsed == timeout
            ));
            assert!(source_release.is_closed());
            assert!(release.is_closed());
            for receipt in receipts {
                assert_eq!(receipt.processed().await, Err(UpdateReceiptError::Aborted));
            }
        }
    }

    struct FailingSource {
        fail: tokio::sync::oneshot::Receiver<()>,
        panic: bool,
    }

    impl Datasource for FailingSource {
        async fn run(self, _context: DatasourceContext) -> Result<(), BoxError> {
            self.fail.await?;
            assert!(!self.panic, "source panic");
            Err(std::io::Error::other("source failure").into())
        }
    }

    #[tokio::test]
    async fn cleanup_preserves_an_earlier_error_or_panic() {
        for abort in [false, true] {
            for panic in [false, true] {
                let (fail, failing) = tokio::sync::oneshot::channel();
                let (context, received) = tokio::sync::oneshot::channel();
                let handle = Pipeline::builder("pipeline")
                    .datasource(
                        "failing",
                        FailingSource {
                            fail: failing,
                            panic,
                        },
                    )
                    .datasource("held", EscapingSource(context))
                    .shutdown_timeout(Duration::from_millis(20))
                    .build()
                    .unwrap()
                    .start()
                    .await
                    .unwrap();
                let context = received.await.unwrap();
                let control = handle.control();
                fail.send(()).unwrap();
                if abort {
                    context.shutdown().requested().await;
                    control.abort().unwrap();
                }
                let error = tokio::time::timeout(Duration::from_secs(1), handle.wait())
                    .await
                    .unwrap()
                    .unwrap_err();
                if panic {
                    assert!(
                        matches!(error, PipelineError::Panicked { message: Some(message) } if message == "source panic")
                    );
                } else {
                    assert!(
                        matches!(error, PipelineError::Datasource { datasource_id, source }
                    if datasource_id.as_str() == "failing" && source.to_string() == "source failure")
                    );
                }
                assert!(context.shutdown().is_requested());
            }
        }
    }

    #[tokio::test]
    async fn empty_pipeline_completes() {
        Pipeline::builder("empty")
            .build()
            .unwrap()
            .run()
            .await
            .unwrap();
    }
}
