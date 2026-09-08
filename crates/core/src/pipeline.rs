//! Pipeline configuration and registration.

use {
    crate::{
        account::AccountDecoder,
        collection::InstructionDecoderCollection,
        datasource::{Datasource, DatasourceOptions, DynDatasource},
        id::{Id, IdError},
        instruction::{InstructionDecoder, NestedInstruction},
        processor::Processor,
        route::{
            AccountClosureRoute, AccountProcessorInput, AccountRoute, BlockRoute,
            DecodedRouteOptions, DynAccountClosureRoute, DynAccountRoute, DynBlockRoute,
            DynInstructionRoute, DynTransactionRoute, InstructionProcessorInput, InstructionRoute,
            RouteOptions, TransactionProcessorInput, TransactionRoute,
        },
        update::{AccountClosureUpdate, AccountUpdate, BlockUpdate, TransactionUpdate},
    },
    std::{collections::HashSet, time::Duration},
};

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
}
