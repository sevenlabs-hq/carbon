use super::RaydiumClmmDecoder;
pub mod close_position;
pub mod collect_fund_fee;
pub mod collect_personal_fee_event;
pub mod collect_protocol_fee;
pub mod collect_protocol_fee_event;
pub mod collect_remaining_rewards;
pub mod config_change_event;
pub mod create_amm_config;
pub mod create_operation_account;
pub mod create_personal_position_event;
pub mod create_pool;
pub mod decrease_liquidity;
pub mod decrease_liquidity_event;
pub mod decrease_liquidity_v2;
pub mod increase_liquidity;
pub mod increase_liquidity_event;
pub mod increase_liquidity_v2;
pub mod initialize_reward;
pub mod liquidity_calculate_event;
pub mod liquidity_change_event;
pub mod open_position;
pub mod open_position_v2;
pub mod pool_created_event;
pub mod set_reward_params;
pub mod swap;
pub mod swap_event;
pub mod swap_router_base_in;
pub mod swap_v2;
pub mod transfer_reward_owner;
pub mod update_amm_config;
pub mod update_operation_account;
pub mod update_pool_status;
pub mod update_reward_infos;
pub mod update_reward_infos_event;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum RaydiumClmmInstruction {
    CreateAmmConfig(create_amm_config::CreateAmmConfig),
    UpdateAmmConfig(update_amm_config::UpdateAmmConfig),
    CreatePool(create_pool::CreatePool),
    UpdatePoolStatus(update_pool_status::UpdatePoolStatus),
    CreateOperationAccount(create_operation_account::CreateOperationAccount),
    UpdateOperationAccount(update_operation_account::UpdateOperationAccount),
    TransferRewardOwner(transfer_reward_owner::TransferRewardOwner),
    InitializeReward(initialize_reward::InitializeReward),
    CollectRemainingRewards(collect_remaining_rewards::CollectRemainingRewards),
    UpdateRewardInfos(update_reward_infos::UpdateRewardInfos),
    SetRewardParams(set_reward_params::SetRewardParams),
    CollectProtocolFee(collect_protocol_fee::CollectProtocolFee),
    CollectFundFee(collect_fund_fee::CollectFundFee),
    OpenPosition(open_position::OpenPosition),
    OpenPositionV2(open_position_v2::OpenPositionV2),
    ClosePosition(close_position::ClosePosition),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    Swap(swap::Swap),
    SwapV2(swap_v2::SwapV2),
    SwapRouterBaseIn(swap_router_base_in::SwapRouterBaseIn),
    ConfigChangeEvent(config_change_event::ConfigChangeEvent),
    CreatePersonalPositionEvent(create_personal_position_event::CreatePersonalPositionEvent),
    IncreaseLiquidityEvent(increase_liquidity_event::IncreaseLiquidityEvent),
    DecreaseLiquidityEvent(decrease_liquidity_event::DecreaseLiquidityEvent),
    LiquidityCalculateEvent(liquidity_calculate_event::LiquidityCalculateEvent),
    CollectPersonalFeeEvent(collect_personal_fee_event::CollectPersonalFeeEvent),
    UpdateRewardInfosEvent(update_reward_infos_event::UpdateRewardInfosEvent),
    PoolCreatedEvent(pool_created_event::PoolCreatedEvent),
    CollectProtocolFeeEvent(collect_protocol_fee_event::CollectProtocolFeeEvent),
    SwapEvent(swap_event::SwapEvent),
    LiquidityChangeEvent(liquidity_change_event::LiquidityChangeEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for RaydiumClmmDecoder {
    type InstructionType = RaydiumClmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            RaydiumClmmInstruction::CreateAmmConfig => create_amm_config::CreateAmmConfig,
            RaydiumClmmInstruction::UpdateAmmConfig => update_amm_config::UpdateAmmConfig,
            RaydiumClmmInstruction::CreatePool => create_pool::CreatePool,
            RaydiumClmmInstruction::UpdatePoolStatus => update_pool_status::UpdatePoolStatus,
            RaydiumClmmInstruction::CreateOperationAccount => create_operation_account::CreateOperationAccount,
            RaydiumClmmInstruction::UpdateOperationAccount => update_operation_account::UpdateOperationAccount,
            RaydiumClmmInstruction::TransferRewardOwner => transfer_reward_owner::TransferRewardOwner,
            RaydiumClmmInstruction::InitializeReward => initialize_reward::InitializeReward,
            RaydiumClmmInstruction::CollectRemainingRewards => collect_remaining_rewards::CollectRemainingRewards,
            RaydiumClmmInstruction::UpdateRewardInfos => update_reward_infos::UpdateRewardInfos,
            RaydiumClmmInstruction::SetRewardParams => set_reward_params::SetRewardParams,
            RaydiumClmmInstruction::CollectProtocolFee => collect_protocol_fee::CollectProtocolFee,
            RaydiumClmmInstruction::CollectFundFee => collect_fund_fee::CollectFundFee,
            RaydiumClmmInstruction::OpenPosition => open_position::OpenPosition,
            RaydiumClmmInstruction::OpenPositionV2 => open_position_v2::OpenPositionV2,
            RaydiumClmmInstruction::ClosePosition => close_position::ClosePosition,
            RaydiumClmmInstruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            RaydiumClmmInstruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            RaydiumClmmInstruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            RaydiumClmmInstruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            RaydiumClmmInstruction::Swap => swap::Swap,
            RaydiumClmmInstruction::SwapV2 => swap_v2::SwapV2,
            RaydiumClmmInstruction::SwapRouterBaseIn => swap_router_base_in::SwapRouterBaseIn,
            RaydiumClmmInstruction::ConfigChangeEvent => config_change_event::ConfigChangeEvent,
            RaydiumClmmInstruction::CreatePersonalPositionEvent => create_personal_position_event::CreatePersonalPositionEvent,
            RaydiumClmmInstruction::IncreaseLiquidityEvent => increase_liquidity_event::IncreaseLiquidityEvent,
            RaydiumClmmInstruction::DecreaseLiquidityEvent => decrease_liquidity_event::DecreaseLiquidityEvent,
            RaydiumClmmInstruction::LiquidityCalculateEvent => liquidity_calculate_event::LiquidityCalculateEvent,
            RaydiumClmmInstruction::CollectPersonalFeeEvent => collect_personal_fee_event::CollectPersonalFeeEvent,
            RaydiumClmmInstruction::UpdateRewardInfosEvent => update_reward_infos_event::UpdateRewardInfosEvent,
            RaydiumClmmInstruction::PoolCreatedEvent => pool_created_event::PoolCreatedEvent,
            RaydiumClmmInstruction::CollectProtocolFeeEvent => collect_protocol_fee_event::CollectProtocolFeeEvent,
            RaydiumClmmInstruction::SwapEvent => swap_event::SwapEvent,
            RaydiumClmmInstruction::LiquidityChangeEvent => liquidity_change_event::LiquidityChangeEvent,
        )
    }
}
