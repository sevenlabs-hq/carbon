

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x83c2c88caff4d9b7")]
pub struct WithdrawPendingFees{
}

pub struct WithdrawPendingFeesInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub ctoken_vault: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_ata: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub klend_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawPendingFees {
    type ArrangedAccounts = WithdrawPendingFeesInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let vault_state = accounts.get(1)?;
        let reserve = accounts.get(2)?;
        let token_vault = accounts.get(3)?;
        let ctoken_vault = accounts.get(4)?;
        let base_vault_authority = accounts.get(5)?;
        let token_ata = accounts.get(6)?;
        let token_mint = accounts.get(7)?;
        let lending_market = accounts.get(8)?;
        let lending_market_authority = accounts.get(9)?;
        let reserve_liquidity_supply = accounts.get(10)?;
        let reserve_collateral_mint = accounts.get(11)?;
        let klend_program = accounts.get(12)?;
        let token_program = accounts.get(13)?;
        let reserve_collateral_token_program = accounts.get(14)?;
        let instruction_sysvar_account = accounts.get(15)?;

        Some(WithdrawPendingFeesInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            reserve: reserve.pubkey,
            token_vault: token_vault.pubkey,
            ctoken_vault: ctoken_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            token_ata: token_ata.pubkey,
            token_mint: token_mint.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            klend_program: klend_program.pubkey,
            token_program: token_program.pubkey,
            reserve_collateral_token_program: reserve_collateral_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}