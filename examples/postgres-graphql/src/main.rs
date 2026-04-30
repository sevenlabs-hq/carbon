use {
    carbon_core::{
        error::{CarbonResult, Error},
        graphql::server::{build_schema, graphql_router},
        pipeline::Pipeline,
        postgres::processors::PostgresJsonAccountProcessor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_token_program_decoder::{
        accounts::TokenProgramAccount, TokenProgramDecoder, PROGRAM_ID as TOKEN_PROGRAM_ID,
    },
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
    juniper::{graphql_object, FieldError, FieldResult, GraphQLObject, Value},
    sqlx::{postgres::PgPoolOptions, PgPool, Row},
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
    },
    tokio::sync::RwLock,
    yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterAccounts},
};

#[derive(Clone, GraphQLObject, Debug)]
pub struct MintInfo {
    pub pubkey: String,
    pub slot: i32,
    pub decimals: i32,
    pub supply: String,
}

#[derive(Clone)]
pub struct AppContext {
    pool: PgPool,
}
impl juniper::Context for AppContext {}

pub struct Query;

#[graphql_object(context = AppContext)]
impl Query {
    async fn mints(context: &AppContext, limit: Option<i32>) -> FieldResult<Vec<MintInfo>> {
        let limit = limit.unwrap_or(20).max(0) as i64;
        let rows = sqlx::query(
            "SELECT __pubkey, __slot, \
                    data->'data'->>'decimals' AS decimals, \
                    data->'data'->>'supply' AS supply \
             FROM accounts \
             WHERE data->>'type' = 'Mint' \
             ORDER BY __slot DESC \
             LIMIT $1",
        )
        .bind(limit)
        .fetch_all(&context.pool)
        .await
        .map_err(|e| {
            log::error!("failed to fetch mints from Postgres: {e}");
            FieldError::new("failed to fetch mints", Value::null())
        })?;

        Ok(rows.into_iter().map(row_to_mint).collect())
    }

    async fn mint(context: &AppContext, pubkey: String) -> FieldResult<Option<MintInfo>> {
        let bytes = bs58::decode(&pubkey)
            .into_vec()
            .map_err(|_| FieldError::new("invalid mint pubkey", Value::null()))?;
        let row = sqlx::query(
            "SELECT __pubkey, __slot, \
                    data->'data'->>'decimals' AS decimals, \
                    data->'data'->>'supply' AS supply \
             FROM accounts \
             WHERE __pubkey = $1 AND data->>'type' = 'Mint'",
        )
        .bind(bytes)
        .fetch_optional(&context.pool)
        .await
        .map_err(|e| {
            log::error!("failed to fetch mint from Postgres: {e}");
            FieldError::new("failed to fetch mint", Value::null())
        })?;

        Ok(row.map(row_to_mint))
    }
}

fn row_to_mint(row: sqlx::postgres::PgRow) -> MintInfo {
    let pubkey_bytes: Vec<u8> = row.get("__pubkey");
    MintInfo {
        pubkey: bs58::encode(pubkey_bytes).into_string(),
        slot: row.get::<i64, _>("__slot") as i32,
        decimals: row
            .get::<Option<String>, _>("decimals")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        supply: row.get::<Option<String>, _>("supply").unwrap_or_default(),
    }
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .map_err(|e| Error::Custom(format!("failed to install rustls default provider: {e:?}")))?;

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| Error::Custom("DATABASE_URL must be set".to_string()))?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .map_err(|e| Error::Custom(format!("failed to connect to Postgres: {e}")))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| Error::Custom(format!("failed to run migrations: {e}")))?;

    let context = AppContext { pool: pool.clone() };
    let schema = build_schema(Query);
    let router = graphql_router(schema, context);
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .map_err(|e| Error::Custom(format!("failed to bind GraphQL server: {e}")))?;
    log::info!("graphiql at http://{bind_addr}/graphiql");
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            log::error!("GraphQL server exited with error: {e}");
        }
    });

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

    let geyser_url =
        env::var("GEYSER_URL").map_err(|_| Error::Custom("GEYSER_URL must be set".to_string()))?;
    let datasource = YellowstoneGrpcGeyserClient::new(
        geyser_url,
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
