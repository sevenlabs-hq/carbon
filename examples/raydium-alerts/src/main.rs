use {
    carbon_core::{
        account::AccountProcessorInputType, error::CarbonResult,
        instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_raydium_amm_v4_decoder::{
        accounts::RaydiumAmmV4Account, instructions::RaydiumAmmV4Instruction, RaydiumAmmV4Decoder,
        PROGRAM_ID as RAYDIUM_AMM_V4_PROGRAM_ID,
    },
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
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
        "raydium_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![RAYDIUM_AMM_V4_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![RAYDIUM_AMM_V4_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("raydium_transaction_filter".to_string(), transaction_filter);

    let geyser_config = YellowstoneGrpcClientConfig::new(
        None,
        Some(Duration::from_secs(15)),
        Some(Duration::from_secs(15)),
        None,
        None,
        None,
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

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(RaydiumAmmV4Decoder, RaydiumAmmV4InstructionProcessor)
        .account(RaydiumAmmV4Decoder, RaydiumAmmV4AccountProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct RaydiumAmmV4InstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, RaydiumAmmV4Instruction>>
    for RaydiumAmmV4InstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, RaydiumAmmV4Instruction>,
    ) -> CarbonResult<()> {
        log::info!(
            "Raydium AMM V4 instruction: {:?} on slot {}",
            input.decoded_instruction,
            input.metadata.transaction_metadata.slot
        );

        Ok(())
    }
}

pub struct RaydiumAmmV4AccountProcessor;

impl Processor<AccountProcessorInputType<'_, RaydiumAmmV4Account>>
    for RaydiumAmmV4AccountProcessor
{
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, RaydiumAmmV4Account>,
    ) -> CarbonResult<()> {
        log::info!(
            "Raydium AMM V4 account: {:?} on slot {}",
            input.decoded_account,
            input.metadata.slot
        );

        Ok(())
    }
}
