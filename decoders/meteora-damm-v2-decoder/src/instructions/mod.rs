use super::CpAmmDecoder;
pub mod add_liquidity;
pub mod claim_partner_fee;
pub mod claim_position_fee;
pub mod claim_protocol_fee;
pub mod claim_reward;
pub mod close_claim_fee_operator;
pub mod close_config;
pub mod close_position;
pub mod create_claim_fee_operator;
pub mod create_config;
pub mod create_dynamic_config;
pub mod create_position;
pub mod create_token_badge;
pub mod evt_add_liquidity_event;
pub mod evt_claim_partner_fee_event;
pub mod evt_claim_position_fee_event;
pub mod evt_claim_protocol_fee_event;
pub mod evt_claim_reward_event;
pub mod evt_close_claim_fee_operator_event;
pub mod evt_close_config_event;
pub mod evt_close_position_event;
pub mod evt_create_claim_fee_operator_event;
pub mod evt_create_config_event;
pub mod evt_create_dynamic_config_event;
pub mod evt_create_position_event;
pub mod evt_create_token_badge_event;
pub mod evt_fund_reward_event;
pub mod evt_initialize_pool_event;
pub mod evt_initialize_reward_event;
pub mod evt_lock_position_event;
pub mod evt_permanent_lock_position_event;
pub mod evt_remove_liquidity_event;
pub mod evt_set_pool_status_event;
pub mod evt_swap_event;
pub mod evt_update_reward_duration_event;
pub mod evt_update_reward_funder_event;
pub mod evt_withdraw_ineligible_reward_event;
pub mod fund_reward;
pub mod initialize_customizable_pool;
pub mod initialize_pool;
pub mod initialize_pool_with_dynamic_config;
pub mod initialize_reward;
pub mod lock_position;
pub mod permanent_lock_position;
pub mod refresh_vesting;
pub mod remove_all_liquidity;
pub mod remove_liquidity;
pub mod set_pool_status;
pub mod swap;
pub mod update_reward_duration;
pub mod update_reward_funder;
pub mod withdraw_ineligible_reward;

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
pub enum CpAmmInstruction {
    AddLiquidity(add_liquidity::AddLiquidity),
    ClaimPartnerFee(claim_partner_fee::ClaimPartnerFee),
    ClaimPositionFee(claim_position_fee::ClaimPositionFee),
    ClaimProtocolFee(claim_protocol_fee::ClaimProtocolFee),
    ClaimReward(claim_reward::ClaimReward),
    CloseClaimFeeOperator(close_claim_fee_operator::CloseClaimFeeOperator),
    CloseConfig(close_config::CloseConfig),
    ClosePosition(close_position::ClosePosition),
    CreateClaimFeeOperator(create_claim_fee_operator::CreateClaimFeeOperator),
    CreateConfig(create_config::CreateConfig),
    CreateDynamicConfig(create_dynamic_config::CreateDynamicConfig),
    CreatePosition(create_position::CreatePosition),
    CreateTokenBadge(create_token_badge::CreateTokenBadge),
    FundReward(fund_reward::FundReward),
    InitializeCustomizablePool(initialize_customizable_pool::InitializeCustomizablePool),
    InitializePool(initialize_pool::InitializePool),
    InitializePoolWithDynamicConfig(
        initialize_pool_with_dynamic_config::InitializePoolWithDynamicConfig,
    ),
    InitializeReward(initialize_reward::InitializeReward),
    LockPosition(lock_position::LockPosition),
    PermanentLockPosition(permanent_lock_position::PermanentLockPosition),
    RefreshVesting(refresh_vesting::RefreshVesting),
    RemoveAllLiquidity(remove_all_liquidity::RemoveAllLiquidity),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    SetPoolStatus(set_pool_status::SetPoolStatus),
    Swap(swap::Swap),
    UpdateRewardDuration(update_reward_duration::UpdateRewardDuration),
    UpdateRewardFunder(update_reward_funder::UpdateRewardFunder),
    WithdrawIneligibleReward(withdraw_ineligible_reward::WithdrawIneligibleReward),
    EvtAddLiquidityEvent(evt_add_liquidity_event::EvtAddLiquidityEvent),
    EvtClaimPartnerFeeEvent(evt_claim_partner_fee_event::EvtClaimPartnerFeeEvent),
    EvtClaimPositionFeeEvent(evt_claim_position_fee_event::EvtClaimPositionFeeEvent),
    EvtClaimProtocolFeeEvent(evt_claim_protocol_fee_event::EvtClaimProtocolFeeEvent),
    EvtClaimRewardEvent(evt_claim_reward_event::EvtClaimRewardEvent),
    EvtCloseClaimFeeOperatorEvent(
        evt_close_claim_fee_operator_event::EvtCloseClaimFeeOperatorEvent,
    ),
    EvtCloseConfigEvent(evt_close_config_event::EvtCloseConfigEvent),
    EvtClosePositionEvent(evt_close_position_event::EvtClosePositionEvent),
    EvtCreateClaimFeeOperatorEvent(
        evt_create_claim_fee_operator_event::EvtCreateClaimFeeOperatorEvent,
    ),
    EvtCreateConfigEvent(evt_create_config_event::EvtCreateConfigEvent),
    EvtCreateDynamicConfigEvent(evt_create_dynamic_config_event::EvtCreateDynamicConfigEvent),
    EvtCreatePositionEvent(evt_create_position_event::EvtCreatePositionEvent),
    EvtCreateTokenBadgeEvent(evt_create_token_badge_event::EvtCreateTokenBadgeEvent),
    EvtFundRewardEvent(evt_fund_reward_event::EvtFundRewardEvent),
    EvtInitializePoolEvent(evt_initialize_pool_event::EvtInitializePoolEvent),
    EvtInitializeRewardEvent(evt_initialize_reward_event::EvtInitializeRewardEvent),
    EvtLockPositionEvent(evt_lock_position_event::EvtLockPositionEvent),
    EvtPermanentLockPositionEvent(evt_permanent_lock_position_event::EvtPermanentLockPositionEvent),
    EvtRemoveLiquidityEvent(evt_remove_liquidity_event::EvtRemoveLiquidityEvent),
    EvtSetPoolStatusEvent(evt_set_pool_status_event::EvtSetPoolStatusEvent),
    EvtSwapEvent(evt_swap_event::EvtSwapEvent),
    EvtUpdateRewardDurationEvent(evt_update_reward_duration_event::EvtUpdateRewardDurationEvent),
    EvtUpdateRewardFunderEvent(evt_update_reward_funder_event::EvtUpdateRewardFunderEvent),
    EvtWithdrawIneligibleRewardEvent(
        evt_withdraw_ineligible_reward_event::EvtWithdrawIneligibleRewardEvent,
    ),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for CpAmmDecoder {
    type InstructionType = CpAmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            CpAmmInstruction::AddLiquidity => add_liquidity::AddLiquidity,
            CpAmmInstruction::ClaimPartnerFee => claim_partner_fee::ClaimPartnerFee,
            CpAmmInstruction::ClaimPositionFee => claim_position_fee::ClaimPositionFee,
            CpAmmInstruction::ClaimProtocolFee => claim_protocol_fee::ClaimProtocolFee,
            CpAmmInstruction::ClaimReward => claim_reward::ClaimReward,
            CpAmmInstruction::CloseClaimFeeOperator => close_claim_fee_operator::CloseClaimFeeOperator,
            CpAmmInstruction::CloseConfig => close_config::CloseConfig,
            CpAmmInstruction::ClosePosition => close_position::ClosePosition,
            CpAmmInstruction::CreateClaimFeeOperator => create_claim_fee_operator::CreateClaimFeeOperator,
            CpAmmInstruction::CreateConfig => create_config::CreateConfig,
            CpAmmInstruction::CreateDynamicConfig => create_dynamic_config::CreateDynamicConfig,
            CpAmmInstruction::CreatePosition => create_position::CreatePosition,
            CpAmmInstruction::CreateTokenBadge => create_token_badge::CreateTokenBadge,
            CpAmmInstruction::FundReward => fund_reward::FundReward,
            CpAmmInstruction::InitializeCustomizablePool => initialize_customizable_pool::InitializeCustomizablePool,
            CpAmmInstruction::InitializePool => initialize_pool::InitializePool,
            CpAmmInstruction::InitializePoolWithDynamicConfig => initialize_pool_with_dynamic_config::InitializePoolWithDynamicConfig,
            CpAmmInstruction::InitializeReward => initialize_reward::InitializeReward,
            CpAmmInstruction::LockPosition => lock_position::LockPosition,
            CpAmmInstruction::PermanentLockPosition => permanent_lock_position::PermanentLockPosition,
            CpAmmInstruction::RefreshVesting => refresh_vesting::RefreshVesting,
            CpAmmInstruction::RemoveAllLiquidity => remove_all_liquidity::RemoveAllLiquidity,
            CpAmmInstruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            CpAmmInstruction::SetPoolStatus => set_pool_status::SetPoolStatus,
            CpAmmInstruction::Swap => swap::Swap,
            CpAmmInstruction::UpdateRewardDuration => update_reward_duration::UpdateRewardDuration,
            CpAmmInstruction::UpdateRewardFunder => update_reward_funder::UpdateRewardFunder,
            CpAmmInstruction::WithdrawIneligibleReward => withdraw_ineligible_reward::WithdrawIneligibleReward,
            CpAmmInstruction::EvtAddLiquidityEvent => evt_add_liquidity_event::EvtAddLiquidityEvent,
            CpAmmInstruction::EvtClaimPartnerFeeEvent => evt_claim_partner_fee_event::EvtClaimPartnerFeeEvent,
            CpAmmInstruction::EvtClaimPositionFeeEvent => evt_claim_position_fee_event::EvtClaimPositionFeeEvent,
            CpAmmInstruction::EvtClaimProtocolFeeEvent => evt_claim_protocol_fee_event::EvtClaimProtocolFeeEvent,
            CpAmmInstruction::EvtClaimRewardEvent => evt_claim_reward_event::EvtClaimRewardEvent,
            CpAmmInstruction::EvtCloseClaimFeeOperatorEvent => evt_close_claim_fee_operator_event::EvtCloseClaimFeeOperatorEvent,
            CpAmmInstruction::EvtCloseConfigEvent => evt_close_config_event::EvtCloseConfigEvent,
            CpAmmInstruction::EvtClosePositionEvent => evt_close_position_event::EvtClosePositionEvent,
            CpAmmInstruction::EvtCreateClaimFeeOperatorEvent => evt_create_claim_fee_operator_event::EvtCreateClaimFeeOperatorEvent,
            CpAmmInstruction::EvtCreateConfigEvent => evt_create_config_event::EvtCreateConfigEvent,
            CpAmmInstruction::EvtCreateDynamicConfigEvent => evt_create_dynamic_config_event::EvtCreateDynamicConfigEvent,
            CpAmmInstruction::EvtCreatePositionEvent => evt_create_position_event::EvtCreatePositionEvent,
            CpAmmInstruction::EvtCreateTokenBadgeEvent => evt_create_token_badge_event::EvtCreateTokenBadgeEvent,
            CpAmmInstruction::EvtFundRewardEvent => evt_fund_reward_event::EvtFundRewardEvent,
            CpAmmInstruction::EvtInitializePoolEvent => evt_initialize_pool_event::EvtInitializePoolEvent,
            CpAmmInstruction::EvtInitializeRewardEvent => evt_initialize_reward_event::EvtInitializeRewardEvent,
            CpAmmInstruction::EvtLockPositionEvent => evt_lock_position_event::EvtLockPositionEvent,
            CpAmmInstruction::EvtPermanentLockPositionEvent => evt_permanent_lock_position_event::EvtPermanentLockPositionEvent,
            CpAmmInstruction::EvtRemoveLiquidityEvent => evt_remove_liquidity_event::EvtRemoveLiquidityEvent,
            CpAmmInstruction::EvtSetPoolStatusEvent => evt_set_pool_status_event::EvtSetPoolStatusEvent,
            CpAmmInstruction::EvtSwapEvent => evt_swap_event::EvtSwapEvent,
            CpAmmInstruction::EvtUpdateRewardDurationEvent => evt_update_reward_duration_event::EvtUpdateRewardDurationEvent,
            CpAmmInstruction::EvtUpdateRewardFunderEvent => evt_update_reward_funder_event::EvtUpdateRewardFunderEvent,
            CpAmmInstruction::EvtWithdrawIneligibleRewardEvent => evt_withdraw_ineligible_reward_event::EvtWithdrawIneligibleRewardEvent,
        )
    }
}
