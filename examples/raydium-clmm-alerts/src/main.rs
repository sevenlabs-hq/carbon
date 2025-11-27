use {
    async_trait::async_trait,
    carbon_core::{
        deserialize::ArrangeAccounts,
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_raydium_clmm_decoder::{
        instructions::{swap_v2::SwapV2, RaydiumClmmInstruction},
        RaydiumClmmDecoder, PROGRAM_ID as RAYDIUM_CLMM_PROGRAM_ID,
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
        .metrics_flush_interval(3)
        .instruction(RaydiumClmmDecoder, RaydiumClmmInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct RaydiumClmmInstructionProcessor;

#[async_trait]
impl Processor for RaydiumClmmInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<RaydiumClmmInstruction>,
        NestedInstructions,
        solana_instruction::Instruction,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions, _): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let accounts = instruction.accounts;

        match instruction.data {
            RaydiumClmmInstruction::CreateAmmConfig(create_amm_cfg) => {
                log::info!(
                    "CreateAmmConfig: signature: {signature}, create_amm_cfg: {create_amm_cfg:?}"
                );
            }
            RaydiumClmmInstruction::UpdateAmmConfig(update_amm_cfg) => {
                log::info!(
                    "UpdateAmmConfig: signature: {signature}, update_amm_cfg: {update_amm_cfg:?}"
                );
            }
            RaydiumClmmInstruction::CreatePool(create_pool) => {
                log::info!("CreatePool: signature: {signature}, create_pool: {create_pool:?}");
            }
            RaydiumClmmInstruction::UpdatePoolStatus(update_pool_status) => {
                log::info!("UpdatePoolStatus: signature: {signature}, update_pool_status: {update_pool_status:?}");
            }
            RaydiumClmmInstruction::CreateOperationAccount(create_opperation_acc) => {
                log::info!("CreateOperationAccount: signature: {signature}, create_opperation_acc: {create_opperation_acc:?}");
            }
            RaydiumClmmInstruction::UpdateOperationAccount(update_opperation_acc) => {
                log::info!("UpdateOperationAccount: signature: {signature}, update_opperation_acc: {update_opperation_acc:?}");
            }
            RaydiumClmmInstruction::TransferRewardOwner(transfer_reward_owner) => {
                log::info!("TransferRewardOwner: signature: {signature}, transfer_reward_owner: {transfer_reward_owner:?}");
            }
            RaydiumClmmInstruction::InitializeReward(init_reward) => {
                log::info!(
                    "InitializeReward: signature: {signature}, init_reward: {init_reward:?}"
                );
            }
            RaydiumClmmInstruction::CollectRemainingRewards(collect_remaining_rewards) => {
                log::info!("CollectRemainingRewards: signature: {signature}, collect_remaining_rewards: {collect_remaining_rewards:?}");
            }
            RaydiumClmmInstruction::UpdateRewardInfos(update_reward_infos) => {
                log::info!("UpdateRewardInfos: signature: {signature}, update_reward_infos: {update_reward_infos:?}");
            }
            RaydiumClmmInstruction::SetRewardParams(set_reward_params) => {
                log::info!("SetRewardParams: signature: {signature}, set_reward_params: {set_reward_params:?}");
            }
            RaydiumClmmInstruction::CollectProtocolFee(collect_protocol_fee) => {
                log::info!("CollectProtocolFee: signature: {signature}, collect_protocol_fee: {collect_protocol_fee:?}");
            }
            RaydiumClmmInstruction::CollectFundFee(collect_fund_fee) => {
                log::info!("CollectFundFee: signature: {signature}, collect_fund_fee: {collect_fund_fee:?}");
            }
            RaydiumClmmInstruction::OpenPosition(open_position) => {
                log::info!(
                    "OpenPosition: signature: {signature}, open_position: {open_position:?}"
                );
            }
            RaydiumClmmInstruction::OpenPositionV2(open_position_v2) => {
                log::info!("OpenPositionV2: signature: {signature}, open_position_v2: {open_position_v2:?}");
            }
            RaydiumClmmInstruction::ClosePosition(close_position) => {
                log::info!(
                    "ClosePosition: signature: {signature}, close_position: {close_position:?}"
                );
            }
            RaydiumClmmInstruction::IncreaseLiquidity(increase_liq) => {
                log::info!(
                    "IncreaseLiquidity: signature: {signature}, increase_liq: {increase_liq:?}"
                );
            }
            RaydiumClmmInstruction::IncreaseLiquidityV2(increase_liq_v2) => {
                log::info!("IncreaseLiquidityV2: signature: {signature}, increase_liq_v2: {increase_liq_v2:?}");
            }
            RaydiumClmmInstruction::DecreaseLiquidity(decrease_liq) => {
                log::info!(
                    "DecreaseLiquidity: signature: {signature}, decrease_liq: {decrease_liq:?}"
                );
            }
            RaydiumClmmInstruction::DecreaseLiquidityV2(decrease_liq_v2) => {
                log::info!("DecreaseLiquidityV2: signature: {signature}, decrease_liq_v2: {decrease_liq_v2:?}");
            }
            RaydiumClmmInstruction::Swap(swap) => {
                log::info!("Swap: signature: {signature}, swap: {swap:?}");
            }
            RaydiumClmmInstruction::SwapV2(swap_v2) => match SwapV2::arrange_accounts(&accounts) {
                Some(accounts) => {
                    log::info!(
                            "SwapV2: signature: {signature}, swap_v2: {swap_v2:?}, accounts: {accounts:?}",
                        );
                }
                None => log::error!("Failed to arrange accounts for SwapV2 {}", accounts.len()),
            },
            RaydiumClmmInstruction::SwapRouterBaseIn(swap_base_in) => {
                log::info!(
                    "SwapRouterBaseIn: signature: {signature}, swap_base_in: {swap_base_in:?}"
                );
            }
            RaydiumClmmInstruction::ConfigChangeEvent(cfg_change_event) => {
                log::info!("ConfigChangeEvent: signature: {signature}, cfg_change_event: {cfg_change_event:?}");
            }
            RaydiumClmmInstruction::CreatePersonalPositionEvent(crete_personal_position) => {
                log::info!("CreatePersonalPositionEvent: signature: {signature}, crete_personal_position: {crete_personal_position:?}");
            }
            RaydiumClmmInstruction::IncreaseLiquidityEvent(increase_liq_event) => {
                log::info!("IncreaseLiquidityEvent: signature: {signature}, increase_liq_event: {increase_liq_event:?}");
            }
            RaydiumClmmInstruction::DecreaseLiquidityEvent(decrease_liq_event) => {
                log::info!("DecreaseLiquidityEvent: signature: {signature}, decrease_liq_event: {decrease_liq_event:?}");
            }
            RaydiumClmmInstruction::LiquidityCalculateEvent(liq_calc_event) => {
                log::info!("LiquidityCalculateEvent: signature: {signature}, liq_calc_event: {liq_calc_event:?}");
            }
            RaydiumClmmInstruction::CollectPersonalFeeEvent(collect_personal_fee_event) => {
                log::info!("CollectPersonalFeeEvent: signature: {signature}, collect_personal_fee_event: {collect_personal_fee_event:?}");
            }
            RaydiumClmmInstruction::UpdateRewardInfosEvent(update_reward_info_event) => {
                log::info!("UpdateRewardInfosEvent: signature: {signature}, update_reward_info_event: {update_reward_info_event:?}");
            }
            RaydiumClmmInstruction::PoolCreatedEvent(pool_create_event) => {
                log::info!("PoolCreatedEvent: signature: {signature}, pool_create_event: {pool_create_event:?}");
            }
            RaydiumClmmInstruction::CollectProtocolFeeEvent(collect_protocol_fee_event) => {
                log::info!("CollectProtocolFeeEvent: signature: {signature}, collect_protocol_fee_event: {collect_protocol_fee_event:?}");
            }
            RaydiumClmmInstruction::SwapEvent(swap_event) => {
                log::info!("SwapEvent: signature: {signature}, swap_event: {swap_event:?}");
            }
            RaydiumClmmInstruction::LiquidityChangeEvent(liq_change_event) => {
                log::info!("LiquidityChangeEvent: signature: {signature}, liq_change_event: {liq_change_event:?}");
            }
            RaydiumClmmInstruction::OpenPositionWithToken22Nft(open_position_with_token22_nft) => {
                log::info!("OpenPositionWithToken22Nft: signature: {signature}, open_position_with_token22_nft: {open_position_with_token22_nft:?}");
            }
        };

        Ok(())
    }
}
