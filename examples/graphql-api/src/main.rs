use {
    carbon_core::{
        account::AccountProcessorInputType,
        error::CarbonResult,
        graphql::server::{build_schema, graphql_router},
        pipeline::Pipeline,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_token_2022_decoder::{
        accounts::Token2022Account, Token2022Decoder, PROGRAM_ID as TOKEN_2022_PROGRAM_ID,
    },
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
    juniper::{graphql_object, GraphQLObject},
    solana_pubkey::Pubkey,
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

#[derive(Clone, Default)]
pub struct AppContext {
    mints: Arc<RwLock<HashMap<Pubkey, MintInfo>>>,
}
impl juniper::Context for AppContext {}

pub struct Query;

#[graphql_object(context = AppContext)]
impl Query {
    async fn mints(context: &AppContext, limit: Option<i32>) -> Vec<MintInfo> {
        let limit = limit.unwrap_or(20).max(0) as usize;
        let mut all: Vec<_> = context.mints.read().await.values().cloned().collect();
        all.sort_by(|a, b| b.slot.cmp(&a.slot));
        all.into_iter().take(limit).collect()
    }

    async fn mint(context: &AppContext, pubkey: String) -> Option<MintInfo> {
        let key = pubkey.parse::<Pubkey>().ok()?;
        context.mints.read().await.get(&key).cloned()
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    // https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls default provider");

    let context = AppContext::default();
    let store_for_pipeline = context.mints.clone();

    let schema = build_schema(Query);
    let router = graphql_router(schema, context.clone());
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    log::info!("graphiql at http://{bind_addr}/graphiql");
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    let mut account_filters = HashMap::new();
    account_filters.insert(
        "token_2022".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![TOKEN_2022_PROGRAM_ID.to_string()],
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
            Token2022Decoder,
            MintCollector {
                store: store_for_pipeline,
            },
        )
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct MintCollector {
    store: Arc<RwLock<HashMap<Pubkey, MintInfo>>>,
}

impl Processor<AccountProcessorInputType<'_, Token2022Account>> for MintCollector {
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, Token2022Account>,
    ) -> CarbonResult<()> {
        if let Token2022Account::Mint(mint) = &input.decoded_account.data {
            let info = MintInfo {
                pubkey: input.metadata.pubkey.to_string(),
                slot: input.metadata.slot as i32,
                decimals: mint.decimals as i32,
                supply: mint.supply.to_string(),
            };
            self.store.write().await.insert(input.metadata.pubkey, info);
        }
        Ok(())
    }
}
