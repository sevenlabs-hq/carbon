

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2574cd67f3c05cc6")]
pub struct WithdrawObligationCollateral{
    pub collateral_amount: u64,
}

pub struct WithdrawObligationCollateralInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_source_collateral: solana_sdk::pubkey::Pubkey,
    pub user_destination_collateral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawObligationCollateral {
    type ArrangedAccounts = WithdrawObligationCollateralInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market = accounts.get(2)?;
        let lending_market_authority = accounts.get(3)?;
        let withdraw_reserve = accounts.get(4)?;
        let reserve_source_collateral = accounts.get(5)?;
        let user_destination_collateral = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let instruction_sysvar_account = accounts.get(8)?;

        Some(WithdrawObligationCollateralInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            withdraw_reserve: withdraw_reserve.pubkey,
            reserve_source_collateral: reserve_source_collateral.pubkey,
            user_destination_collateral: user_destination_collateral.pubkey,
            token_program: token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}