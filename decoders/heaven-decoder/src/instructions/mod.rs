use crate::PROGRAM_ID;

use super::HeavenDecoder;
pub mod admin_borrow_sol;
pub mod admin_claim_msol;
pub mod admin_claim_staking_rewards;
pub mod admin_claim_standard_creator_trading_fees;
pub mod admin_deposit_msol;
pub mod admin_mint_msol;
pub mod admin_repay_sol;
pub mod admin_unstake_msol;
pub mod admin_update_standard_liquidity_pool_state;
pub mod admin_withdraw_msol;
pub mod admin_withdraw_transfer_fee;
pub mod buy;
pub mod claim_standard_creator_trading_fee_protocol_fees;
pub mod claim_standard_creator_trading_fees;
pub mod claim_standard_protocol_trading_fees;
pub mod claim_standard_reflection_trading_fees;
pub mod close_protocol_lookup_table;
pub mod create_liquidity_pool_event;
pub mod create_or_update_protocol_fee_admin;
pub mod create_or_update_protocol_owner;
pub mod create_or_update_protocol_staking_admin;
pub mod create_protocol_config;
pub mod create_protocol_lookup_table;
pub mod create_standard_liquidity_pool;
pub mod create_standard_liquidity_pool_event;
pub mod creating_liquidity_pool_event;
pub mod deactivate_protocol_lookup_table;
pub mod extend_protocol_lookup_table;
pub mod initialize_protocol_lending;
pub mod remaining_accounts_stub;
pub mod sell;
pub mod set_protocol_slot_fees;
pub mod trade_event;
pub mod update_allow_create_pool;
pub mod update_creator_trading_fee_receiver;
pub mod update_protocol_config;
pub mod user_defined_event;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
)]
pub enum HeavenInstruction {
    AdminBorrowSol(admin_borrow_sol::AdminBorrowSol),
    AdminClaimMsol(admin_claim_msol::AdminClaimMsol),
    AdminClaimStakingRewards(admin_claim_staking_rewards::AdminClaimStakingRewards),
    AdminClaimStandardCreatorTradingFees(admin_claim_standard_creator_trading_fees::AdminClaimStandardCreatorTradingFees),
    AdminDepositMsol(admin_deposit_msol::AdminDepositMsol),
    AdminMintMsol(admin_mint_msol::AdminMintMsol),
    AdminRepaySol(admin_repay_sol::AdminRepaySol),
    AdminUnstakeMsol(admin_unstake_msol::AdminUnstakeMsol),
    AdminUpdateStandardLiquidityPoolState(admin_update_standard_liquidity_pool_state::AdminUpdateStandardLiquidityPoolState),
    AdminWithdrawMsol(admin_withdraw_msol::AdminWithdrawMsol),
    AdminWithdrawTransferFee(admin_withdraw_transfer_fee::AdminWithdrawTransferFee),
    Buy(buy::Buy),
    ClaimStandardCreatorTradingFeeProtocolFees(claim_standard_creator_trading_fee_protocol_fees::ClaimStandardCreatorTradingFeeProtocolFees),
    ClaimStandardCreatorTradingFees(claim_standard_creator_trading_fees::ClaimStandardCreatorTradingFees),
    ClaimStandardProtocolTradingFees(claim_standard_protocol_trading_fees::ClaimStandardProtocolTradingFees),
    ClaimStandardReflectionTradingFees(claim_standard_reflection_trading_fees::ClaimStandardReflectionTradingFees),
    CloseProtocolLookupTable(close_protocol_lookup_table::CloseProtocolLookupTable),
    CreateOrUpdateProtocolFeeAdmin(create_or_update_protocol_fee_admin::CreateOrUpdateProtocolFeeAdmin),
    CreateOrUpdateProtocolOwner(create_or_update_protocol_owner::CreateOrUpdateProtocolOwner),
    CreateOrUpdateProtocolStakingAdmin(create_or_update_protocol_staking_admin::CreateOrUpdateProtocolStakingAdmin),
    CreateProtocolConfig(create_protocol_config::CreateProtocolConfig),
    CreateProtocolLookupTable(create_protocol_lookup_table::CreateProtocolLookupTable),
    CreateStandardLiquidityPool(create_standard_liquidity_pool::CreateStandardLiquidityPool),
    DeactivateProtocolLookupTable(deactivate_protocol_lookup_table::DeactivateProtocolLookupTable),
    ExtendProtocolLookupTable(extend_protocol_lookup_table::ExtendProtocolLookupTable),
    InitializeProtocolLending(initialize_protocol_lending::InitializeProtocolLending),
    RemainingAccountsStub(remaining_accounts_stub::RemainingAccountsStub),
    Sell(sell::Sell),
    SetProtocolSlotFees(set_protocol_slot_fees::SetProtocolSlotFees),
    UpdateAllowCreatePool(update_allow_create_pool::UpdateAllowCreatePool),
    UpdateCreatorTradingFeeReceiver(update_creator_trading_fee_receiver::UpdateCreatorTradingFeeReceiver),
    UpdateProtocolConfig(update_protocol_config::UpdateProtocolConfig),
    CreateLiquidityPoolEvent(create_liquidity_pool_event::CreateLiquidityPoolEvent),
    CreateStandardLiquidityPoolEvent(create_standard_liquidity_pool_event::CreateStandardLiquidityPoolEvent),
    CreatingLiquidityPoolEvent(creating_liquidity_pool_event::CreatingLiquidityPoolEvent),
    TradeEvent(trade_event::TradeEvent),
    UserDefinedEvent(user_defined_event::UserDefinedEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for HeavenDecoder {
    type InstructionType = HeavenInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            HeavenInstruction::AdminBorrowSol => admin_borrow_sol::AdminBorrowSol,
            HeavenInstruction::AdminClaimMsol => admin_claim_msol::AdminClaimMsol,
            HeavenInstruction::AdminClaimStakingRewards => admin_claim_staking_rewards::AdminClaimStakingRewards,
            HeavenInstruction::AdminClaimStandardCreatorTradingFees => admin_claim_standard_creator_trading_fees::AdminClaimStandardCreatorTradingFees,
            HeavenInstruction::AdminDepositMsol => admin_deposit_msol::AdminDepositMsol,
            HeavenInstruction::AdminMintMsol => admin_mint_msol::AdminMintMsol,
            HeavenInstruction::AdminRepaySol => admin_repay_sol::AdminRepaySol,
            HeavenInstruction::AdminUnstakeMsol => admin_unstake_msol::AdminUnstakeMsol,
            HeavenInstruction::AdminUpdateStandardLiquidityPoolState => admin_update_standard_liquidity_pool_state::AdminUpdateStandardLiquidityPoolState,
            HeavenInstruction::AdminWithdrawMsol => admin_withdraw_msol::AdminWithdrawMsol,
            HeavenInstruction::AdminWithdrawTransferFee => admin_withdraw_transfer_fee::AdminWithdrawTransferFee,
            HeavenInstruction::Buy => buy::Buy,
            HeavenInstruction::ClaimStandardCreatorTradingFeeProtocolFees => claim_standard_creator_trading_fee_protocol_fees::ClaimStandardCreatorTradingFeeProtocolFees,
            HeavenInstruction::ClaimStandardCreatorTradingFees => claim_standard_creator_trading_fees::ClaimStandardCreatorTradingFees,
            HeavenInstruction::ClaimStandardProtocolTradingFees => claim_standard_protocol_trading_fees::ClaimStandardProtocolTradingFees,
            HeavenInstruction::ClaimStandardReflectionTradingFees => claim_standard_reflection_trading_fees::ClaimStandardReflectionTradingFees,
            HeavenInstruction::CloseProtocolLookupTable => close_protocol_lookup_table::CloseProtocolLookupTable,
            HeavenInstruction::CreateOrUpdateProtocolFeeAdmin => create_or_update_protocol_fee_admin::CreateOrUpdateProtocolFeeAdmin,
            HeavenInstruction::CreateOrUpdateProtocolOwner => create_or_update_protocol_owner::CreateOrUpdateProtocolOwner,
            HeavenInstruction::CreateOrUpdateProtocolStakingAdmin => create_or_update_protocol_staking_admin::CreateOrUpdateProtocolStakingAdmin,
            HeavenInstruction::CreateProtocolConfig => create_protocol_config::CreateProtocolConfig,
            HeavenInstruction::CreateProtocolLookupTable => create_protocol_lookup_table::CreateProtocolLookupTable,
            HeavenInstruction::CreateStandardLiquidityPool => create_standard_liquidity_pool::CreateStandardLiquidityPool,
            HeavenInstruction::DeactivateProtocolLookupTable => deactivate_protocol_lookup_table::DeactivateProtocolLookupTable,
            HeavenInstruction::ExtendProtocolLookupTable => extend_protocol_lookup_table::ExtendProtocolLookupTable,
            HeavenInstruction::InitializeProtocolLending => initialize_protocol_lending::InitializeProtocolLending,
            HeavenInstruction::RemainingAccountsStub => remaining_accounts_stub::RemainingAccountsStub,
            HeavenInstruction::Sell => sell::Sell,
            HeavenInstruction::SetProtocolSlotFees => set_protocol_slot_fees::SetProtocolSlotFees,
            HeavenInstruction::UpdateAllowCreatePool => update_allow_create_pool::UpdateAllowCreatePool,
            HeavenInstruction::UpdateCreatorTradingFeeReceiver => update_creator_trading_fee_receiver::UpdateCreatorTradingFeeReceiver,
            HeavenInstruction::UpdateProtocolConfig => update_protocol_config::UpdateProtocolConfig,
            HeavenInstruction::CreateLiquidityPoolEvent => create_liquidity_pool_event::CreateLiquidityPoolEvent,
            HeavenInstruction::CreateStandardLiquidityPoolEvent => create_standard_liquidity_pool_event::CreateStandardLiquidityPoolEvent,
            HeavenInstruction::CreatingLiquidityPoolEvent => creating_liquidity_pool_event::CreatingLiquidityPoolEvent,
            HeavenInstruction::TradeEvent => trade_event::TradeEvent,
            HeavenInstruction::UserDefinedEvent => user_defined_event::UserDefinedEvent,
        )
    }
}
