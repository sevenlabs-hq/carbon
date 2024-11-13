use super::OrcaWhirlpoolDecoder;
pub mod close_bundled_position;
pub mod close_position;
pub mod collect_fees;
pub mod collect_fees_v2;
pub mod collect_protocol_fees;
pub mod collect_protocol_fees_v2;
pub mod collect_reward;
pub mod collect_reward_v2;
pub mod decrease_liquidity;
pub mod decrease_liquidity_v2;
pub mod delete_position_bundle;
pub mod delete_token_badge;
pub mod increase_liquidity;
pub mod increase_liquidity_v2;
pub mod initialize_config;
pub mod initialize_config_extension;
pub mod initialize_fee_tier;
pub mod initialize_pool;
pub mod initialize_pool_v2;
pub mod initialize_position_bundle;
pub mod initialize_position_bundle_with_metadata;
pub mod initialize_reward;
pub mod initialize_reward_v2;
pub mod initialize_tick_array;
pub mod initialize_token_badge;
pub mod open_bundled_position;
pub mod open_position;
pub mod open_position_with_metadata;
pub mod set_collect_protocol_fees_authority;
pub mod set_config_extension_authority;
pub mod set_default_fee_rate;
pub mod set_default_protocol_fee_rate;
pub mod set_fee_authority;
pub mod set_fee_rate;
pub mod set_protocol_fee_rate;
pub mod set_reward_authority;
pub mod set_reward_authority_by_super_authority;
pub mod set_reward_emissions;
pub mod set_reward_emissions_super_authority;
pub mod set_reward_emissions_v2;
pub mod set_token_badge_authority;
pub mod swap;
pub mod swap_v2;
pub mod two_hop_swap;
pub mod two_hop_swap_v2;
pub mod update_fees_and_rewards;

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
pub enum OrcaWhirlpoolInstruction {
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
    SetCollectProtocolFeesAuthority(
        set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority,
    ),
    SetRewardAuthority(set_reward_authority::SetRewardAuthority),
    SetRewardAuthorityBySuperAuthority(
        set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority,
    ),
    SetRewardEmissionsSuperAuthority(
        set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority,
    ),
    TwoHopSwap(two_hop_swap::TwoHopSwap),
    InitializePositionBundle(initialize_position_bundle::InitializePositionBundle),
    InitializePositionBundleWithMetadata(
        initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata,
    ),
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

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for OrcaWhirlpoolDecoder {
    type InstructionType = OrcaWhirlpoolInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            OrcaWhirlpoolInstruction::InitializeConfig => initialize_config::InitializeConfig,
            OrcaWhirlpoolInstruction::InitializePool => initialize_pool::InitializePool,
            OrcaWhirlpoolInstruction::InitializeTickArray => initialize_tick_array::InitializeTickArray,
            OrcaWhirlpoolInstruction::InitializeFeeTier => initialize_fee_tier::InitializeFeeTier,
            OrcaWhirlpoolInstruction::InitializeReward => initialize_reward::InitializeReward,
            OrcaWhirlpoolInstruction::SetRewardEmissions => set_reward_emissions::SetRewardEmissions,
            OrcaWhirlpoolInstruction::OpenPosition => open_position::OpenPosition,
            OrcaWhirlpoolInstruction::OpenPositionWithMetadata => open_position_with_metadata::OpenPositionWithMetadata,
            OrcaWhirlpoolInstruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            OrcaWhirlpoolInstruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            OrcaWhirlpoolInstruction::UpdateFeesAndRewards => update_fees_and_rewards::UpdateFeesAndRewards,
            OrcaWhirlpoolInstruction::CollectFees => collect_fees::CollectFees,
            OrcaWhirlpoolInstruction::CollectReward => collect_reward::CollectReward,
            OrcaWhirlpoolInstruction::CollectProtocolFees => collect_protocol_fees::CollectProtocolFees,
            OrcaWhirlpoolInstruction::Swap => swap::Swap,
            OrcaWhirlpoolInstruction::ClosePosition => close_position::ClosePosition,
            OrcaWhirlpoolInstruction::SetDefaultFeeRate => set_default_fee_rate::SetDefaultFeeRate,
            OrcaWhirlpoolInstruction::SetDefaultProtocolFeeRate => set_default_protocol_fee_rate::SetDefaultProtocolFeeRate,
            OrcaWhirlpoolInstruction::SetFeeRate => set_fee_rate::SetFeeRate,
            OrcaWhirlpoolInstruction::SetProtocolFeeRate => set_protocol_fee_rate::SetProtocolFeeRate,
            OrcaWhirlpoolInstruction::SetFeeAuthority => set_fee_authority::SetFeeAuthority,
            OrcaWhirlpoolInstruction::SetCollectProtocolFeesAuthority => set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority,
            OrcaWhirlpoolInstruction::SetRewardAuthority => set_reward_authority::SetRewardAuthority,
            OrcaWhirlpoolInstruction::SetRewardAuthorityBySuperAuthority => set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority,
            OrcaWhirlpoolInstruction::SetRewardEmissionsSuperAuthority => set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority,
            OrcaWhirlpoolInstruction::TwoHopSwap => two_hop_swap::TwoHopSwap,
            OrcaWhirlpoolInstruction::InitializePositionBundle => initialize_position_bundle::InitializePositionBundle,
            OrcaWhirlpoolInstruction::InitializePositionBundleWithMetadata => initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata,
            OrcaWhirlpoolInstruction::DeletePositionBundle => delete_position_bundle::DeletePositionBundle,
            OrcaWhirlpoolInstruction::OpenBundledPosition => open_bundled_position::OpenBundledPosition,
            OrcaWhirlpoolInstruction::CloseBundledPosition => close_bundled_position::CloseBundledPosition,
            OrcaWhirlpoolInstruction::CollectFeesV2 => collect_fees_v2::CollectFeesV2,
            OrcaWhirlpoolInstruction::CollectProtocolFeesV2 => collect_protocol_fees_v2::CollectProtocolFeesV2,
            OrcaWhirlpoolInstruction::CollectRewardV2 => collect_reward_v2::CollectRewardV2,
            OrcaWhirlpoolInstruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            OrcaWhirlpoolInstruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            OrcaWhirlpoolInstruction::InitializePoolV2 => initialize_pool_v2::InitializePoolV2,
            OrcaWhirlpoolInstruction::InitializeRewardV2 => initialize_reward_v2::InitializeRewardV2,
            OrcaWhirlpoolInstruction::SetRewardEmissionsV2 => set_reward_emissions_v2::SetRewardEmissionsV2,
            OrcaWhirlpoolInstruction::SwapV2 => swap_v2::SwapV2,
            OrcaWhirlpoolInstruction::TwoHopSwapV2 => two_hop_swap_v2::TwoHopSwapV2,
            OrcaWhirlpoolInstruction::InitializeConfigExtension => initialize_config_extension::InitializeConfigExtension,
            OrcaWhirlpoolInstruction::SetConfigExtensionAuthority => set_config_extension_authority::SetConfigExtensionAuthority,
            OrcaWhirlpoolInstruction::SetTokenBadgeAuthority => set_token_badge_authority::SetTokenBadgeAuthority,
            OrcaWhirlpoolInstruction::InitializeTokenBadge => initialize_token_badge::InitializeTokenBadge,
            OrcaWhirlpoolInstruction::DeleteTokenBadge => delete_token_badge::DeleteTokenBadge,
        )
    }
}
