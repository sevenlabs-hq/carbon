use {
    carbon_core::{
        error::{CarbonResult, Error as CarbonError},
        postgres::{
            processors::PostgresJsonInstructionProcessor,
            rows::{GenericAccountsMigration, GenericInstructionMigration},
        },
    },
    carbon_jupiter_swap_decoder::{
        instructions::JupiterSwapInstruction, JupiterSwapDecoder,
        PROGRAM_ID as JUPITER_SWAP_PROGRAM_ID,
    },
    carbon_log_metrics::LogMetrics,
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    solana_client::rpc_config::RpcBlockSubscribeFilter,
    sqlx::Pool,
    sqlx_migrator::{Info, Migrate, Migrator, Plan},
    std::{env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Database connection and migrations
    let db_url = env::var("DATABASE_URL")
        .map_err(|err| CarbonError::Custom(format!("DATABASE_URL must be set ({err})")))?;

    let pool = Pool::<sqlx::Postgres>::connect(&db_url)
        .await
        .map_err(|err| CarbonError::Custom(format!("Failed to connect to Postgres: {err}")))?;

    let mut migrator = Migrator::default();
    migrator
        .add_migration(Box::new(GenericAccountsMigration))
        .map_err(|err| CarbonError::Custom(format!("Failed to add accounts migration: {err}")))?;
    migrator
        .add_migration(Box::new(GenericInstructionMigration))
        .map_err(|err| {
            CarbonError::Custom(format!("Failed to add instruction migration: {err}"))
        })?;

    let mut conn = pool.acquire().await.map_err(|err| {
        CarbonError::Custom(format!("Failed to acquire Postgres connection: {err}"))
    })?;
    migrator
        .run(&mut *conn, &Plan::apply_all())
        .await
        .map_err(|err| CarbonError::Custom(format!("Failed to run migrations: {err}")))?;

    // Datasource setup
    let rpc_ws_url = env::var("RPC_WS_URL")
        .map_err(|err| CarbonError::Custom(format!("RPC_WS_URL must be set ({err})")))?;
    let filters = Filters::new(
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(JUPITER_SWAP_PROGRAM_ID.to_string()),
        None,
    );
    let datasource = RpcBlockSubscribe::new(rpc_ws_url, filters);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(5)
        .instruction(
            JupiterSwapDecoder,
            PostgresJsonInstructionProcessor::<JupiterSwapInstruction>::new(pool.clone()),
        )
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}