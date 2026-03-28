use std::env;

use solana_transaction_status::UiTransactionEncoding;

use {
    carbon_core::{
        error::{CarbonResult, Error as CarbonError},
        postgres::processors::PostgresInstructionProcessor,
    },
    carbon_jupiter_swap_decoder::{
        instructions::{
            postgres::{JupiterSwapInstructionWithMetadata, JupiterSwapInstructionsMigration},
            JupiterSwapInstruction,
        },
        JupiterSwapDecoder,
    },
    carbon_rpc_block_crawler_datasource::{RpcBlockConfig, RpcBlockCrawler},
    clap::Parser,
    sqlx::postgres::PgPoolOptions,
    sqlx_migrator::{Info, Migrate, Migrator, Plan},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    start_slot: Option<u64>,

    #[arg(short, long)]
    end_slot: Option<u64>,
}

fn env_usize(name: &str, default: usize) -> usize {
    match env::var(name) {
        Ok(value) => match value.parse::<usize>() {
            Ok(parsed) => parsed,
            Err(err) => {
                log::warn!("Invalid {name}={value:?}: {err}. Using default {default}.");
                default
            }
        },
        Err(_) => default,
    }
}

fn env_u64(name: &str) -> CarbonResult<u64> {
    let value =
        env::var(name).map_err(|err| CarbonError::Custom(format!("{name} must be set ({err})")))?;

    value
        .parse::<u64>()
        .map_err(|err| CarbonError::Custom(format!("Invalid {name}={value:?}: {err}")))
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    let mut logger = env_logger::Builder::new();
    logger.filter_level(log::LevelFilter::Info);
    if let Ok(log_level) = env::var("LOG_LEVEL") {
        logger.parse_filters(&log_level);
    }
    logger.init();

    let args = Args::parse();
    let start_slot = match args.start_slot {
        Some(start_slot) => start_slot,
        None => env_u64("BLOCK_CRAWLER_START_SLOT")?,
    };
    let end_slot = match args.end_slot {
        Some(end_slot) => end_slot,
        None => env_u64("BLOCK_CRAWLER_END_SLOT")?,
    };
    let database_url = env::var("DATABASE_URL")
        .map_err(|err| CarbonError::Custom(format!("DATABASE_URL must be set ({err})")))?;
    let max_concurrent_requests = env_usize("BLOCK_CRAWLER_MAX_CONCURRENT_REQUESTS", 1);
    let channel_buffer_size = env_usize("BLOCK_CRAWLER_CHANNEL_BUFFER_SIZE", 10);
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .map_err(|err| CarbonError::Custom(format!("Failed to connect to Postgres: {err}")))?;

    let mut migrator = Migrator::default();
    migrator
        .add_migration(Box::new(JupiterSwapInstructionsMigration))
        .map_err(|err| {
            CarbonError::Custom(format!(
                "Failed to add Jupiter swap instructions migration: {err}"
            ))
        })?;

    let mut conn = pool.acquire().await.map_err(|err| {
        CarbonError::Custom(format!("Failed to acquire Postgres connection: {err}"))
    })?;
    migrator
        .run(&mut *conn, &Plan::apply_all())
        .await
        .map_err(|err| CarbonError::Custom(format!("Failed to run migrations: {err}")))?;

    let rpc_block_ds = RpcBlockCrawler::new(
        env::var("RPC_URL").unwrap_or_default(),
        start_slot,
        Some(end_slot),
        None,
        RpcBlockConfig {
            encoding: Some(UiTransactionEncoding::Binary),
            max_supported_transaction_version: Some(0),
            ..Default::default()
        },
        Some(max_concurrent_requests),
        Some(channel_buffer_size),
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(rpc_block_ds)
        .instruction(
            JupiterSwapDecoder,
            PostgresInstructionProcessor::<
                JupiterSwapInstruction,
                JupiterSwapInstructionWithMetadata,
            >::new(pool),
        )
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}
