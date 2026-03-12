use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_raydium_cpmm_decoder::{
        instructions::RaydiumCpmmInstruction, RaydiumCpmmDecoder,
        PROGRAM_ID as RAYDIUM_CPMM_PROGRAM_ID,
    },
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    std::{env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let filters = Filters::new(
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(RAYDIUM_CPMM_PROGRAM_ID.to_string()),
        Some(RpcBlockSubscribeConfig {
            max_supported_transaction_version: Some(0),
            ..RpcBlockSubscribeConfig::default()
        }),
    );

    let rpc_ws_url =
        env::var("RPC_WS_URL").unwrap_or("wss://api.mainnet-beta.solana.com/".to_string());

    log::info!("Starting with RPC: {rpc_ws_url}");
    let block_subscribe = RpcBlockSubscribe::new(rpc_ws_url, filters);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(block_subscribe)
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
        let signature = input.metadata.transaction_metadata.signature;

        match input.decoded_instruction {
            RaydiumCpmmInstruction::CollectFundFee { data, .. } => {
                log::info!("CollectFundFee: signature: {signature}, collect_fund_fee: {data:?}");
            }
            RaydiumCpmmInstruction::CollectProtocolFee { data, .. } => {
                log::info!(
                    "CollectProtocolFee: signature: {signature}, collect_protocol_fee: {data:?}"
                );
            }
            RaydiumCpmmInstruction::CreateAmmConfig { data, .. } => {
                log::info!("CreateAmmConfig: signature: {signature}, create_amm_config: {data:?}");
            }
            RaydiumCpmmInstruction::Deposit { data, .. } => {
                log::info!("Deposit: signature: {signature}, deposit: {data:?}");
            }
            RaydiumCpmmInstruction::Initialize { data, .. } => {
                log::info!("Initialize: signature: {signature}, initialize: {data:?}");
            }
            RaydiumCpmmInstruction::SwapBaseInput { data, .. } => {
                log::info!("SwapBaseInput: signature: {signature}, swap_base_input: {data:?}");
            }
            RaydiumCpmmInstruction::SwapBaseOutput { data, .. } => {
                log::info!("SwapBaseOutput: signature: {signature}, swap_base_output: {data:?}");
            }
            RaydiumCpmmInstruction::UpdateAmmConfig { data, .. } => {
                log::info!("UpdateAmmConfig: signature: {signature}, update_amm_config: {data:?}");
            }
            RaydiumCpmmInstruction::UpdatePoolStatus { data, .. } => {
                log::info!(
                    "UpdatePoolStatus: signature: {signature}, update_pool_status: {data:?}"
                );
            }
            RaydiumCpmmInstruction::Withdraw { data, .. } => {
                log::info!("Withdraw: signature: {signature}, withdraw: {data:?}");
            }
            RaydiumCpmmInstruction::CpiEvent { data, .. } => {
                log::info!("CpiEvent: signature: {signature}, cpi_event: {data:?}");
            }
        }

        Ok(())
    }
}
