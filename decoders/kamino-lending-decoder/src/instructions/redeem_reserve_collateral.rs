

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xea75b57db98edc1d")]
pub struct RedeemReserveCollateral{
    pub collateral_amount: u64,
}

pub struct RedeemReserveCollateralInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_sdk::pubkey::Pubkey,
    pub user_source_collateral: solana_sdk::pubkey::Pubkey,
    pub user_destination_liquidity: solana_sdk::pubkey::Pubkey,
    pub collateral_token_program: solana_sdk::pubkey::Pubkey,
    pub liquidity_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RedeemReserveCollateral {
    type ArrangedAccounts = RedeemReserveCollateralInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let reserve = accounts.get(2)?;
        let lending_market_authority = accounts.get(3)?;
        let reserve_liquidity_mint = accounts.get(4)?;
        let reserve_collateral_mint = accounts.get(5)?;
        let reserve_liquidity_supply = accounts.get(6)?;
        let user_source_collateral = accounts.get(7)?;
        let user_destination_liquidity = accounts.get(8)?;
        let collateral_token_program = accounts.get(9)?;
        let liquidity_token_program = accounts.get(10)?;
        let instruction_sysvar_account = accounts.get(11)?;

        Some(RedeemReserveCollateralInstructionAccounts {
            owner: owner.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            user_source_collateral: user_source_collateral.pubkey,
            user_destination_liquidity: user_destination_liquidity.pubkey,
            collateral_token_program: collateral_token_program.pubkey,
            liquidity_token_program: liquidity_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}