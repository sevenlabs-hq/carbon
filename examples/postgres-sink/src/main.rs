use {
    carbon_core::{pipeline::Pipeline, postgres::processors::PostgresJsonAccountProcessor},
    carbon_log_metrics::LogMetrics,
    carbon_token_program_decoder::{
        accounts::TokenProgramAccount, TokenProgramDecoder, PROGRAM_ID as TOKEN_PROGRAM_ID,
    },
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
    sqlx::postgres::PgPoolOptions,
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
    },
    tokio::sync::RwLock,
    yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterAccounts},
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    // https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls default provider");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let mut account_filters = HashMap::new();
    account_filters.insert(
        "spl_token".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![TOKEN_PROGRAM_ID.to_string()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let datasource = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").expect("GEYSER_URL must be set"),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        HashMap::default(),
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        YellowstoneGrpcClientConfig::default(),
        None,
        None,
    );

    Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .account(
            TokenProgramDecoder,
            PostgresJsonAccountProcessor::<TokenProgramAccount>::new(pool),
        )
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}
