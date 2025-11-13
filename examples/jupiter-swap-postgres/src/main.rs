mod db;
mod models;
mod processor;

use {
    carbon_core::error::{CarbonResult, Error as CarbonError},
    carbon_jupiter_swap_decoder::{JupiterSwapDecoder, PROGRAM_ID as JUPITER_SWAP_PROGRAM_ID},
    carbon_log_metrics::LogMetrics,
    carbon_rpc_block_crawler_datasource::{RpcBlockConfig, RpcBlockCrawler},
    carbon_rpc_transaction_crawler_datasource::{ConnectionConfig, Filters, RpcTransactionCrawler},
    processor::JupiterSwapProcessor,
    simplelog::{
        ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, SharedLogger, TermLogger,
        TerminalMode, WriteLogger,
    },
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_transaction_status::UiTransactionEncoding,
    sqlx::Pool,
    sqlx_migrator::{Info, Migrate, Migrator, Plan},
    std::{
        env::{self, VarError},
        fmt::Display,
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

const DATASOURCE_ENV_KEY: &str = "DATASOURCE";
const TRANSACTION_DATASOURCE_VALUE: &str = "rpc_transaction_crawler";
const BLOCK_DATASOURCE_VALUE: &str = "rpc_block_crawler";
const DEFAULT_RATE_LIMIT: u32 = 10;

enum DatasourceMode {
    TransactionCrawler,
    BlockCrawler,
}

enum DatasourceSelection {
    TransactionCrawler(RpcTransactionCrawler),
    BlockCrawler(RpcBlockCrawler),
}

async fn configure_datasource(rpc_url: String) -> CarbonResult<DatasourceSelection> {
    match datasource_mode_from_env()? {
        DatasourceMode::TransactionCrawler => {
            let rate_limit = read_optional_env_var::<u32>("RATE_LIMIT")?
                .filter(|value| *value > 0)
                .unwrap_or(DEFAULT_RATE_LIMIT);
            log::info!(
                "Using RpcTransactionCrawler datasource (rate limit: {rate_limit} requests/sec)"
            );
            let filters = Filters::new(None, None, None);
            let connection_config = ConnectionConfig::default().with_rate_limit(rate_limit);
            Ok(DatasourceSelection::TransactionCrawler(
                RpcTransactionCrawler::new(
                    rpc_url,
                    JUPITER_SWAP_PROGRAM_ID,
                    connection_config,
                    filters,
                    None,
                ),
            ))
        }
        DatasourceMode::BlockCrawler => {
            let rpc_client = RpcClient::new(rpc_url.clone());
            let latest_slot = rpc_client.get_slot().await.map_err(|err| {
                CarbonError::Custom(format!(
                    "Failed to fetch the most recent slot for RpcBlockCrawler: {err}"
                ))
            })?;
            let start_slot = latest_slot.saturating_sub(1);

            let block_config = RpcBlockConfig {
                encoding: Some(UiTransactionEncoding::Binary),
                max_supported_transaction_version: Some(0),
                ..RpcBlockConfig::default()
            };

            log::info!(
                "Using RpcBlockCrawler datasource (start_slot: {}, interval: 100ms, max_concurrent_requests: 10)",
                start_slot,
            );

            Ok(DatasourceSelection::BlockCrawler(RpcBlockCrawler::new(
                rpc_url,
                start_slot,
                None,
                None,
                block_config,
                None,
                None,
            )))
        }
    }
}

fn datasource_mode_from_env() -> CarbonResult<DatasourceMode> {
    let raw_value = match env::var(DATASOURCE_ENV_KEY) {
        Ok(value) => value,
        Err(VarError::NotPresent) => String::new(),
        Err(err) => {
            return Err(CarbonError::Custom(format!(
                "Failed to read {DATASOURCE_ENV_KEY}: {err}"
            )))
        }
    };

    let normalized = {
        let trimmed = raw_value.trim();
        if trimmed.is_empty() {
            TRANSACTION_DATASOURCE_VALUE.to_string()
        } else {
            trimmed.to_ascii_lowercase()
        }
    };

    match normalized.as_str() {
        TRANSACTION_DATASOURCE_VALUE
        | "transaction"
        | "transaction_crawler"
        | "rpc_transaction" => Ok(DatasourceMode::TransactionCrawler),
        BLOCK_DATASOURCE_VALUE | "block" | "rpc_block" | "block_crawler" => {
            Ok(DatasourceMode::BlockCrawler)
        }
        other => Err(CarbonError::Custom(format!(
            "Unsupported {DATASOURCE_ENV_KEY} value '{other}'. Expected '{TRANSACTION_DATASOURCE_VALUE}' or '{BLOCK_DATASOURCE_VALUE}'."
        ))),
    }
}

fn read_optional_env_var<T>(key: &str) -> CarbonResult<Option<T>>
where
    T: std::str::FromStr,
    T::Err: Display,
{
    match env::var(key) {
        Ok(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                Ok(None)
            } else {
                trimmed
                    .parse::<T>()
                    .map(Some)
                    .map_err(|err| CarbonError::Custom(format!("Failed to parse {key}: {err}")))
            }
        }
        Err(VarError::NotPresent) => Ok(None),
        Err(err) => Err(CarbonError::Custom(format!("Failed to read {key}: {err}"))),
    }
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

    let rpc_url = env::var("RPC_URL")
        .map_err(|err| CarbonError::Custom(format!("RPC_URL must be set ({err})")))?;
    let datasource = configure_datasource(rpc_url).await?;

    let pipeline_builder = match datasource {
        DatasourceSelection::TransactionCrawler(datasource) => {
            carbon_core::pipeline::Pipeline::builder().datasource(datasource)
        }
        DatasourceSelection::BlockCrawler(datasource) => {
            carbon_core::pipeline::Pipeline::builder().datasource(datasource)
        }
    };

    pipeline_builder
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(5)
        .instruction(JupiterSwapDecoder, JupiterSwapProcessor::new(pool.clone()))
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}
