
use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;


use super::WhirlpoolDecoder;
pub mod initialize_config;
pub mod initialize_pool;
pub mod initialize_tick_array;
pub mod initialize_fee_tier;
pub mod initialize_reward;
pub mod set_reward_emissions;
pub mod open_position;
pub mod open_position_with_metadata;
pub mod increase_liquidity;
pub mod decrease_liquidity;
pub mod update_fees_and_rewards;
pub mod collect_fees;
pub mod collect_reward;
pub mod collect_protocol_fees;
pub mod swap;
pub mod close_position;
pub mod set_default_fee_rate;
pub mod set_default_protocol_fee_rate;
pub mod set_fee_rate;
pub mod set_protocol_fee_rate;
pub mod set_fee_authority;
pub mod set_collect_protocol_fees_authority;
pub mod set_reward_authority;
pub mod set_reward_authority_by_super_authority;
pub mod set_reward_emissions_super_authority;
pub mod two_hop_swap;
pub mod initialize_position_bundle;
pub mod initialize_position_bundle_with_metadata;
pub mod delete_position_bundle;
pub mod open_bundled_position;
pub mod close_bundled_position;
pub mod collect_fees_v2;
pub mod collect_protocol_fees_v2;
pub mod collect_reward_v2;
pub mod decrease_liquidity_v2;
pub mod increase_liquidity_v2;
pub mod initialize_pool_v2;
pub mod initialize_reward_v2;
pub mod set_reward_emissions_v2;
pub mod swap_v2;
pub mod two_hop_swap_v2;
pub mod initialize_config_extension;
pub mod set_config_extension_authority;
pub mod set_token_badge_authority;
pub mod initialize_token_badge;
pub mod delete_token_badge;

#[derive(carbon_proc_macros::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum WhirlpoolInstruction {
    InitializeConfig(initialize_config::InitializeConfig),
    InitializePool(initialize_pool::InitializePool),
    InitializeTickArray(initialize_tick_array::InitializeTickArray),
    InitializeFeeTier(initialize_fee_tier::InitializeFeeTier),
    InitializeReward(initialize_reward::InitializeReward),
    SetRewardEmissions(set_reward_emissions::SetRewardEmissions),
    OpenPosition(open_position::OpenPosition),
    OpenPositionWithMetadata(open_position_with_metadata::OpenPositionWithMetadata),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    UpdateFeesAndRewards(update_fees_and_rewards::UpdateFeesAndRewards),
    CollectFees(collect_fees::CollectFees),
    CollectReward(collect_reward::CollectReward),
    CollectProtocolFees(collect_protocol_fees::CollectProtocolFees),
    Swap(swap::Swap),
    ClosePosition(close_position::ClosePosition),
    SetDefaultFeeRate(set_default_fee_rate::SetDefaultFeeRate),
    SetDefaultProtocolFeeRate(set_default_protocol_fee_rate::SetDefaultProtocolFeeRate),
    SetFeeRate(set_fee_rate::SetFeeRate),
    SetProtocolFeeRate(set_protocol_fee_rate::SetProtocolFeeRate),
    SetFeeAuthority(set_fee_authority::SetFeeAuthority),
    SetCollectProtocolFeesAuthority(set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority),
    SetRewardAuthority(set_reward_authority::SetRewardAuthority),
    SetRewardAuthorityBySuperAuthority(set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority),
    SetRewardEmissionsSuperAuthority(set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority),
    TwoHopSwap(two_hop_swap::TwoHopSwap),
    InitializePositionBundle(initialize_position_bundle::InitializePositionBundle),
    InitializePositionBundleWithMetadata(initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata),
    DeletePositionBundle(delete_position_bundle::DeletePositionBundle),
    OpenBundledPosition(open_bundled_position::OpenBundledPosition),
    CloseBundledPosition(close_bundled_position::CloseBundledPosition),
    CollectFeesV2(collect_fees_v2::CollectFeesV2),
    CollectProtocolFeesV2(collect_protocol_fees_v2::CollectProtocolFeesV2),
    CollectRewardV2(collect_reward_v2::CollectRewardV2),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    InitializePoolV2(initialize_pool_v2::InitializePoolV2),
    InitializeRewardV2(initialize_reward_v2::InitializeRewardV2),
    SetRewardEmissionsV2(set_reward_emissions_v2::SetRewardEmissionsV2),
    SwapV2(swap_v2::SwapV2),
    TwoHopSwapV2(two_hop_swap_v2::TwoHopSwapV2),
    InitializeConfigExtension(initialize_config_extension::InitializeConfigExtension),
    SetConfigExtensionAuthority(set_config_extension_authority::SetConfigExtensionAuthority),
    SetTokenBadgeAuthority(set_token_badge_authority::SetTokenBadgeAuthority),
    InitializeTokenBadge(initialize_token_badge::InitializeTokenBadge),
    DeleteTokenBadge(delete_token_badge::DeleteTokenBadge),
}

impl InstructionDecoder for WhirlpoolDecoder {
    type InstructionType = WhirlpoolInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) = initialize_config::InitializeConfig::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializeConfig(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_pool::InitializePool::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializePool(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_tick_array::InitializeTickArray::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializeTickArray(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_fee_tier::InitializeFeeTier::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializeFeeTier(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_reward::InitializeReward::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializeReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_reward_emissions::SetRewardEmissions::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetRewardEmissions(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = open_position::OpenPosition::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::OpenPosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = open_position_with_metadata::OpenPositionWithMetadata::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::OpenPositionWithMetadata(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = increase_liquidity::IncreaseLiquidity::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::IncreaseLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = decrease_liquidity::DecreaseLiquidity::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::DecreaseLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_fees_and_rewards::UpdateFeesAndRewards::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::UpdateFeesAndRewards(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_fees::CollectFees::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::CollectFees(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_reward::CollectReward::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::CollectReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_protocol_fees::CollectProtocolFees::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::CollectProtocolFees(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap::Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = close_position::ClosePosition::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::ClosePosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_default_fee_rate::SetDefaultFeeRate::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetDefaultFeeRate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_default_protocol_fee_rate::SetDefaultProtocolFeeRate::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetDefaultProtocolFeeRate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_fee_rate::SetFeeRate::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetFeeRate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_protocol_fee_rate::SetProtocolFeeRate::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetProtocolFeeRate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_fee_authority::SetFeeAuthority::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetFeeAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetCollectProtocolFeesAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_reward_authority::SetRewardAuthority::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetRewardAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetRewardAuthorityBySuperAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetRewardEmissionsSuperAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = two_hop_swap::TwoHopSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::TwoHopSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_position_bundle::InitializePositionBundle::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializePositionBundle(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializePositionBundleWithMetadata(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = delete_position_bundle::DeletePositionBundle::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::DeletePositionBundle(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = open_bundled_position::OpenBundledPosition::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::OpenBundledPosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = close_bundled_position::CloseBundledPosition::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::CloseBundledPosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_fees_v2::CollectFeesV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::CollectFeesV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_protocol_fees_v2::CollectProtocolFeesV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::CollectProtocolFeesV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect_reward_v2::CollectRewardV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::CollectRewardV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = decrease_liquidity_v2::DecreaseLiquidityV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::DecreaseLiquidityV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = increase_liquidity_v2::IncreaseLiquidityV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::IncreaseLiquidityV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_pool_v2::InitializePoolV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializePoolV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_reward_v2::InitializeRewardV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializeRewardV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_reward_emissions_v2::SetRewardEmissionsV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetRewardEmissionsV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap_v2::SwapV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SwapV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = two_hop_swap_v2::TwoHopSwapV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::TwoHopSwapV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_config_extension::InitializeConfigExtension::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializeConfigExtension(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_config_extension_authority::SetConfigExtensionAuthority::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetConfigExtensionAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_token_badge_authority::SetTokenBadgeAuthority::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::SetTokenBadgeAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_token_badge::InitializeTokenBadge::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::InitializeTokenBadge(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = delete_token_badge::DeleteTokenBadge::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: WhirlpoolInstruction::DeleteTokenBadge(decoded_instruction),
            });
        }

        None
    }
}