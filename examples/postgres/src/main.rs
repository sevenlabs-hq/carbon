use {
    carbon_core::{
        error::CarbonResult,
        postgres::{
            processors::{
                PostgresAccountProcessor, PostgresInstructionProcessor,
            },
            rows::{GenericAccountsMigration, GenericInstructionMigration},
        },
    },
    carbon_log_metrics::LogMetrics,
    carbon_example_meteora_dlmm_decoder::{
        accounts::{
            postgres::{LbClmmAccountWithMetadata, LbClmmAccountsMigration},
            LbClmmAccount,
        },
        instructions::{
            postgres::{LbClmmInstructionWithMetadata, LbClmmInstructionsMigration},
            LbClmmInstruction,
        },
        LbClmmDecoder, PROGRAM_ID,
    },
    carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient,
    sqlx_migrator::{Info, Migrate, Plan},
    std::{
        collections::{HashMap, HashSet},
        env,
        net::SocketAddr,
        sync::Arc,
    },
    tokio::sync::RwLock,
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    },
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::Pool::<sqlx::Postgres>::connect(&db_url)
        .await
        .unwrap();
    let mut migrator = sqlx_migrator::Migrator::default();
    let mut conn = pool.acquire().await.unwrap();

    // Generic Migrations
    migrator
        .add_migration(Box::new(GenericAccountsMigration))
        .unwrap();
    migrator
        .add_migration(Box::new(GenericInstructionMigration))
        .unwrap();

    // Concrete Migrations
    migrator
        .add_migration(Box::new(LbClmmAccountsMigration))
        .unwrap();
    migrator
        .add_migration(Box::new(LbClmmInstructionsMigration))
        .unwrap();
    migrator.run(&mut *conn, &Plan::apply_all()).await.unwrap();

    // NOTE: Workaround, that solving issue https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Can't set crypto provider to aws_lc_rs");

    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "meteora_dlmm_transaction_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert(
        "meteora_dlmm_transaction_filter".to_string(),
        transaction_filter,
    );

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
    );

    let mut pipeline = carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        // Generic Processors
        // .account(
        //     PumpAmmDecoder,
        //     PostgresJsonAccountProcessor::<PumpAmmAccount>::new(pool.clone()),
        // )
        // .instruction(
        //     PumpAmmDecoder,
        //     PostgresJsonInstructionProcessor::<PumpAmmInstruction>::new(pool.clone()),
        // )
        // Concrete Processors
        .account(
            LbClmmDecoder,
            PostgresAccountProcessor::<LbClmmAccount, LbClmmAccountWithMetadata>::new(
                pool.clone(),
            ),
        )
        .instruction(
            LbClmmDecoder,
            PostgresInstructionProcessor::<LbClmmInstruction, LbClmmInstructionWithMetadata>::new(
                pool.clone(),
            ),
        )
        .build()?;

    tokio::select! {
        res = run_graphql(Arc::new(pool.clone())) => {
            res?;
        }
        res = pipeline.run() => {
            res?;
        }
    }

    Ok(())
}

async fn run_graphql(pool: Arc<sqlx::PgPool>) -> CarbonResult<()> {
    let schema =
        carbon_core::graphql::server::build_schema(carbon_example_meteora_dlmm_decoder::graphql::QueryRoot);
    let ctx = carbon_example_meteora_dlmm_decoder::graphql::context::GraphQLContext { pool: pool.clone() };
    let app = carbon_core::graphql::server::graphql_router::<
    carbon_example_meteora_dlmm_decoder::graphql::QueryRoot,
    carbon_example_meteora_dlmm_decoder::graphql::context::GraphQLContext,
    >(schema, ctx);

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("GraphQL: http://{addr}/graphql");
    println!("GraphiQL: http://{addr}/graphiql");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
    Ok(())
}
