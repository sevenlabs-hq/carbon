use {
    async_trait::async_trait,
    carbon_core::{
        account::AccountProcessorInputType,
        datasource::{AccountUpdate, Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
        filter::DatasourceFilter,
        instruction::InstructionProcessorInputType,
        processor::Processor,
    },
    carbon_kamino_lending_decoder::{
        accounts::KaminoLendingAccount, instructions::KaminoLendingInstruction,
        KaminoLendingDecoder, PROGRAM_ID as KAMINO_LENDING_PROGRAM_ID,
    },
    carbon_log_metrics::LogMetrics,
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
    solana_account::Account,
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcProgramAccountsConfig},
    solana_pubkey::Pubkey,
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
        time::Duration,
    },
    tokio::sync::{mpsc::Sender, RwLock},
    tokio_util::sync::CancellationToken,
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    },
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // NOTE: Workaround, that solving issue https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Can't set crypto provider to aws_lc_rs");

    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "kamino_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![KAMINO_LENDING_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![KAMINO_LENDING_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("kamino_transaction_filter".to_string(), transaction_filter);

    let geyser_config = YellowstoneGrpcClientConfig::new(
        None,
        Some(Duration::from_secs(15)),
        Some(Duration::from_secs(15)),
        env::var("MAX_DECODING_MESSAGE_SIZE")
            .ok()
            .and_then(|v| v.parse::<usize>().ok()),
        None,
        env::var("TCP_NODE")
            .ok()
            .and_then(|v| v.parse::<bool>().ok()),
    );

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        geyser_config,
    );
    let yellowstone_grpc_id = DatasourceId::new_named("yellowstone_grpc");

    let gpa_backfill_datasource = GpaRpcDatasource::new(
        env::var("RPC_URL").unwrap_or_default(),
        KAMINO_LENDING_PROGRAM_ID,
        None,
    );
    let gpa_backfill_datasource_id = DatasourceId::new_named("gpa_backfill");

    carbon_core::pipeline::Pipeline::builder()
        .datasource_with_id(yellowstone_grpc, yellowstone_grpc_id.clone())
        .datasource_with_id(gpa_backfill_datasource, gpa_backfill_datasource_id.clone())
        .instruction(KaminoLendingDecoder, KaminoLendingInstructionProcessor)
        .account_with_filters(
            KaminoLendingDecoder,
            KaminoLendingRealtimeAccountProcessor,
            vec![Box::new(DatasourceFilter::new(yellowstone_grpc_id))],
        )
        .account_with_filters(
            KaminoLendingDecoder,
            KaminoLendingStartupAccountProcessor,
            vec![Box::new(DatasourceFilter::new(gpa_backfill_datasource_id))],
        )
        .channel_buffer_size(1_000_000)
        .metrics(Arc::new(LogMetrics::new()))
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct KaminoLendingInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, KaminoLendingInstruction>>
    for KaminoLendingInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, KaminoLendingInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;

        let signature = format!(
            "{}...{}",
            &signature.to_string()[..4],
            &signature.to_string()[signature.to_string().len() - 4..signature.to_string().len()]
        );

        log::info!("instruction processed ({signature}) {:?}", input.decoded_instruction);

        Ok(())
    }
}

pub struct KaminoLendingRealtimeAccountProcessor;
impl Processor<AccountProcessorInputType<'_, KaminoLendingAccount>>
    for KaminoLendingRealtimeAccountProcessor
{
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, KaminoLendingAccount>,
    ) -> CarbonResult<()> {
        log::info!(
            "account updated ({}) {:?}",
            input.metadata.pubkey,
            input.decoded_account.data
        );

        Ok(())
    }
}

pub struct KaminoLendingStartupAccountProcessor;
impl Processor<AccountProcessorInputType<'_, KaminoLendingAccount>>
    for KaminoLendingStartupAccountProcessor
{
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, KaminoLendingAccount>,
    ) -> CarbonResult<()> {
        log::info!(
            "gpa account received ({}) {:?}",
            input.metadata.pubkey,
            input.decoded_account.data
        );

        Ok(())
    }
}

pub struct GpaRpcDatasource {
    pub rpc_url: String,
    pub program_id: Pubkey,
    pub config: Option<RpcProgramAccountsConfig>,
}

impl GpaRpcDatasource {
    pub fn new(
        rpc_url: String,
        program_id: Pubkey,
        config: Option<RpcProgramAccountsConfig>,
    ) -> Self {
        Self {
            rpc_url,
            program_id,
            config,
        }
    }
}

#[async_trait]
impl Datasource for GpaRpcDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        _cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        let rpc_client = RpcClient::new(self.rpc_url.clone());

        let Ok(slot) = rpc_client.get_slot().await else {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Failed to fetch slot".to_string(),
            ));
        };

        let program_accounts = match &self.config {
            Some(config) => {
                rpc_client
                    .get_program_ui_accounts_with_config(&self.program_id, config.clone())
                    .await
            }
            None => {
                rpc_client
                    .get_program_ui_accounts_with_config(
                        &self.program_id,
                        RpcProgramAccountsConfig::default(),
                    )
                    .await
            }
        };

        let Ok(program_accounts) = program_accounts else {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Failed to fetch program accounts".to_string(),
            ));
        };

        let id_for_loop = id.clone();

        for (pubkey, account) in program_accounts {
            if let Some(account) = account.decode::<Account>() {
                if let Err(e) = sender.try_send((
                    Update::Account(AccountUpdate {
                        pubkey,
                        account,
                        slot,
                        transaction_signature: None,
                    }),
                    id_for_loop.clone(),
                )) {
                    log::error!("Failed to send account update: {e:?}");
                }
            }
        }

        Ok(())
    }

    fn update_types(&self) -> Vec<carbon_core::datasource::UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
