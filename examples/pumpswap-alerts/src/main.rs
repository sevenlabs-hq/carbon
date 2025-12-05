use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType,
        metrics::MetricsCollection, processor::Processor,
    },
    carbon_helius_laserstream_datasource::{LaserStreamClientConfig, LaserStreamGeyserClient},
    carbon_log_metrics::LogMetrics,
    carbon_pump_amm_decoder::{
        instructions::PumpAmmInstruction, PumpAmmDecoder, PROGRAM_ID as PUMPSWAP_PROGRAM_ID,
    },
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
        time::Duration,
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

    // NOTE: Workaround, that solving issue https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Can't set crypto provider to aws_lc_rs");

    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "pumpswap_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![PUMPSWAP_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![PUMPSWAP_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert(
        "pumpswap_transaction_filter".to_string(),
        transaction_filter,
    );

    let laserstream_config = LaserStreamClientConfig::new(
        None,                          // compression
        Some(Duration::from_secs(15)), // connect_timeout
        Some(Duration::from_secs(15)), // timeout
        None,                          // max_decoding_message_size
        None,                          // tls_config
        None,                          // tcp_nodelay
        true,                          // replay_enabled - This is the key LaserStream feature!
    );

    let laserstream_datasource = LaserStreamGeyserClient::new(
        env::var("LASERSTREAM_URL").unwrap_or_default(),
        env::var("API_KEY").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        laserstream_config,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(laserstream_datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .instruction(PumpAmmDecoder, PumpSwapInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpSwapInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, PumpAmmInstruction>>
    for PumpSwapInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, PumpAmmInstruction>,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;

        match input.instruction {
            PumpAmmInstruction::AdminSetCoinCreator {
                program_id,
                data,
                accounts,
            } => {
                log::info!("PumpSwap (Program ID: {program_id}) instruction: signature: {signature}, data: {data:?}, accounts: {accounts:?}");
            }
            PumpAmmInstruction::Buy {
                program_id,
                data,
                accounts,
            } => {
                log::info!("PumpSwap (Program ID: {program_id}) instruction: signature: {signature}, data: {data:?}, accounts: {accounts:?}");
            }
            _ => {}
        };

        Ok(())
    }
}
