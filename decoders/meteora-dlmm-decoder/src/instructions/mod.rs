use crate::PROGRAM_ID;

use super::MeteoraDlmmDecoder;
pub mod add_liquidity;
pub mod add_liquidity2;
pub mod add_liquidity_by_strategy;
pub mod add_liquidity_by_strategy2;
pub mod add_liquidity_by_strategy_one_side;
pub mod add_liquidity_by_weight;
pub mod add_liquidity_event;
pub mod add_liquidity_one_side;
pub mod add_liquidity_one_side_precise;
pub mod add_liquidity_one_side_precise2;
pub mod claim_fee;
pub mod claim_fee2;
pub mod claim_fee_event;
pub mod claim_reward;
pub mod claim_reward2;
pub mod claim_reward_event;
pub mod close_claim_protocol_fee_operator;
pub mod close_position;
pub mod close_position2;
pub mod close_position_if_empty;
pub mod close_preset_parameter;
pub mod close_preset_parameter2;
pub mod composition_fee_event;
pub mod create_claim_protocol_fee_operator;
pub mod decrease_position_length_event;
pub mod dynamic_fee_parameter_update_event;
pub mod fee_parameter_update_event;
pub mod fund_reward;
pub mod fund_reward_event;
pub mod go_to_a_bin;
pub mod go_to_a_bin_event;
pub mod increase_observation_event;
pub mod increase_oracle_length;
pub mod increase_position_length_event;
pub mod initialize_bin_array;
pub mod initialize_bin_array_bitmap_extension;
pub mod initialize_customizable_permissionless_lb_pair;
pub mod initialize_customizable_permissionless_lb_pair2;
pub mod initialize_lb_pair;
pub mod initialize_lb_pair2;
pub mod initialize_permission_lb_pair;
pub mod initialize_position;
pub mod initialize_position_by_operator;
pub mod initialize_position_pda;
pub mod initialize_preset_parameter;
pub mod initialize_preset_parameter2;
pub mod initialize_reward;
pub mod initialize_reward_event;
pub mod initialize_token_badge;
pub mod lb_pair_create_event;
pub mod migrate_bin_array;
pub mod migrate_position;
pub mod position_close_event;
pub mod position_create_event;
pub mod remove_all_liquidity;
pub mod remove_liquidity;
pub mod remove_liquidity2;
pub mod remove_liquidity_by_range;
pub mod remove_liquidity_by_range2;
pub mod remove_liquidity_event;
pub mod set_activation_point;
pub mod set_pair_status;
pub mod set_pair_status_permissionless;
pub mod set_pre_activation_duration;
pub mod set_pre_activation_swap_address;
pub mod swap;
pub mod swap2;
pub mod swap_event;
pub mod swap_exact_out;
pub mod swap_exact_out2;
pub mod swap_with_price_impact;
pub mod swap_with_price_impact2;
pub mod update_base_fee_parameters;
pub mod update_dynamic_fee_parameters;
pub mod update_fees_and_reward2;
pub mod update_fees_and_rewards;
pub mod update_position_lock_release_point_event;
pub mod update_position_operator;
pub mod update_position_operator_event;
pub mod update_reward_duration;
pub mod update_reward_duration_event;
pub mod update_reward_funder;
pub mod update_reward_funder_event;
pub mod withdraw_ineligible_reward;
pub mod withdraw_ineligible_reward_event;
pub mod withdraw_protocol_fee;

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
pub enum MeteoraDlmmInstruction {
    InitializeLbPair(initialize_lb_pair::InitializeLbPair),
    InitializePermissionLbPair(initialize_permission_lb_pair::InitializePermissionLbPair),
    InitializeCustomizablePermissionlessLbPair(initialize_customizable_permissionless_lb_pair::InitializeCustomizablePermissionlessLbPair),
    InitializeBinArrayBitmapExtension(initialize_bin_array_bitmap_extension::InitializeBinArrayBitmapExtension),
    InitializeBinArray(initialize_bin_array::InitializeBinArray),
    AddLiquidity(add_liquidity::AddLiquidity),
    AddLiquidityByWeight(add_liquidity_by_weight::AddLiquidityByWeight),
    AddLiquidityByStrategy(add_liquidity_by_strategy::AddLiquidityByStrategy),
    AddLiquidityByStrategyOneSide(add_liquidity_by_strategy_one_side::AddLiquidityByStrategyOneSide),
    AddLiquidityOneSide(add_liquidity_one_side::AddLiquidityOneSide),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    InitializePosition(initialize_position::InitializePosition),
    InitializePositionPda(initialize_position_pda::InitializePositionPda),
    InitializePositionByOperator(initialize_position_by_operator::InitializePositionByOperator),
    UpdatePositionOperator(update_position_operator::UpdatePositionOperator),
    Swap(swap::Swap),
    SwapExactOut(swap_exact_out::SwapExactOut),
    SwapWithPriceImpact(swap_with_price_impact::SwapWithPriceImpact),
    WithdrawProtocolFee(withdraw_protocol_fee::WithdrawProtocolFee),
    InitializeReward(initialize_reward::InitializeReward),
    FundReward(fund_reward::FundReward),
    UpdateRewardFunder(update_reward_funder::UpdateRewardFunder),
    UpdateRewardDuration(update_reward_duration::UpdateRewardDuration),
    ClaimReward(claim_reward::ClaimReward),
    ClaimFee(claim_fee::ClaimFee),
    ClosePosition(close_position::ClosePosition),
    UpdateBaseFeeParameters(update_base_fee_parameters::UpdateBaseFeeParameters),
    UpdateDynamicFeeParameters(update_dynamic_fee_parameters::UpdateDynamicFeeParameters),
    IncreaseOracleLength(increase_oracle_length::IncreaseOracleLength),
    InitializePresetParameter(initialize_preset_parameter::InitializePresetParameter),
    ClosePresetParameter(close_preset_parameter::ClosePresetParameter),
    ClosePresetParameter2(close_preset_parameter2::ClosePresetParameter2),
    RemoveAllLiquidity(remove_all_liquidity::RemoveAllLiquidity),
    SetPairStatus(set_pair_status::SetPairStatus),
    MigratePosition(migrate_position::MigratePosition),
    MigrateBinArray(migrate_bin_array::MigrateBinArray),
    UpdateFeesAndRewards(update_fees_and_rewards::UpdateFeesAndRewards),
    WithdrawIneligibleReward(withdraw_ineligible_reward::WithdrawIneligibleReward),
    SetActivationPoint(set_activation_point::SetActivationPoint),
    RemoveLiquidityByRange(remove_liquidity_by_range::RemoveLiquidityByRange),
    AddLiquidityOneSidePrecise(add_liquidity_one_side_precise::AddLiquidityOneSidePrecise),
    GoToABin(go_to_a_bin::GoToABin),
    SetPreActivationDuration(set_pre_activation_duration::SetPreActivationDuration),
    SetPreActivationSwapAddress(set_pre_activation_swap_address::SetPreActivationSwapAddress),
    SetPairStatusPermissionless(set_pair_status_permissionless::SetPairStatusPermissionless),
    InitializeTokenBadge(initialize_token_badge::InitializeTokenBadge),
    CreateClaimProtocolFeeOperator(create_claim_protocol_fee_operator::CreateClaimProtocolFeeOperator),
    CloseClaimProtocolFeeOperator(close_claim_protocol_fee_operator::CloseClaimProtocolFeeOperator),
    InitializePresetParameter2(initialize_preset_parameter2::InitializePresetParameter2),
    InitializeLbPair2(initialize_lb_pair2::InitializeLbPair2),
    InitializeCustomizablePermissionlessLbPair2(initialize_customizable_permissionless_lb_pair2::InitializeCustomizablePermissionlessLbPair2),
    ClaimFee2(claim_fee2::ClaimFee2),
    ClaimReward2(claim_reward2::ClaimReward2),
    AddLiquidity2(add_liquidity2::AddLiquidity2),
    AddLiquidityByStrategy2(add_liquidity_by_strategy2::AddLiquidityByStrategy2),
    AddLiquidityOneSidePrecise2(add_liquidity_one_side_precise2::AddLiquidityOneSidePrecise2),
    RemoveLiquidity2(remove_liquidity2::RemoveLiquidity2),
    RemoveLiquidityByRange2(remove_liquidity_by_range2::RemoveLiquidityByRange2),
    Swap2(swap2::Swap2),
    SwapExactOut2(swap_exact_out2::SwapExactOut2),
    SwapWithPriceImpact2(swap_with_price_impact2::SwapWithPriceImpact2),
    ClosePosition2(close_position2::ClosePosition2),
    UpdateFeesAndReward2(update_fees_and_reward2::UpdateFeesAndReward2),
    ClosePositionIfEmpty(close_position_if_empty::ClosePositionIfEmpty),
    CompositionFeeEvent(composition_fee_event::CompositionFeeEvent),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    SwapEvent(swap_event::SwapEvent),
    ClaimRewardEvent(claim_reward_event::ClaimRewardEvent),
    FundRewardEvent(fund_reward_event::FundRewardEvent),
    InitializeRewardEvent(initialize_reward_event::InitializeRewardEvent),
    UpdateRewardDurationEvent(update_reward_duration_event::UpdateRewardDurationEvent),
    UpdateRewardFunderEvent(update_reward_funder_event::UpdateRewardFunderEvent),
    PositionCloseEvent(position_close_event::PositionCloseEvent),
    ClaimFeeEvent(claim_fee_event::ClaimFeeEvent),
    LbPairCreateEvent(lb_pair_create_event::LbPairCreateEvent),
    PositionCreateEvent(position_create_event::PositionCreateEvent),
    IncreasePositionLengthEvent(increase_position_length_event::IncreasePositionLengthEvent),
    DecreasePositionLengthEvent(decrease_position_length_event::DecreasePositionLengthEvent),
    FeeParameterUpdateEvent(fee_parameter_update_event::FeeParameterUpdateEvent),
    DynamicFeeParameterUpdateEvent(dynamic_fee_parameter_update_event::DynamicFeeParameterUpdateEvent),
    IncreaseObservationEvent(increase_observation_event::IncreaseObservationEvent),
    WithdrawIneligibleRewardEvent(withdraw_ineligible_reward_event::WithdrawIneligibleRewardEvent),
    UpdatePositionOperatorEvent(update_position_operator_event::UpdatePositionOperatorEvent),
    UpdatePositionLockReleasePointEvent(update_position_lock_release_point_event::UpdatePositionLockReleasePointEvent),
    GoToABinEvent(go_to_a_bin_event::GoToABinEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MeteoraDlmmDecoder {
    type InstructionType = MeteoraDlmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            MeteoraDlmmInstruction::InitializeLbPair => initialize_lb_pair::InitializeLbPair,
            MeteoraDlmmInstruction::InitializePermissionLbPair => initialize_permission_lb_pair::InitializePermissionLbPair,
            MeteoraDlmmInstruction::InitializeCustomizablePermissionlessLbPair => initialize_customizable_permissionless_lb_pair::InitializeCustomizablePermissionlessLbPair,
            MeteoraDlmmInstruction::InitializeBinArrayBitmapExtension => initialize_bin_array_bitmap_extension::InitializeBinArrayBitmapExtension,
            MeteoraDlmmInstruction::InitializeBinArray => initialize_bin_array::InitializeBinArray,
            MeteoraDlmmInstruction::AddLiquidity => add_liquidity::AddLiquidity,
            MeteoraDlmmInstruction::AddLiquidityByWeight => add_liquidity_by_weight::AddLiquidityByWeight,
            MeteoraDlmmInstruction::AddLiquidityByStrategy => add_liquidity_by_strategy::AddLiquidityByStrategy,
            MeteoraDlmmInstruction::AddLiquidityByStrategyOneSide => add_liquidity_by_strategy_one_side::AddLiquidityByStrategyOneSide,
            MeteoraDlmmInstruction::AddLiquidityOneSide => add_liquidity_one_side::AddLiquidityOneSide,
            MeteoraDlmmInstruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            MeteoraDlmmInstruction::InitializePosition => initialize_position::InitializePosition,
            MeteoraDlmmInstruction::InitializePositionPda => initialize_position_pda::InitializePositionPda,
            MeteoraDlmmInstruction::InitializePositionByOperator => initialize_position_by_operator::InitializePositionByOperator,
            MeteoraDlmmInstruction::UpdatePositionOperator => update_position_operator::UpdatePositionOperator,
            MeteoraDlmmInstruction::Swap => swap::Swap,
            MeteoraDlmmInstruction::SwapExactOut => swap_exact_out::SwapExactOut,
            MeteoraDlmmInstruction::SwapWithPriceImpact => swap_with_price_impact::SwapWithPriceImpact,
            MeteoraDlmmInstruction::WithdrawProtocolFee => withdraw_protocol_fee::WithdrawProtocolFee,
            MeteoraDlmmInstruction::InitializeReward => initialize_reward::InitializeReward,
            MeteoraDlmmInstruction::FundReward => fund_reward::FundReward,
            MeteoraDlmmInstruction::UpdateRewardFunder => update_reward_funder::UpdateRewardFunder,
            MeteoraDlmmInstruction::UpdateRewardDuration => update_reward_duration::UpdateRewardDuration,
            MeteoraDlmmInstruction::ClaimReward => claim_reward::ClaimReward,
            MeteoraDlmmInstruction::ClaimFee => claim_fee::ClaimFee,
            MeteoraDlmmInstruction::ClosePosition => close_position::ClosePosition,
            MeteoraDlmmInstruction::UpdateBaseFeeParameters => update_base_fee_parameters::UpdateBaseFeeParameters,
            MeteoraDlmmInstruction::UpdateDynamicFeeParameters => update_dynamic_fee_parameters::UpdateDynamicFeeParameters,
            MeteoraDlmmInstruction::IncreaseOracleLength => increase_oracle_length::IncreaseOracleLength,
            MeteoraDlmmInstruction::InitializePresetParameter => initialize_preset_parameter::InitializePresetParameter,
            MeteoraDlmmInstruction::ClosePresetParameter => close_preset_parameter::ClosePresetParameter,
            MeteoraDlmmInstruction::ClosePresetParameter2 => close_preset_parameter2::ClosePresetParameter2,
            MeteoraDlmmInstruction::RemoveAllLiquidity => remove_all_liquidity::RemoveAllLiquidity,
            MeteoraDlmmInstruction::SetPairStatus => set_pair_status::SetPairStatus,
            MeteoraDlmmInstruction::MigratePosition => migrate_position::MigratePosition,
            MeteoraDlmmInstruction::MigrateBinArray => migrate_bin_array::MigrateBinArray,
            MeteoraDlmmInstruction::UpdateFeesAndRewards => update_fees_and_rewards::UpdateFeesAndRewards,
            MeteoraDlmmInstruction::WithdrawIneligibleReward => withdraw_ineligible_reward::WithdrawIneligibleReward,
            MeteoraDlmmInstruction::SetActivationPoint => set_activation_point::SetActivationPoint,
            MeteoraDlmmInstruction::RemoveLiquidityByRange => remove_liquidity_by_range::RemoveLiquidityByRange,
            MeteoraDlmmInstruction::AddLiquidityOneSidePrecise => add_liquidity_one_side_precise::AddLiquidityOneSidePrecise,
            MeteoraDlmmInstruction::GoToABin => go_to_a_bin::GoToABin,
            MeteoraDlmmInstruction::SetPreActivationDuration => set_pre_activation_duration::SetPreActivationDuration,
            MeteoraDlmmInstruction::SetPreActivationSwapAddress => set_pre_activation_swap_address::SetPreActivationSwapAddress,
            MeteoraDlmmInstruction::SetPairStatusPermissionless => set_pair_status_permissionless::SetPairStatusPermissionless,
            MeteoraDlmmInstruction::InitializeTokenBadge => initialize_token_badge::InitializeTokenBadge,
            MeteoraDlmmInstruction::CreateClaimProtocolFeeOperator => create_claim_protocol_fee_operator::CreateClaimProtocolFeeOperator,
            MeteoraDlmmInstruction::CloseClaimProtocolFeeOperator => close_claim_protocol_fee_operator::CloseClaimProtocolFeeOperator,
            MeteoraDlmmInstruction::InitializePresetParameter2 => initialize_preset_parameter2::InitializePresetParameter2,
            MeteoraDlmmInstruction::InitializeLbPair2 => initialize_lb_pair2::InitializeLbPair2,
            MeteoraDlmmInstruction::InitializeCustomizablePermissionlessLbPair2 => initialize_customizable_permissionless_lb_pair2::InitializeCustomizablePermissionlessLbPair2,
            MeteoraDlmmInstruction::ClaimFee2 => claim_fee2::ClaimFee2,
            MeteoraDlmmInstruction::ClaimReward2 => claim_reward2::ClaimReward2,
            MeteoraDlmmInstruction::AddLiquidity2 => add_liquidity2::AddLiquidity2,
            MeteoraDlmmInstruction::AddLiquidityByStrategy2 => add_liquidity_by_strategy2::AddLiquidityByStrategy2,
            MeteoraDlmmInstruction::AddLiquidityOneSidePrecise2 => add_liquidity_one_side_precise2::AddLiquidityOneSidePrecise2,
            MeteoraDlmmInstruction::RemoveLiquidity2 => remove_liquidity2::RemoveLiquidity2,
            MeteoraDlmmInstruction::RemoveLiquidityByRange2 => remove_liquidity_by_range2::RemoveLiquidityByRange2,
            MeteoraDlmmInstruction::Swap2 => swap2::Swap2,
            MeteoraDlmmInstruction::SwapExactOut2 => swap_exact_out2::SwapExactOut2,
            MeteoraDlmmInstruction::SwapWithPriceImpact2 => swap_with_price_impact2::SwapWithPriceImpact2,
            MeteoraDlmmInstruction::ClosePosition2 => close_position2::ClosePosition2,
            MeteoraDlmmInstruction::UpdateFeesAndReward2 => update_fees_and_reward2::UpdateFeesAndReward2,
            MeteoraDlmmInstruction::ClosePositionIfEmpty => close_position_if_empty::ClosePositionIfEmpty,
            MeteoraDlmmInstruction::CompositionFeeEvent => composition_fee_event::CompositionFeeEvent,
            MeteoraDlmmInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            MeteoraDlmmInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            MeteoraDlmmInstruction::SwapEvent => swap_event::SwapEvent,
            MeteoraDlmmInstruction::ClaimRewardEvent => claim_reward_event::ClaimRewardEvent,
            MeteoraDlmmInstruction::FundRewardEvent => fund_reward_event::FundRewardEvent,
            MeteoraDlmmInstruction::InitializeRewardEvent => initialize_reward_event::InitializeRewardEvent,
            MeteoraDlmmInstruction::UpdateRewardDurationEvent => update_reward_duration_event::UpdateRewardDurationEvent,
            MeteoraDlmmInstruction::UpdateRewardFunderEvent => update_reward_funder_event::UpdateRewardFunderEvent,
            MeteoraDlmmInstruction::PositionCloseEvent => position_close_event::PositionCloseEvent,
            MeteoraDlmmInstruction::ClaimFeeEvent => claim_fee_event::ClaimFeeEvent,
            MeteoraDlmmInstruction::LbPairCreateEvent => lb_pair_create_event::LbPairCreateEvent,
            MeteoraDlmmInstruction::PositionCreateEvent => position_create_event::PositionCreateEvent,
            MeteoraDlmmInstruction::IncreasePositionLengthEvent => increase_position_length_event::IncreasePositionLengthEvent,
            MeteoraDlmmInstruction::DecreasePositionLengthEvent => decrease_position_length_event::DecreasePositionLengthEvent,
            MeteoraDlmmInstruction::FeeParameterUpdateEvent => fee_parameter_update_event::FeeParameterUpdateEvent,
            MeteoraDlmmInstruction::DynamicFeeParameterUpdateEvent => dynamic_fee_parameter_update_event::DynamicFeeParameterUpdateEvent,
            MeteoraDlmmInstruction::IncreaseObservationEvent => increase_observation_event::IncreaseObservationEvent,
            MeteoraDlmmInstruction::WithdrawIneligibleRewardEvent => withdraw_ineligible_reward_event::WithdrawIneligibleRewardEvent,
            MeteoraDlmmInstruction::UpdatePositionOperatorEvent => update_position_operator_event::UpdatePositionOperatorEvent,
            MeteoraDlmmInstruction::UpdatePositionLockReleasePointEvent => update_position_lock_release_point_event::UpdatePositionLockReleasePointEvent,
            MeteoraDlmmInstruction::GoToABinEvent => go_to_a_bin_event::GoToABinEvent,
        )
    }
}
