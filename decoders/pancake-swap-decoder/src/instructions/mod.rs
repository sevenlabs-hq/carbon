use super::PancakeSwapDecoder;
pub mod close_position;
pub mod collect_fund_fee;
pub mod collect_personal_fee_event;
pub mod collect_protocol_fee;
pub mod collect_protocol_fee_event;
pub mod collect_remaining_rewards;
pub mod config_change_event;
pub mod create_amm_config;
pub mod create_operation_account;
pub mod create_permissionless_farm_switch;
pub mod create_personal_position_event;
pub mod create_pool;
pub mod create_support_mint_associated;
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
pub mod open_position_with_token22_nft;
pub mod pool_created_event;
pub mod set_reward_params;
pub mod swap;
pub mod swap_event;
pub mod swap_router_base_in;
pub mod swap_v2;
pub mod toggle_permissionless_farm_switch;
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
pub enum AmmV3Instruction {
    ClosePosition(close_position::ClosePosition),
    CollectFundFee(collect_fund_fee::CollectFundFee),
    CollectProtocolFee(collect_protocol_fee::CollectProtocolFee),
    CollectRemainingRewards(collect_remaining_rewards::CollectRemainingRewards),
    CreateAmmConfig(create_amm_config::CreateAmmConfig),
    CreateOperationAccount(create_operation_account::CreateOperationAccount),
    CreatePermissionlessFarmSwitch(
        create_permissionless_farm_switch::CreatePermissionlessFarmSwitch,
    ),
    CreatePool(create_pool::CreatePool),
    CreateSupportMintAssociated(create_support_mint_associated::CreateSupportMintAssociated),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    InitializeReward(initialize_reward::InitializeReward),
    OpenPosition(open_position::OpenPosition),
    OpenPositionV2(open_position_v2::OpenPositionV2),
    OpenPositionWithToken22Nft(open_position_with_token22_nft::OpenPositionWithToken22Nft),
    SetRewardParams(set_reward_params::SetRewardParams),
    Swap(swap::Swap),
    SwapRouterBaseIn(swap_router_base_in::SwapRouterBaseIn),
    SwapV2(swap_v2::SwapV2),
    TogglePermissionlessFarmSwitch(
        toggle_permissionless_farm_switch::TogglePermissionlessFarmSwitch,
    ),
    TransferRewardOwner(transfer_reward_owner::TransferRewardOwner),
    UpdateAmmConfig(update_amm_config::UpdateAmmConfig),
    UpdateOperationAccount(update_operation_account::UpdateOperationAccount),
    UpdatePoolStatus(update_pool_status::UpdatePoolStatus),
    UpdateRewardInfos(update_reward_infos::UpdateRewardInfos),
    CollectPersonalFeeEvent(collect_personal_fee_event::CollectPersonalFeeEvent),
    CollectProtocolFeeEvent(collect_protocol_fee_event::CollectProtocolFeeEvent),
    ConfigChangeEvent(config_change_event::ConfigChangeEvent),
    CreatePersonalPositionEvent(create_personal_position_event::CreatePersonalPositionEvent),
    DecreaseLiquidityEvent(decrease_liquidity_event::DecreaseLiquidityEvent),
    IncreaseLiquidityEvent(increase_liquidity_event::IncreaseLiquidityEvent),
    LiquidityCalculateEvent(liquidity_calculate_event::LiquidityCalculateEvent),
    LiquidityChangeEvent(liquidity_change_event::LiquidityChangeEvent),
    PoolCreatedEvent(pool_created_event::PoolCreatedEvent),
    SwapEvent(swap_event::SwapEvent),
    UpdateRewardInfosEvent(update_reward_infos_event::UpdateRewardInfosEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PancakeSwapDecoder {
    type InstructionType = AmmV3Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            AmmV3Instruction::ClosePosition => close_position::ClosePosition,
            AmmV3Instruction::CollectFundFee => collect_fund_fee::CollectFundFee,
            AmmV3Instruction::CollectProtocolFee => collect_protocol_fee::CollectProtocolFee,
            AmmV3Instruction::CollectRemainingRewards => collect_remaining_rewards::CollectRemainingRewards,
            AmmV3Instruction::CreateAmmConfig => create_amm_config::CreateAmmConfig,
            AmmV3Instruction::CreateOperationAccount => create_operation_account::CreateOperationAccount,
            AmmV3Instruction::CreatePermissionlessFarmSwitch => create_permissionless_farm_switch::CreatePermissionlessFarmSwitch,
            AmmV3Instruction::CreatePool => create_pool::CreatePool,
            AmmV3Instruction::CreateSupportMintAssociated => create_support_mint_associated::CreateSupportMintAssociated,
            AmmV3Instruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            AmmV3Instruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            AmmV3Instruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            AmmV3Instruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            AmmV3Instruction::InitializeReward => initialize_reward::InitializeReward,
            AmmV3Instruction::OpenPosition => open_position::OpenPosition,
            AmmV3Instruction::OpenPositionV2 => open_position_v2::OpenPositionV2,
            AmmV3Instruction::OpenPositionWithToken22Nft => open_position_with_token22_nft::OpenPositionWithToken22Nft,
            AmmV3Instruction::SetRewardParams => set_reward_params::SetRewardParams,
            AmmV3Instruction::Swap => swap::Swap,
            AmmV3Instruction::SwapRouterBaseIn => swap_router_base_in::SwapRouterBaseIn,
            AmmV3Instruction::SwapV2 => swap_v2::SwapV2,
            AmmV3Instruction::TogglePermissionlessFarmSwitch => toggle_permissionless_farm_switch::TogglePermissionlessFarmSwitch,
            AmmV3Instruction::TransferRewardOwner => transfer_reward_owner::TransferRewardOwner,
            AmmV3Instruction::UpdateAmmConfig => update_amm_config::UpdateAmmConfig,
            AmmV3Instruction::UpdateOperationAccount => update_operation_account::UpdateOperationAccount,
            AmmV3Instruction::UpdatePoolStatus => update_pool_status::UpdatePoolStatus,
            AmmV3Instruction::UpdateRewardInfos => update_reward_infos::UpdateRewardInfos,
            AmmV3Instruction::CollectPersonalFeeEvent => collect_personal_fee_event::CollectPersonalFeeEvent,
            AmmV3Instruction::CollectProtocolFeeEvent => collect_protocol_fee_event::CollectProtocolFeeEvent,
            AmmV3Instruction::ConfigChangeEvent => config_change_event::ConfigChangeEvent,
            AmmV3Instruction::CreatePersonalPositionEvent => create_personal_position_event::CreatePersonalPositionEvent,
            AmmV3Instruction::DecreaseLiquidityEvent => decrease_liquidity_event::DecreaseLiquidityEvent,
            AmmV3Instruction::IncreaseLiquidityEvent => increase_liquidity_event::IncreaseLiquidityEvent,
            AmmV3Instruction::LiquidityCalculateEvent => liquidity_calculate_event::LiquidityCalculateEvent,
            AmmV3Instruction::LiquidityChangeEvent => liquidity_change_event::LiquidityChangeEvent,
            AmmV3Instruction::PoolCreatedEvent => pool_created_event::PoolCreatedEvent,
            AmmV3Instruction::SwapEvent => swap_event::SwapEvent,
            AmmV3Instruction::UpdateRewardInfosEvent => update_reward_infos_event::UpdateRewardInfosEvent,
        )
    }
}
