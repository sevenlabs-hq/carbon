use crate::PROGRAM_ID;

use super::KaminoLendingDecoder;
pub mod borrow_obligation_liquidity;
pub mod delete_referrer_state_and_short_url;
pub mod deposit_obligation_collateral;
pub mod deposit_reserve_liquidity;
pub mod deposit_reserve_liquidity_and_obligation_collateral;
pub mod flash_borrow_reserve_liquidity;
pub mod flash_repay_reserve_liquidity;
pub mod idl_missing_types;
pub mod init_farms_for_reserve;
pub mod init_lending_market;
pub mod init_obligation;
pub mod init_obligation_farms_for_reserve;
pub mod init_referrer_state_and_short_url;
pub mod init_referrer_token_state;
pub mod init_reserve;
pub mod init_user_metadata;
pub mod liquidate_obligation_and_redeem_reserve_collateral;
pub mod mark_obligation_for_deleveraging;
pub mod redeem_fees;
pub mod redeem_reserve_collateral;
pub mod refresh_obligation;
pub mod refresh_obligation_farms_for_reserve;
pub mod refresh_reserve;
pub mod refresh_reserves_batch;
pub mod repay_and_withdraw_and_redeem;
pub mod repay_obligation_liquidity;
pub mod request_elevation_group;
pub mod socialize_loss;
pub mod update_lending_market;
pub mod update_lending_market_owner;
pub mod update_reserve_config;
pub mod withdraw_obligation_collateral;
pub mod withdraw_obligation_collateral_and_redeem_reserve_collateral;
pub mod withdraw_protocol_fee;
pub mod withdraw_referrer_fees;

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
pub enum KaminoLendingInstruction {
    InitLendingMarket(init_lending_market::InitLendingMarket),
    UpdateLendingMarket(update_lending_market::UpdateLendingMarket),
    UpdateLendingMarketOwner(update_lending_market_owner::UpdateLendingMarketOwner),
    InitReserve(init_reserve::InitReserve),
    InitFarmsForReserve(init_farms_for_reserve::InitFarmsForReserve),
    UpdateReserveConfig(update_reserve_config::UpdateReserveConfig),
    RedeemFees(redeem_fees::RedeemFees),
    WithdrawProtocolFee(withdraw_protocol_fee::WithdrawProtocolFee),
    SocializeLoss(socialize_loss::SocializeLoss),
    MarkObligationForDeleveraging(mark_obligation_for_deleveraging::MarkObligationForDeleveraging),
    RefreshReserve(refresh_reserve::RefreshReserve),
    RefreshReservesBatch(refresh_reserves_batch::RefreshReservesBatch),
    DepositReserveLiquidity(deposit_reserve_liquidity::DepositReserveLiquidity),
    RedeemReserveCollateral(redeem_reserve_collateral::RedeemReserveCollateral),
    InitObligation(init_obligation::InitObligation),
    InitObligationFarmsForReserve(init_obligation_farms_for_reserve::InitObligationFarmsForReserve),
    RefreshObligationFarmsForReserve(refresh_obligation_farms_for_reserve::RefreshObligationFarmsForReserve),
    RefreshObligation(refresh_obligation::RefreshObligation),
    DepositObligationCollateral(deposit_obligation_collateral::DepositObligationCollateral),
    WithdrawObligationCollateral(withdraw_obligation_collateral::WithdrawObligationCollateral),
    BorrowObligationLiquidity(borrow_obligation_liquidity::BorrowObligationLiquidity),
    RepayObligationLiquidity(repay_obligation_liquidity::RepayObligationLiquidity),
    RepayAndWithdrawAndRedeem(repay_and_withdraw_and_redeem::RepayAndWithdrawAndRedeem),
    DepositReserveLiquidityAndObligationCollateral(deposit_reserve_liquidity_and_obligation_collateral::DepositReserveLiquidityAndObligationCollateral),
    WithdrawObligationCollateralAndRedeemReserveCollateral(withdraw_obligation_collateral_and_redeem_reserve_collateral::WithdrawObligationCollateralAndRedeemReserveCollateral),
    LiquidateObligationAndRedeemReserveCollateral(liquidate_obligation_and_redeem_reserve_collateral::LiquidateObligationAndRedeemReserveCollateral),
    FlashRepayReserveLiquidity(flash_repay_reserve_liquidity::FlashRepayReserveLiquidity),
    FlashBorrowReserveLiquidity(flash_borrow_reserve_liquidity::FlashBorrowReserveLiquidity),
    RequestElevationGroup(request_elevation_group::RequestElevationGroup),
    InitReferrerTokenState(init_referrer_token_state::InitReferrerTokenState),
    InitUserMetadata(init_user_metadata::InitUserMetadata),
    WithdrawReferrerFees(withdraw_referrer_fees::WithdrawReferrerFees),
    InitReferrerStateAndShortUrl(init_referrer_state_and_short_url::InitReferrerStateAndShortUrl),
    DeleteReferrerStateAndShortUrl(delete_referrer_state_and_short_url::DeleteReferrerStateAndShortUrl),
    IdlMissingTypes(idl_missing_types::IdlMissingTypes),
}

impl carbon_core::instruction::InstructionDecoder<'_> for KaminoLendingDecoder {
    type InstructionType = KaminoLendingInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            KaminoLendingInstruction::InitLendingMarket => init_lending_market::InitLendingMarket,
            KaminoLendingInstruction::UpdateLendingMarket => update_lending_market::UpdateLendingMarket,
            KaminoLendingInstruction::UpdateLendingMarketOwner => update_lending_market_owner::UpdateLendingMarketOwner,
            KaminoLendingInstruction::InitReserve => init_reserve::InitReserve,
            KaminoLendingInstruction::InitFarmsForReserve => init_farms_for_reserve::InitFarmsForReserve,
            KaminoLendingInstruction::UpdateReserveConfig => update_reserve_config::UpdateReserveConfig,
            KaminoLendingInstruction::RedeemFees => redeem_fees::RedeemFees,
            KaminoLendingInstruction::WithdrawProtocolFee => withdraw_protocol_fee::WithdrawProtocolFee,
            KaminoLendingInstruction::SocializeLoss => socialize_loss::SocializeLoss,
            KaminoLendingInstruction::MarkObligationForDeleveraging => mark_obligation_for_deleveraging::MarkObligationForDeleveraging,
            KaminoLendingInstruction::RefreshReserve => refresh_reserve::RefreshReserve,
            KaminoLendingInstruction::RefreshReservesBatch => refresh_reserves_batch::RefreshReservesBatch,
            KaminoLendingInstruction::DepositReserveLiquidity => deposit_reserve_liquidity::DepositReserveLiquidity,
            KaminoLendingInstruction::RedeemReserveCollateral => redeem_reserve_collateral::RedeemReserveCollateral,
            KaminoLendingInstruction::InitObligation => init_obligation::InitObligation,
            KaminoLendingInstruction::InitObligationFarmsForReserve => init_obligation_farms_for_reserve::InitObligationFarmsForReserve,
            KaminoLendingInstruction::RefreshObligationFarmsForReserve => refresh_obligation_farms_for_reserve::RefreshObligationFarmsForReserve,
            KaminoLendingInstruction::RefreshObligation => refresh_obligation::RefreshObligation,
            KaminoLendingInstruction::DepositObligationCollateral => deposit_obligation_collateral::DepositObligationCollateral,
            KaminoLendingInstruction::WithdrawObligationCollateral => withdraw_obligation_collateral::WithdrawObligationCollateral,
            KaminoLendingInstruction::BorrowObligationLiquidity => borrow_obligation_liquidity::BorrowObligationLiquidity,
            KaminoLendingInstruction::RepayObligationLiquidity => repay_obligation_liquidity::RepayObligationLiquidity,
            KaminoLendingInstruction::RepayAndWithdrawAndRedeem => repay_and_withdraw_and_redeem::RepayAndWithdrawAndRedeem,
            KaminoLendingInstruction::DepositReserveLiquidityAndObligationCollateral => deposit_reserve_liquidity_and_obligation_collateral::DepositReserveLiquidityAndObligationCollateral,
            KaminoLendingInstruction::WithdrawObligationCollateralAndRedeemReserveCollateral => withdraw_obligation_collateral_and_redeem_reserve_collateral::WithdrawObligationCollateralAndRedeemReserveCollateral,
            KaminoLendingInstruction::LiquidateObligationAndRedeemReserveCollateral => liquidate_obligation_and_redeem_reserve_collateral::LiquidateObligationAndRedeemReserveCollateral,
            KaminoLendingInstruction::FlashRepayReserveLiquidity => flash_repay_reserve_liquidity::FlashRepayReserveLiquidity,
            KaminoLendingInstruction::FlashBorrowReserveLiquidity => flash_borrow_reserve_liquidity::FlashBorrowReserveLiquidity,
            KaminoLendingInstruction::RequestElevationGroup => request_elevation_group::RequestElevationGroup,
            KaminoLendingInstruction::InitReferrerTokenState => init_referrer_token_state::InitReferrerTokenState,
            KaminoLendingInstruction::InitUserMetadata => init_user_metadata::InitUserMetadata,
            KaminoLendingInstruction::WithdrawReferrerFees => withdraw_referrer_fees::WithdrawReferrerFees,
            KaminoLendingInstruction::InitReferrerStateAndShortUrl => init_referrer_state_and_short_url::InitReferrerStateAndShortUrl,
            KaminoLendingInstruction::DeleteReferrerStateAndShortUrl => delete_referrer_state_and_short_url::DeleteReferrerStateAndShortUrl,
            KaminoLendingInstruction::IdlMissingTypes => idl_missing_types::IdlMissingTypes,
        )
    }
}
