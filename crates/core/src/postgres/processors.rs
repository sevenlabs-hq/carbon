use crate::{
    account::{AccountMetadata, AccountProcessorInputType},
    error::CarbonResult,
    instruction::{InstructionMetadata, InstructionProcessorInputType},
    metrics::{Counter, Histogram, MetricsRegistry},
    postgres::{
        operations::Upsert,
        rows::{AccountRow, InstructionRow},
    },
};
use std::sync::OnceLock;

static POSTGRES_ACCOUNTS_UPSERTED: Counter = Counter::new(
    "postgres.accounts.upsert.upserted",
    "Total number of account upserts successfully completed",
);

static POSTGRES_ACCOUNTS_UPSERT_FAILED: Counter = Counter::new(
    "postgres.accounts.upsert.failed",
    "Total number of account upserts that failed",
);

static POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS: OnceLock<Histogram> = OnceLock::new();

static POSTGRES_INSTRUCTIONS_UPSERTED: Counter = Counter::new(
    "postgres.instructions.upsert.upserted",
    "Total number of instruction upserts successfully completed",
);

static POSTGRES_INSTRUCTIONS_UPSERT_FAILED: Counter = Counter::new(
    "postgres.instructions.upsert.failed",
    "Total number of instruction upserts that failed",
);

static POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS: OnceLock<Histogram> = OnceLock::new();

fn init_postgres_histograms() {
    POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS.get_or_init(|| {
        Histogram::new(
            "postgres.accounts.upsert.duration_milliseconds",
            "Duration of account upsert operations in milliseconds",
            vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0],
        )
    });
    POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS.get_or_init(|| {
        Histogram::new(
            "postgres.instructions.upsert.duration_milliseconds",
            "Duration of instruction upsert operations in milliseconds",
            vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0],
        )
    });
}

pub fn register_postgres_metrics() {
    init_postgres_histograms();
    let registry = MetricsRegistry::global();
    registry.register_counter(&POSTGRES_ACCOUNTS_UPSERTED);
    registry.register_counter(&POSTGRES_ACCOUNTS_UPSERT_FAILED);
    registry.register_histogram(POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS.get().unwrap());
    registry.register_counter(&POSTGRES_INSTRUCTIONS_UPSERTED);
    registry.register_counter(&POSTGRES_INSTRUCTIONS_UPSERT_FAILED);
    registry.register_histogram(POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS.get().unwrap());
}

pub struct PostgresAccountProcessor<T, W> {
    pool: sqlx::PgPool,
    _phantom: std::marker::PhantomData<(T, W)>,
}

impl<T, W> PostgresAccountProcessor<T, W> {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T, W> crate::processor::Processor for PostgresAccountProcessor<T, W>
where
    T: Clone + Send + Sync + 'static,
    W: From<(T, AccountMetadata)> + Upsert + Send + 'static,
{
    type InputType = AccountProcessorInputType<T>;

    async fn process(&mut self, input: Self::InputType) -> CarbonResult<()> {
        let (metadata, decoded_account, _raw) = input;

        let start = std::time::Instant::now();

        let wrapper = W::from((decoded_account.data, metadata));

        match wrapper.upsert(&self.pool).await {
            Ok(()) => {
                POSTGRES_ACCOUNTS_UPSERTED.inc();
                POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS
                    .get()
                    .unwrap()
                    .record(start.elapsed().as_millis() as f64);
                Ok(())
            }
            Err(e) => {
                POSTGRES_ACCOUNTS_UPSERT_FAILED.inc();
                return Err(e);
            }
        }
    }
}

pub struct PostgresJsonAccountProcessor<T> {
    pool: sqlx::PgPool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> PostgresJsonAccountProcessor<T> {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T> crate::processor::Processor for PostgresJsonAccountProcessor<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
{
    type InputType = AccountProcessorInputType<T>;

    async fn process(&mut self, input: Self::InputType) -> CarbonResult<()> {
        let (metadata, decoded_account, _raw) = input;

        let account_row = AccountRow::from_parts(decoded_account.data, metadata);

        let start = std::time::Instant::now();

        match account_row.upsert(&self.pool).await {
            Ok(()) => {
                POSTGRES_ACCOUNTS_UPSERTED.inc();
                POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS
                    .get()
                    .unwrap()
                    .record(start.elapsed().as_millis() as f64);
                Ok(())
            }
            Err(e) => {
                POSTGRES_ACCOUNTS_UPSERT_FAILED.inc();
                return Err(e);
            }
        }
    }
}

pub struct PostgresInstructionProcessor<T, W> {
    pool: sqlx::PgPool,
    _phantom: std::marker::PhantomData<(T, W)>,
}

impl<T, W> PostgresInstructionProcessor<T, W> {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T, W> crate::processor::Processor for PostgresInstructionProcessor<T, W>
where
    T: Clone + Send + Sync + 'static,
    W: From<(T, InstructionMetadata)> + Upsert + Send + 'static,
{
    type InputType = InstructionProcessorInputType<T>;

    async fn process(&mut self, input: Self::InputType) -> CarbonResult<()> {
        let (metadata, decoded_instruction, _nested_instructions, _raw) = input;

        let start = std::time::Instant::now();

        let wrapper = W::from((decoded_instruction.data, metadata));

        match wrapper.upsert(&self.pool).await {
            Ok(()) => {
                POSTGRES_INSTRUCTIONS_UPSERTED.inc();
                POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS
                    .get()
                    .unwrap()
                    .record(start.elapsed().as_millis() as f64);
                Ok(())
            }
            Err(e) => {
                POSTGRES_INSTRUCTIONS_UPSERT_FAILED.inc();
                return Err(e);
            }
        }
    }
}

pub struct PostgresJsonInstructionProcessor<T> {
    pool: sqlx::PgPool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> PostgresJsonInstructionProcessor<T> {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T> crate::processor::Processor for PostgresJsonInstructionProcessor<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
{
    type InputType = InstructionProcessorInputType<T>;

    async fn process(&mut self, input: Self::InputType) -> CarbonResult<()> {
        let (metadata, decoded_instruction, _nested_instructions, _raw) = input;

        let instruction_row = InstructionRow::from_parts(
            decoded_instruction.data,
            metadata,
            decoded_instruction.accounts,
        );

        let start = std::time::Instant::now();

        match instruction_row.upsert(&self.pool).await {
            Ok(()) => {
                POSTGRES_INSTRUCTIONS_UPSERTED.inc();
                POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS
                    .get()
                    .unwrap()
                    .record(start.elapsed().as_millis() as f64);
                Ok(())
            }
            Err(e) => {
                POSTGRES_INSTRUCTIONS_UPSERT_FAILED.inc();
                return Err(e);
            }
        }
    }
}
