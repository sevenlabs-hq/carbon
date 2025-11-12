mod db;
mod models;
mod processor;

use {
    carbon_core::error::{CarbonResult, Error as CarbonError},
    carbon_jupiter_swap_decoder::{JupiterSwapDecoder, PROGRAM_ID as JUPITER_SWAP_PROGRAM_ID},
    carbon_log_metrics::LogMetrics,
    carbon_rpc_transaction_crawler_datasource::{ConnectionConfig, Filters, RpcTransactionCrawler},
    processor::JupiterSwapProcessor,
    simplelog::{
        ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, SharedLogger, TermLogger,
        TerminalMode, WriteLogger,
    },
    sqlx::Pool,
    sqlx_migrator::{Info, Migrate, Migrator, Plan},
    std::{
        env,
        fs::{self, File},
        path::PathBuf,
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    },
};

fn init_logging() -> CarbonResult<()> {
    let log_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("logs");
    fs::create_dir_all(&log_dir).map_err(|err| {
        CarbonError::Custom(format!("Failed to create log directory {log_dir:?}: {err}"))
    })?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| CarbonError::Custom(format!("Failed to read system time: {err}")))?
        .as_secs();
    let log_file_path = log_dir.join(format!("run-{timestamp}.log"));
    let log_file = File::create(&log_file_path).map_err(|err| {
        CarbonError::Custom(format!(
            "Failed to create log file {log_file_path:?}: {err}"
        ))
    })?;

    let config = ConfigBuilder::new().build();

    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();
    loggers.push(TermLogger::new(
        LevelFilter::Info,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ));
    loggers.push(WriteLogger::new(LevelFilter::Info, config, log_file));

    CombinedLogger::init(loggers)
        .map_err(|err| CarbonError::Custom(format!("Failed to initialize logger: {err}")))?;

    log::info!("File logging enabled at {}", log_file_path.display());
    Ok(())
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    init_logging()?;

    // Database connection and migrations
    let db_url = env::var("DATABASE_URL")
        .map_err(|err| CarbonError::Custom(format!("DATABASE_URL must be set ({err})")))?;

    let pool = Pool::<sqlx::Postgres>::connect(&db_url)
        .await
        .map_err(|err| CarbonError::Custom(format!("Failed to connect to Postgres: {err}")))?;

    let mut migrator = Migrator::default();
    migrator
        .add_migration(db::JupiterSwapMigration::boxed())
        .map_err(|err| CarbonError::Custom(format!("Failed to add Jupiter migration: {err}")))?;

    let mut conn = pool.acquire().await.map_err(|err| {
        CarbonError::Custom(format!("Failed to acquire Postgres connection: {err}"))
    })?;
    migrator
        .run(&mut *conn, &Plan::apply_all())
        .await
        .map_err(|err| CarbonError::Custom(format!("Failed to run migrations: {err}")))?;

    // Datasource setup
    let rpc_url = env::var("RPC_URL")
        .map_err(|err| CarbonError::Custom(format!("RPC_URL must be set ({err})")))?;
    let rate_limit = env::var("RATE_LIMIT")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(20);
    log::info!("Using RPC rate limit: {rate_limit} requests/sec");
    let filters = Filters::new(None, None, None);
    let connection_config = ConnectionConfig::default().with_rate_limit(rate_limit);
    let datasource = RpcTransactionCrawler::new(
        rpc_url,
        JUPITER_SWAP_PROGRAM_ID,
        connection_config,
        filters,
        None,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(5)
        .instruction(JupiterSwapDecoder, JupiterSwapProcessor::new(pool.clone()))
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}
