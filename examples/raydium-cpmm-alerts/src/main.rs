use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
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
        .metrics_flush_interval(3)
        .instruction(RaydiumCpmmDecoder, RaydiumCpmmInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct RaydiumCpmmInstructionProcessor;

#[async_trait]
impl Processor for RaydiumCpmmInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<RaydiumCpmmInstruction>,
        NestedInstructions,
        solana_instruction::Instruction,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions, _): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;

        match instruction.data {
            RaydiumCpmmInstruction::CreateAmmConfig(create_amm_config) => {
                log::info!("CreateAmmConfig: signature: {signature}, create_amm_config: {create_amm_config:?}");
            }
            RaydiumCpmmInstruction::UpdateAmmConfig(update_amm_config) => {
                log::info!("UpdateAmmConfig: signature: {signature}, update_amm_config: {update_amm_config:?}");
            }
            RaydiumCpmmInstruction::UpdatePoolStatus(update_pool_status) => {
                log::info!("UpdatePoolStatus: signature: {signature}, update_pool_status: {update_pool_status:?}");
            }
            RaydiumCpmmInstruction::CollectProtocolFee(collect_protocol_fee) => {
                log::info!("CollectProtocolFee: signature: {signature}, collect_protocol_fee: {collect_protocol_fee:?}");
            }
            RaydiumCpmmInstruction::CollectFundFee(collect_fund_fee) => {
                log::info!("CollectFundFee: signature: {signature}, collect_fund_fee: {collect_fund_fee:?}");
            }
            RaydiumCpmmInstruction::Initialize(initialize) => {
                log::info!("Initialize: signature: {signature}, initialize: {initialize:?}");
            }
            RaydiumCpmmInstruction::Deposit(deposit) => {
                log::info!("Deposit: signature: {signature}, deposit: {deposit:?}");
            }
            RaydiumCpmmInstruction::Withdraw(withdraw) => {
                log::info!("Withdraw: signature: {signature}, withdraw: {withdraw:?}");
            }
            RaydiumCpmmInstruction::SwapBaseInput(swap_base_input) => {
                log::info!(
                    "SwapBaseInput: signature: {signature}, swap_base_input: {swap_base_input:?}"
                );
            }
            RaydiumCpmmInstruction::SwapBaseOutput(swap_base_output) => {
                log::info!("SwapBaseOutput: signature: {signature}, swap_base_output: {swap_base_output:?}");
            }
            RaydiumCpmmInstruction::LpChangeEvent(lp_change_event) => {
                log::info!(
                    "LpChangeEvent: signature: {signature}, lp_change_event: {lp_change_event:?}"
                );
            }
            RaydiumCpmmInstruction::SwapEvent(swap_event) => {
                log::info!("SwapEvent: signature: {signature}, swap_event: {swap_event:?}");
            }
            RaydiumCpmmInstruction::ClosePermissionPda(close_permission_pda) => {
                log::info!("ClosePermissionPda: signature: {signature}, close_permission_pda: {close_permission_pda:?}");
            }
            RaydiumCpmmInstruction::CollectCreatorFee(collect_creator_fee) => {
                log::info!("CollectCreatorFee: signature: {signature}, collect_creator_fee: {collect_creator_fee:?}");
            }
            RaydiumCpmmInstruction::CreatePermissionPda(create_permission_pda) => {
                log::info!("CreatePermissionPda: signature: {signature}, create_permission_pda: {create_permission_pda:?}");
            }
            RaydiumCpmmInstruction::InitializeWithPermission(initialize_with_permission) => {
                log::info!("InitializeWithPermission: signature: {signature}, initialize_with_permission: {initialize_with_permission:?}");
            }
        };

        Ok(())
    }
}
