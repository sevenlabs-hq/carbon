use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_raydium_clmm_decoder::{
        instructions::RaydiumClmmInstruction, RaydiumClmmDecoder,
        PROGRAM_ID as RAYDIUM_CLMM_PROGRAM_ID,
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
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(RAYDIUM_CLMM_PROGRAM_ID.to_string()),
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
        .instruction(RaydiumClmmDecoder, RaydiumClmmInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct RaydiumClmmInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, RaydiumClmmInstruction>>
    for RaydiumClmmInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, RaydiumClmmInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;

        match input.decoded_instruction {
            RaydiumClmmInstruction::CreateAmmConfig { data, .. } => {
                log::info!("CreateAmmConfig: signature: {signature}, create_amm_cfg: {data:?}");
            }
            RaydiumClmmInstruction::UpdateAmmConfig { data, .. } => {
                log::info!("UpdateAmmConfig: signature: {signature}, update_amm_cfg: {data:?}");
            }
            RaydiumClmmInstruction::CreatePool { data, .. } => {
                log::info!("CreatePool: signature: {signature}, create_pool: {data:?}");
            }
            RaydiumClmmInstruction::CreateSupportMintAssociated { data, .. } => {
                log::info!(
                    "CreateSupportMintAssociated: signature: {signature}, create_support_mint_associated: {data:?}"
                );
            }
            RaydiumClmmInstruction::UpdatePoolStatus { data, .. } => {
                log::info!(
                    "UpdatePoolStatus: signature: {signature}, update_pool_status: {data:?}"
                );
            }
            RaydiumClmmInstruction::CreateOperationAccount { data, .. } => {
                log::info!(
                    "CreateOperationAccount: signature: {signature}, create_operation_acc: {data:?}"
                );
            }
            RaydiumClmmInstruction::UpdateOperationAccount { data, .. } => {
                log::info!(
                    "UpdateOperationAccount: signature: {signature}, update_operation_acc: {data:?}"
                );
            }
            RaydiumClmmInstruction::TransferRewardOwner { data, .. } => {
                log::info!(
                    "TransferRewardOwner: signature: {signature}, transfer_reward_owner: {data:?}"
                );
            }
            RaydiumClmmInstruction::InitializeReward { data, .. } => {
                log::info!("InitializeReward: signature: {signature}, init_reward: {data:?}");
            }
            RaydiumClmmInstruction::CollectRemainingRewards { data, .. } => {
                log::info!(
                    "CollectRemainingRewards: signature: {signature}, collect_remaining_rewards: {data:?}"
                );
            }
            RaydiumClmmInstruction::UpdateRewardInfos { data, .. } => {
                log::info!(
                    "UpdateRewardInfos: signature: {signature}, update_reward_infos: {data:?}"
                );
            }
            RaydiumClmmInstruction::SetRewardParams { data, .. } => {
                log::info!("SetRewardParams: signature: {signature}, set_reward_params: {data:?}");
            }
            RaydiumClmmInstruction::CollectProtocolFee { data, .. } => {
                log::info!(
                    "CollectProtocolFee: signature: {signature}, collect_protocol_fee: {data:?}"
                );
            }
            RaydiumClmmInstruction::CollectFundFee { data, .. } => {
                log::info!("CollectFundFee: signature: {signature}, collect_fund_fee: {data:?}");
            }
            RaydiumClmmInstruction::OpenPosition { data, .. } => {
                log::info!("OpenPosition: signature: {signature}, open_position: {data:?}");
            }
            RaydiumClmmInstruction::OpenPositionV2 { data, .. } => {
                log::info!("OpenPositionV2: signature: {signature}, open_position_v2: {data:?}");
            }
            RaydiumClmmInstruction::OpenPositionWithToken22Nft { data, .. } => {
                log::info!(
                    "OpenPositionWithToken22Nft: signature: {signature}, open_position_with_token22_nft: {data:?}"
                );
            }
            RaydiumClmmInstruction::ClosePosition { data, .. } => {
                log::info!("ClosePosition: signature: {signature}, close_position: {data:?}");
            }
            RaydiumClmmInstruction::IncreaseLiquidity { data, .. } => {
                log::info!("IncreaseLiquidity: signature: {signature}, increase_liq: {data:?}");
            }
            RaydiumClmmInstruction::IncreaseLiquidityV2 { data, .. } => {
                log::info!(
                    "IncreaseLiquidityV2: signature: {signature}, increase_liq_v2: {data:?}"
                );
            }
            RaydiumClmmInstruction::DecreaseLiquidity { data, .. } => {
                log::info!("DecreaseLiquidity: signature: {signature}, decrease_liq: {data:?}");
            }
            RaydiumClmmInstruction::DecreaseLiquidityV2 { data, .. } => {
                log::info!(
                    "DecreaseLiquidityV2: signature: {signature}, decrease_liq_v2: {data:?}"
                );
            }
            RaydiumClmmInstruction::Swap { data, .. } => {
                log::info!("Swap: signature: {signature}, swap: {data:?}");
            }
            RaydiumClmmInstruction::SwapV2 { data, .. } => {
                log::info!("SwapV2: signature: {signature}, swap_v2: {data:?}");
            }
            RaydiumClmmInstruction::SwapRouterBaseIn { data, .. } => {
                log::info!("SwapRouterBaseIn: signature: {signature}, swap_base_in: {data:?}");
            }
            RaydiumClmmInstruction::CpiEvent { data, .. } => {
                log::info!("CpiEvent: signature: {signature}, cpi_event: {data:?}");
            }
        }

        Ok(())
    }
}
