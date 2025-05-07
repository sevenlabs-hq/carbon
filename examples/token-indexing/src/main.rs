use {
    async_trait::async_trait,
    axum::{
        response::Html,
        routing::{get, on, MethodFilter},
        Extension, Router,
    },
    carbon_core::{
        account::{AccountMetadata, DecodedAccount},
        borsh::schema,
        error::CarbonResult,
        instruction::InstructionProcessorInputType,
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_gql_server::server,
    carbon_log_metrics::LogMetrics,
    carbon_postgres_client::PgClient,
    carbon_token_program_decoder::{
        accounts::TokenProgramAccount,
        api::{TokenProgramSchema, TokenQuery},
        instructions::TokenProgramInstruction,
        storage::{migrations::InitMigration, queries::TokenQueries},
        TokenProgramDecoder,
    },
    carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient,
    juniper::{EmptyMutation, EmptySubscription, RootNode},
    juniper_axum::{graphiql, graphql, playground},
    juniper_graphql_ws::Schema,
    spl_token::state::Mint,
    std::{
        collections::{HashMap, HashSet},
        env,
        net::SocketAddr,
        sync::Arc,
        time::Duration,
    },
    tokio::{net::TcpListener, sync::RwLock},
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    },
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    // NOTE: Workaround, that solving issue https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Can't set crypto provider to aws_lc_rs");

    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "spl_token_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![spl_token::id().to_string()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    // Connect to database
    let db_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_client = PgClient::new(&db_uri, 1, 10)
        .await
        .expect("Failed to create Postgres client");
    let schema =
        TokenProgramSchema::new(TokenQuery, EmptyMutation::new(), EmptySubscription::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    server::run(addr, pg_client, Arc::new(schema)).await;

    // Run Migrations
    pg_client
        .migrate(vec![Box::new(InitMigration)])
        .await
        .expect("Failed to migrate");

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![spl_token::id().to_string()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert(
        "spl_token_transaction_filter".to_string(),
        transaction_filter,
    );

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .instruction(
            TokenProgramDecoder,
            TokenProgramInstructionProcessor {
                pg_client: pg_client.clone(),
            },
        )
        .account(
            TokenProgramDecoder,
            TokenProgramAccountProcessor {
                pg_client: pg_client.clone(),
            },
        )
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct TokenProgramAccountProcessor {
    pub pg_client: PgClient,
}

#[async_trait]
impl Processor for TokenProgramAccountProcessor {
    type InputType = (AccountMetadata, DecodedAccount<TokenProgramAccount>);

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let account = data.1;

        match account.data {
            TokenProgramAccount::Account(account) => {
                log::info!("Token Account: {:#?}", account);
                if let Err(e) = self.pg_client.save_token(account).await {
                    log::error!("Failed to save token account: {}", e);
                };
            }
            TokenProgramAccount::Mint(mint) => {
                log::info!("Token Mint: {:#?}", mint);
            }
            TokenProgramAccount::Multisig(multisig) => {
                log::info!("Token Multisig: {:#?}", multisig);
            }
        };

        Ok(())
    }
}

pub struct TokenProgramInstructionProcessor {
    pub pg_client: PgClient,
}

#[async_trait]
impl Processor for TokenProgramInstructionProcessor {
    type InputType = InstructionProcessorInputType<TokenProgramInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let token_program_instruction: TokenProgramInstruction = data.1.data;
        let accounts = data.1.accounts;

        match token_program_instruction {
            TokenProgramInstruction::InitializeMint(initialize_mint) => {
                log::info!("Initialize Mint: {:#?}", initialize_mint);
                let mint = Mint {
                    mint_authority: Some(initialize_mint.mint_authority).into(),
                    supply: 0,
                    decimals: initialize_mint.decimals,
                    is_initialized: true,
                    freeze_authority: initialize_mint.freeze_authority.into(),
                };
                if let Err(e) = self.pg_client.save_mint(mint, &accounts[0].pubkey).await {
                    log::error!("Failed to save mint: {}", e);
                };
            }
            TokenProgramInstruction::InitializeMint2(initialize_mint2) => {
                log::info!("Initialize Mint 2: {:#?}", initialize_mint2);
                let mint = Mint {
                    mint_authority: Some(initialize_mint2.mint_authority).into(),
                    supply: 0,
                    decimals: initialize_mint2.decimals,
                    is_initialized: true,
                    freeze_authority: initialize_mint2.freeze_authority.into(),
                };

                if let Err(e) = self.pg_client.save_mint(mint, &accounts[0].pubkey).await {
                    log::error!("Failed to save mint: {}", e);
                };
            }
            _ => {}
        };

        Ok(())
    }
}
