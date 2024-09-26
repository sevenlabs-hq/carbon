
use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;


use crate::AmmV3Decoder;
pub mod create_amm_config;
pub mod update_amm_config;
pub mod create_pool;
pub mod update_pool_status;
pub mod create_operation_account;
pub mod update_operation_account;
pub mod transfer_reward_owner;
pub mod initialize_reward;
pub mod collect_remaining_rewards;
pub mod update_reward_infos;
pub mod set_reward_params;
pub mod collect_protocol_fee;
pub mod collect_fund_fee;
pub mod open_position;
pub mod open_position_v2;
pub mod close_position;
pub mod increase_liquidity;
pub mod increase_liquidity_v2;
pub mod decrease_liquidity;
pub mod decrease_liquidity_v2;
pub mod swap;
pub mod swap_v2;
pub mod swap_router_base_in;
pub mod config_change_event;
pub mod create_personal_position_event;
pub mod increase_liquidity_event;
pub mod decrease_liquidity_event;
pub mod liquidity_calculate_event;
pub mod collect_personal_fee_event;
pub mod update_reward_infos_event;
pub mod pool_created_event;
pub mod collect_protocol_fee_event;
pub mod swap_event;
pub mod liquidity_change_event;

#[derive(carbon_proc_macros::InstructionType, serde::Serialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum AmmV3Instruction {
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

impl InstructionDecoder for AmmV3Decoder {
    type InstructionType = AmmV3Instruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) = create_amm_config::CreateAmmConfig::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CreateAmmConfig(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_amm_config::UpdateAmmConfig::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::UpdateAmmConfig(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_pool::CreatePool::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CreatePool(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_pool_status::UpdatePoolStatus::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::UpdatePoolStatus(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_operation_account::CreateOperationAccount::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CreateOperationAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_operation_account::UpdateOperationAccount::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::UpdateOperationAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = transfer_reward_owner::TransferRewardOwner::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::TransferRewardOwner(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_reward::InitializeReward::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::InitializeReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_remaining_rewards::CollectRemainingRewards::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CollectRemainingRewards(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_reward_infos::UpdateRewardInfos::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::UpdateRewardInfos(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_reward_params::SetRewardParams::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::SetRewardParams(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_protocol_fee::CollectProtocolFee::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CollectProtocolFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_fund_fee::CollectFundFee::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CollectFundFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = open_position::OpenPosition::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::OpenPosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = open_position_v2::OpenPositionV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::OpenPositionV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = close_position::ClosePosition::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::ClosePosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = increase_liquidity::IncreaseLiquidity::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::IncreaseLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = increase_liquidity_v2::IncreaseLiquidityV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::IncreaseLiquidityV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = decrease_liquidity::DecreaseLiquidity::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::DecreaseLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = decrease_liquidity_v2::DecreaseLiquidityV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::DecreaseLiquidityV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap::Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap_v2::SwapV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::SwapV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap_router_base_in::SwapRouterBaseIn::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::SwapRouterBaseIn(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = config_change_event::ConfigChangeEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::ConfigChangeEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_personal_position_event::CreatePersonalPositionEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CreatePersonalPositionEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = increase_liquidity_event::IncreaseLiquidityEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::IncreaseLiquidityEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = decrease_liquidity_event::DecreaseLiquidityEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::DecreaseLiquidityEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = liquidity_calculate_event::LiquidityCalculateEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::LiquidityCalculateEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_personal_fee_event::CollectPersonalFeeEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CollectPersonalFeeEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_reward_infos_event::UpdateRewardInfosEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::UpdateRewardInfosEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = pool_created_event::PoolCreatedEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::PoolCreatedEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_protocol_fee_event::CollectProtocolFeeEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::CollectProtocolFeeEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap_event::SwapEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::SwapEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = liquidity_change_event::LiquidityChangeEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: AmmV3Instruction::LiquidityChangeEvent(decoded_instruction),
            });
        }

        None
    }
}