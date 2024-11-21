

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb1479abce2854a37")]
pub struct LiquidateObligationAndRedeemReserveCollateral{
    pub liquidity_amount: u64,
    pub min_acceptable_received_liquidity_amount: u64,
    pub max_allowed_ltv_override_percent: u64,
}

pub struct LiquidateObligationAndRedeemReserveCollateralInstructionAccounts {
    pub liquidator: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub repay_reserve: solana_sdk::pubkey::Pubkey,
    pub repay_reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub repay_reserve_liquidity_supply: solana_sdk::pubkey::Pubkey,
    pub withdraw_reserve: solana_sdk::pubkey::Pubkey,
    pub withdraw_reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub withdraw_reserve_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub withdraw_reserve_collateral_supply: solana_sdk::pubkey::Pubkey,
    pub withdraw_reserve_liquidity_supply: solana_sdk::pubkey::Pubkey,
    pub withdraw_reserve_liquidity_fee_receiver: solana_sdk::pubkey::Pubkey,
    pub user_source_liquidity: solana_sdk::pubkey::Pubkey,
    pub user_destination_collateral: solana_sdk::pubkey::Pubkey,
    pub user_destination_liquidity: solana_sdk::pubkey::Pubkey,
    pub collateral_token_program: solana_sdk::pubkey::Pubkey,
    pub repay_liquidity_token_program: solana_sdk::pubkey::Pubkey,
    pub withdraw_liquidity_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidateObligationAndRedeemReserveCollateral {
    type ArrangedAccounts = LiquidateObligationAndRedeemReserveCollateralInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let liquidator = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market = accounts.get(2)?;
        let lending_market_authority = accounts.get(3)?;
        let repay_reserve = accounts.get(4)?;
        let repay_reserve_liquidity_mint = accounts.get(5)?;
        let repay_reserve_liquidity_supply = accounts.get(6)?;
        let withdraw_reserve = accounts.get(7)?;
        let withdraw_reserve_liquidity_mint = accounts.get(8)?;
        let withdraw_reserve_collateral_mint = accounts.get(9)?;
        let withdraw_reserve_collateral_supply = accounts.get(10)?;
        let withdraw_reserve_liquidity_supply = accounts.get(11)?;
        let withdraw_reserve_liquidity_fee_receiver = accounts.get(12)?;
        let user_source_liquidity = accounts.get(13)?;
        let user_destination_collateral = accounts.get(14)?;
        let user_destination_liquidity = accounts.get(15)?;
        let collateral_token_program = accounts.get(16)?;
        let repay_liquidity_token_program = accounts.get(17)?;
        let withdraw_liquidity_token_program = accounts.get(18)?;
        let instruction_sysvar_account = accounts.get(19)?;

        Some(LiquidateObligationAndRedeemReserveCollateralInstructionAccounts {
            liquidator: liquidator.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            repay_reserve: repay_reserve.pubkey,
            repay_reserve_liquidity_mint: repay_reserve_liquidity_mint.pubkey,
            repay_reserve_liquidity_supply: repay_reserve_liquidity_supply.pubkey,
            withdraw_reserve: withdraw_reserve.pubkey,
            withdraw_reserve_liquidity_mint: withdraw_reserve_liquidity_mint.pubkey,
            withdraw_reserve_collateral_mint: withdraw_reserve_collateral_mint.pubkey,
            withdraw_reserve_collateral_supply: withdraw_reserve_collateral_supply.pubkey,
            withdraw_reserve_liquidity_supply: withdraw_reserve_liquidity_supply.pubkey,
            withdraw_reserve_liquidity_fee_receiver: withdraw_reserve_liquidity_fee_receiver.pubkey,
            user_source_liquidity: user_source_liquidity.pubkey,
            user_destination_collateral: user_destination_collateral.pubkey,
            user_destination_liquidity: user_destination_liquidity.pubkey,
            collateral_token_program: collateral_token_program.pubkey,
            repay_liquidity_token_program: repay_liquidity_token_program.pubkey,
            withdraw_liquidity_token_program: withdraw_liquidity_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}