use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_raydium_cpmm_decoder::{
        instructions::{CpiEvent, RaydiumCpmmInstruction},
        RaydiumCpmmDecoder, PROGRAM_ID as RAYDIUM_CPMM_PROGRAM_ID,
    },
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
    log,
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
        time::Duration,
    },
    tokio::sync::RwLock,
    yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterTransactions},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // NOTE: Workaround, that solving issue https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Can't set crypto provider to aws_lc_rs");

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![RAYDIUM_CPMM_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert(
        "jupiter_swap_transaction_filter".to_string(),
        transaction_filter,
    );

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
        HashMap::default(),
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        geyser_config,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(RaydiumCpmmDecoder, RaydiumCpmmInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct RaydiumCpmmInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, RaydiumCpmmInstruction>>
    for RaydiumCpmmInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, RaydiumCpmmInstruction>,
    ) -> CarbonResult<()> {
        match input.decoded_instruction {
            RaydiumCpmmInstruction::SwapBaseInput { data, accounts, .. } => {
                log::info!(
                    "SwapBaseInput: amount_in={}, minimum_amount_out={}, input_mint={}, output_mint={}, slot={}",
                    data.amount_in,
                    data.minimum_amount_out,
                    accounts.input_token_mint,
                    accounts.output_token_mint,
                    input.metadata.transaction_metadata.slot
                );
            }
            RaydiumCpmmInstruction::SwapBaseOutput { data, accounts, .. } => {
                log::info!(
                    "SwapBaseOutput: amount_out={}, max_amount_in={}, input_mint={}, output_mint={}, slot={}",
                    data.amount_out,
                    data.max_amount_in,
                    accounts.input_token_mint,
                    accounts.output_token_mint,
                    input.metadata.transaction_metadata.slot
                );
            }
            RaydiumCpmmInstruction::CpiEvent { data, .. } => {
                if let CpiEvent::SwapEvent(swap_event) = data {
                    log::info!(
                        "SwapEvent: {:?} on slot {}",
                        swap_event,
                        input.metadata.transaction_metadata.slot
                    );
                }
            }
            _ => {
                log::info!("Unknown instruction: {:?}", input.decoded_instruction);
            }
        }

        Ok(())
    }
}
