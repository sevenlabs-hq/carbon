use std::sync::Arc;

use crate::{
    account::AccountMetadata,
    error::CarbonResult,
    instruction::{InstructionMetadata, InstructionProcessorInputType},
    metrics::MetricsCollection,
    postgres::{
        operations::Upsert,
        rows::{AccountRow, InstructionRow},
    },
};

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

impl<T, W> crate::processor::Processor<(AccountMetadata, T, solana_account::Account)>
    for PostgresAccountProcessor<T, W>
where
    T: Clone + Send + Sync + 'static,
    W: From<(T, AccountMetadata)> + Upsert + Send + 'static,
{
    async fn process(
        &mut self,
        input: &(AccountMetadata, T, solana_account::Account),
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded_account, _raw) = input;
        let metadata = metadata.clone();
        let decoded_account = decoded_account.clone();
        let pool = self.pool.clone();

        let start = std::time::Instant::now();

        let wrapper = W::from((decoded_account, metadata));

        match wrapper.upsert(&pool).await {
            Ok(()) => {
                metrics
                    .increment_counter("postgres.accounts.upsert.upserted", 1)
                    .await?;
                metrics
                    .record_histogram(
                        "postgres.accounts.upsert.duration_milliseconds",
                        start.elapsed().as_millis() as f64,
                    )
                    .await?;
                Ok(())
            }
            Err(e) => {
                metrics
                    .increment_counter("postgres.accounts.upsert.failed", 1)
                    .await?;
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

impl<T> crate::processor::Processor<(AccountMetadata, T, solana_account::Account)>
    for PostgresJsonAccountProcessor<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
{
    async fn process(
        &mut self,
        input: &(AccountMetadata, T, solana_account::Account),
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded_account, _raw) = input;

        let account_row = AccountRow::from_parts(decoded_account.clone(), metadata);

        let start = std::time::Instant::now();

        match account_row.upsert(&self.pool).await {
            Ok(()) => {
                metrics
                    .increment_counter("postgres.accounts.upsert.upserted", 1)
                    .await?;
                metrics
                    .record_histogram(
                        "postgres.accounts.upsert.duration_milliseconds",
                        start.elapsed().as_millis() as f64,
                    )
                    .await?;
                Ok(())
            }
            Err(e) => {
                metrics
                    .increment_counter("postgres.accounts.upsert.failed", 1)
                    .await?;
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
    W: From<(T, InstructionMetadata)> + Upsert + Send + 'static,
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, T>,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let metadata = input.metadata;
        let instruction = input.instruction;

        let start = std::time::Instant::now();

        let wrapper = W::from((instruction.clone(), metadata.clone()));

        match wrapper.upsert(&self.pool).await {
            Ok(()) => {
                metrics
                    .increment_counter("postgres.instructions.upsert.upserted", 1)
                    .await?;
                metrics
                    .record_histogram(
                        "postgres.instructions.upsert.duration_milliseconds",
                        start.elapsed().as_millis() as f64,
                    )
                    .await?;
                Ok(())
            }
            Err(e) => {
                metrics
                    .increment_counter("postgres.instructions.upsert.failed", 1)
                    .await?;
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
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, T>,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let metadata = input.metadata;
        let instruction = input.instruction;

        let instruction_row = InstructionRow::from_parts(instruction.clone(), metadata.clone());

        let start = std::time::Instant::now();

        match instruction_row.upsert(&self.pool).await {
            Ok(()) => {
                metrics
                    .increment_counter("postgres.instructions.upsert.upserted", 1)
                    .await?;
                metrics
                    .record_histogram(
                        "postgres.instructions.upsert.duration_milliseconds",
                        start.elapsed().as_millis() as f64,
                    )
                    .await?;
                Ok(())
            }
            Err(e) => {
                metrics
                    .increment_counter("postgres.instructions.upsert.failed", 1)
                    .await?;
                Err(e)
            }
        }
    }
}
