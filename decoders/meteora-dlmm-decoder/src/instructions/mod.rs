use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use super::LbClmmDecoder;
pub mod add_liquidity;
pub mod add_liquidity_by_strategy;
pub mod add_liquidity_by_strategy_one_side;
pub mod add_liquidity_by_weight;
pub mod add_liquidity_one_side;
pub mod add_liquidity_one_side_precise;
pub mod claim_fee;
pub mod claim_reward;
pub mod close_position;
pub mod close_preset_parameter;
pub mod composition_fee;
pub mod fee_parameter_update;
pub mod fund_reward;
pub mod go_to_a_bin;
pub mod increase_observation;
pub mod increase_oracle_length;
pub mod initialize_bin_array;
pub mod initialize_bin_array_bitmap_extension;
pub mod initialize_lb_pair;
pub mod initialize_permission_lb_pair;
pub mod initialize_position;
pub mod initialize_position_by_operator;
pub mod initialize_position_pda;
pub mod initialize_preset_parameter;
pub mod initialize_reward;
pub mod lb_pair_create;
pub mod migrate_bin_array;
pub mod migrate_position;
pub mod position_close;
pub mod position_create;
pub mod remove_all_liquidity;
pub mod remove_liquidity;
pub mod remove_liquidity_by_range;
pub mod remove_liquidity_single_side;
pub mod set_activation_point;
pub mod set_lock_release_point;
pub mod set_pre_activation_duration;
pub mod set_pre_activation_swap_address;
pub mod swap;
pub mod swap_exact_out;
pub mod swap_with_price_impact;
pub mod toggle_pair_status;
pub mod update_fee_parameters;
pub mod update_fees_and_rewards;
pub mod update_position_lock_release_point;
pub mod update_position_operator;
pub mod update_reward_duration;
pub mod update_reward_funder;
pub mod update_whitelisted_wallet;
pub mod withdraw_ineligible_reward;
pub mod withdraw_protocol_fee;

#[derive(
    carbon_proc_macros::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum LbClmmInstruction {
    InitializeLbPair(initialize_lb_pair::InitializeLbPair),
    InitializePermissionLbPair(initialize_permission_lb_pair::InitializePermissionLbPair),
    InitializeBinArrayBitmapExtension(
        initialize_bin_array_bitmap_extension::InitializeBinArrayBitmapExtension,
    ),
    InitializeBinArray(initialize_bin_array::InitializeBinArray),
    AddLiquidity(add_liquidity::AddLiquidity),
    AddLiquidityByWeight(add_liquidity_by_weight::AddLiquidityByWeight),
    AddLiquidityByStrategy(add_liquidity_by_strategy::AddLiquidityByStrategy),
    AddLiquidityByStrategyOneSide(
        add_liquidity_by_strategy_one_side::AddLiquidityByStrategyOneSide,
    ),
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
    UpdateFeeParameters(update_fee_parameters::UpdateFeeParameters),
    IncreaseOracleLength(increase_oracle_length::IncreaseOracleLength),
    InitializePresetParameter(initialize_preset_parameter::InitializePresetParameter),
    ClosePresetParameter(close_preset_parameter::ClosePresetParameter),
    RemoveAllLiquidity(remove_all_liquidity::RemoveAllLiquidity),
    RemoveLiquiditySingleSide(remove_liquidity_single_side::RemoveLiquiditySingleSide),
    TogglePairStatus(toggle_pair_status::TogglePairStatus),
    UpdateWhitelistedWallet(update_whitelisted_wallet::UpdateWhitelistedWallet),
    MigratePosition(migrate_position::MigratePosition),
    MigrateBinArray(migrate_bin_array::MigrateBinArray),
    UpdateFeesAndRewards(update_fees_and_rewards::UpdateFeesAndRewards),
    WithdrawIneligibleReward(withdraw_ineligible_reward::WithdrawIneligibleReward),
    SetActivationPoint(set_activation_point::SetActivationPoint),
    SetLockReleasePoint(set_lock_release_point::SetLockReleasePoint),
    RemoveLiquidityByRange(remove_liquidity_by_range::RemoveLiquidityByRange),
    AddLiquidityOneSidePrecise(add_liquidity_one_side_precise::AddLiquidityOneSidePrecise),
    GoToABin(go_to_a_bin::GoToABin),
    SetPreActivationDuration(set_pre_activation_duration::SetPreActivationDuration),
    SetPreActivationSwapAddress(set_pre_activation_swap_address::SetPreActivationSwapAddress),
    CompositionFee(composition_fee::CompositionFee),
    PositionClose(position_close::PositionClose),
    LbPairCreate(lb_pair_create::LbPairCreate),
    PositionCreate(position_create::PositionCreate),
    FeeParameterUpdate(fee_parameter_update::FeeParameterUpdate),
    IncreaseObservation(increase_observation::IncreaseObservation),
    UpdatePositionLockReleasePoint(
        update_position_lock_release_point::UpdatePositionLockReleasePoint,
    ),
}

impl InstructionDecoder for LbClmmDecoder {
    type InstructionType = LbClmmInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            initialize_lb_pair::InitializeLbPair::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializeLbPair(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_permission_lb_pair::InitializePermissionLbPair::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializePermissionLbPair(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_bin_array_bitmap_extension::InitializeBinArrayBitmapExtension::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializeBinArrayBitmapExtension(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_bin_array::InitializeBinArray::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializeBinArray(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            add_liquidity::AddLiquidity::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::AddLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            add_liquidity_by_weight::AddLiquidityByWeight::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::AddLiquidityByWeight(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            add_liquidity_by_strategy::AddLiquidityByStrategy::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::AddLiquidityByStrategy(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            add_liquidity_by_strategy_one_side::AddLiquidityByStrategyOneSide::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::AddLiquidityByStrategyOneSide(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            add_liquidity_one_side::AddLiquidityOneSide::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::AddLiquidityOneSide(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            remove_liquidity::RemoveLiquidity::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::RemoveLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_position::InitializePosition::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializePosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_position_pda::InitializePositionPda::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializePositionPda(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_position_by_operator::InitializePositionByOperator::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializePositionByOperator(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_position_operator::UpdatePositionOperator::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdatePositionOperator(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap::Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            swap_exact_out::SwapExactOut::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::SwapExactOut(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            swap_with_price_impact::SwapWithPriceImpact::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::SwapWithPriceImpact(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw_protocol_fee::WithdrawProtocolFee::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::WithdrawProtocolFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_reward::InitializeReward::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializeReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            fund_reward::FundReward::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::FundReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_reward_funder::UpdateRewardFunder::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdateRewardFunder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_reward_duration::UpdateRewardDuration::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdateRewardDuration(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            claim_reward::ClaimReward::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::ClaimReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            claim_fee::ClaimFee::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::ClaimFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            close_position::ClosePosition::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::ClosePosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_fee_parameters::UpdateFeeParameters::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdateFeeParameters(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            increase_oracle_length::IncreaseOracleLength::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::IncreaseOracleLength(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_preset_parameter::InitializePresetParameter::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializePresetParameter(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            close_preset_parameter::ClosePresetParameter::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::ClosePresetParameter(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            remove_all_liquidity::RemoveAllLiquidity::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::RemoveAllLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            remove_liquidity_single_side::RemoveLiquiditySingleSide::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::RemoveLiquiditySingleSide(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            toggle_pair_status::TogglePairStatus::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::TogglePairStatus(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_whitelisted_wallet::UpdateWhitelistedWallet::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdateWhitelistedWallet(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            migrate_position::MigratePosition::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::MigratePosition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            migrate_bin_array::MigrateBinArray::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::MigrateBinArray(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_fees_and_rewards::UpdateFeesAndRewards::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdateFeesAndRewards(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw_ineligible_reward::WithdrawIneligibleReward::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::WithdrawIneligibleReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_activation_point::SetActivationPoint::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::SetActivationPoint(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_lock_release_point::SetLockReleasePoint::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::SetLockReleasePoint(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            remove_liquidity_by_range::RemoveLiquidityByRange::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::RemoveLiquidityByRange(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            add_liquidity_one_side_precise::AddLiquidityOneSidePrecise::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::AddLiquidityOneSidePrecise(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            go_to_a_bin::GoToABin::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::GoToABin(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_pre_activation_duration::SetPreActivationDuration::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::SetPreActivationDuration(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_pre_activation_swap_address::SetPreActivationSwapAddress::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::SetPreActivationSwapAddress(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            composition_fee::CompositionFee::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::CompositionFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            add_liquidity::AddLiquidity::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::AddLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            remove_liquidity::RemoveLiquidity::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::RemoveLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap::Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            claim_reward::ClaimReward::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::ClaimReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            fund_reward::FundReward::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::FundReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_reward::InitializeReward::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::InitializeReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_reward_duration::UpdateRewardDuration::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdateRewardDuration(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_reward_funder::UpdateRewardFunder::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdateRewardFunder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            position_close::PositionClose::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::PositionClose(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            claim_fee::ClaimFee::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::ClaimFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            lb_pair_create::LbPairCreate::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::LbPairCreate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            position_create::PositionCreate::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::PositionCreate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            fee_parameter_update::FeeParameterUpdate::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::FeeParameterUpdate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            increase_observation::IncreaseObservation::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::IncreaseObservation(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw_ineligible_reward::WithdrawIneligibleReward::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::WithdrawIneligibleReward(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_position_operator::UpdatePositionOperator::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdatePositionOperator(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_position_lock_release_point::UpdatePositionLockReleasePoint::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::UpdatePositionLockReleasePoint(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            go_to_a_bin::GoToABin::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LbClmmInstruction::GoToABin(decoded_instruction),
            });
        }

        None
    }
}
