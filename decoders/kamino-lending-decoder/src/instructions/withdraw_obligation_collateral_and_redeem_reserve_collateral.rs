

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4b5d5ddc2296dac4")]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateral{
    pub collateral_amount: u64,
}

pub struct WithdrawObligationCollateralAndRedeemReserveCollateralInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_source_collateral: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_sdk::pubkey::Pubkey,
    pub user_destination_liquidity: solana_sdk::pubkey::Pubkey,
    pub placeholder_user_destination_collateral: solana_sdk::pubkey::Pubkey,
    pub collateral_token_program: solana_sdk::pubkey::Pubkey,
    pub liquidity_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawObligationCollateralAndRedeemReserveCollateral {
    type ArrangedAccounts = WithdrawObligationCollateralAndRedeemReserveCollateralInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market = accounts.get(2)?;
        let lending_market_authority = accounts.get(3)?;
        let withdraw_reserve = accounts.get(4)?;
        let reserve_liquidity_mint = accounts.get(5)?;
        let reserve_source_collateral = accounts.get(6)?;
        let reserve_collateral_mint = accounts.get(7)?;
        let reserve_liquidity_supply = accounts.get(8)?;
        let user_destination_liquidity = accounts.get(9)?;
        let placeholder_user_destination_collateral = accounts.get(10)?;
        let collateral_token_program = accounts.get(11)?;
        let liquidity_token_program = accounts.get(12)?;
        let instruction_sysvar_account = accounts.get(13)?;

        Some(WithdrawObligationCollateralAndRedeemReserveCollateralInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            withdraw_reserve: withdraw_reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_source_collateral: reserve_source_collateral.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            user_destination_liquidity: user_destination_liquidity.pubkey,
            placeholder_user_destination_collateral: placeholder_user_destination_collateral.pubkey,
            collateral_token_program: collateral_token_program.pubkey,
            liquidity_token_program: liquidity_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}