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
use solana_instruction::AccountMeta;
use std::sync::LazyLock;

static POSTGRES_ACCOUNTS_UPSERTED: Counter = Counter::new(
    "postgres.accounts.upsert.upserted",
    "Total number of account upserts successfully completed",
);

static POSTGRES_ACCOUNTS_UPSERT_FAILED: Counter = Counter::new(
    "postgres.accounts.upsert.failed",
    "Total number of account upserts that failed",
);

static POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "postgres.accounts.upsert.duration_milliseconds",
        "Duration of account upsert operations in milliseconds",
        vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0],
    )
});

static POSTGRES_INSTRUCTIONS_UPSERTED: Counter = Counter::new(
    "postgres.instructions.upsert.upserted",
    "Total number of instruction upserts successfully completed",
);

static POSTGRES_INSTRUCTIONS_UPSERT_FAILED: Counter = Counter::new(
    "postgres.instructions.upsert.failed",
    "Total number of instruction upserts that failed",
);

static POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "postgres.instructions.upsert.duration_milliseconds",
        "Duration of instruction upsert operations in milliseconds",
        vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0],
    )
});

pub fn register_postgres_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&POSTGRES_ACCOUNTS_UPSERTED);
    registry.register_counter(&POSTGRES_ACCOUNTS_UPSERT_FAILED);
    registry.register_histogram(&POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS);
    registry.register_counter(&POSTGRES_INSTRUCTIONS_UPSERTED);
    registry.register_counter(&POSTGRES_INSTRUCTIONS_UPSERT_FAILED);
    registry.register_histogram(&POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS);
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

impl<T, W> crate::processor::Processor<AccountProcessorInputType<'_, T>>
    for PostgresAccountProcessor<T, W>
where
    T: Clone + Send + Sync + 'static,
    W: From<(T, AccountMetadata)> + Upsert + Send + 'static,
{
    async fn process(&mut self, input: &AccountProcessorInputType<'_, T>) -> CarbonResult<()> {
        let start = std::time::Instant::now();

        let wrapper = W::from((input.decoded_account.data.clone(), input.metadata.clone()));

        match wrapper.upsert(&self.pool).await {
            Ok(()) => {
                POSTGRES_ACCOUNTS_UPSERTED.inc();
                POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS.record(start.elapsed().as_millis() as f64);
                Ok(())
            }
            Err(e) => {
                POSTGRES_ACCOUNTS_UPSERT_FAILED.inc();
                Err(e)
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

impl<T> crate::processor::Processor<AccountProcessorInputType<'_, T>>
    for PostgresJsonAccountProcessor<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
{
    async fn process(&mut self, input: &AccountProcessorInputType<'_, T>) -> CarbonResult<()> {
        let account_row =
            AccountRow::from_parts(input.decoded_account.data.clone(), input.metadata.clone());

        let start = std::time::Instant::now();

        match account_row.upsert(&self.pool).await {
            Ok(()) => {
                POSTGRES_ACCOUNTS_UPSERTED.inc();
                POSTGRES_ACCOUNTS_UPSERT_DURATION_MILLIS.record(start.elapsed().as_millis() as f64);
                Ok(())
            }
            Err(e) => {
                POSTGRES_ACCOUNTS_UPSERT_FAILED.inc();
                Err(e)
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

impl<T, W> crate::processor::Processor<InstructionProcessorInputType<'_, T>>
    for PostgresInstructionProcessor<T, W>
where
    T: Clone + Send + Sync + 'static,
    W: From<(T, InstructionMetadata, Vec<AccountMeta>)> + Upsert + Send + 'static,
{
    async fn process(&mut self, input: &InstructionProcessorInputType<'_, T>) -> CarbonResult<()> {
        let start = std::time::Instant::now();

        let wrapper = W::from((
            input.decoded_instruction.clone(),
            input.metadata.clone(),
            input.raw_instruction.accounts.clone(),
        ));

        match wrapper.upsert(&self.pool).await {
            Ok(()) => {
                POSTGRES_INSTRUCTIONS_UPSERTED.inc();
                POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS
                    .record(start.elapsed().as_millis() as f64);
                Ok(())
            }
            Err(e) => {
                POSTGRES_INSTRUCTIONS_UPSERT_FAILED.inc();
                Err(e)
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

impl<T> crate::processor::Processor<InstructionProcessorInputType<'_, T>>
    for PostgresJsonInstructionProcessor<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
{
    async fn process(&mut self, input: &InstructionProcessorInputType<'_, T>) -> CarbonResult<()> {
        let instruction_row =
            InstructionRow::from_parts(input.decoded_instruction.clone(), input.metadata.clone());

        let start = std::time::Instant::now();

        match instruction_row.upsert(&self.pool).await {
            Ok(()) => {
                POSTGRES_INSTRUCTIONS_UPSERTED.inc();
                POSTGRES_INSTRUCTIONS_UPSERT_DURATION_MILLIS
                    .record(start.elapsed().as_millis() as f64);
                Ok(())
            }
            Err(e) => {
                POSTGRES_INSTRUCTIONS_UPSERT_FAILED.inc();
                Err(e)
            }
        }
    }
}
