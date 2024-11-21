

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0df5b467feb67904")]
pub struct Invest{
}

pub struct InvestInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub payer_token_account: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub ctoken_vault: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub klend_program: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_token_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Invest {
    type ArrangedAccounts = InvestInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let payer_token_account = accounts.get(1)?;
        let vault_state = accounts.get(2)?;
        let token_vault = accounts.get(3)?;
        let token_mint = accounts.get(4)?;
        let base_vault_authority = accounts.get(5)?;
        let ctoken_vault = accounts.get(6)?;
        let reserve = accounts.get(7)?;
        let lending_market = accounts.get(8)?;
        let lending_market_authority = accounts.get(9)?;
        let reserve_liquidity_supply = accounts.get(10)?;
        let reserve_collateral_mint = accounts.get(11)?;
        let klend_program = accounts.get(12)?;
        let reserve_collateral_token_program = accounts.get(13)?;
        let token_program = accounts.get(14)?;
        let instruction_sysvar_account = accounts.get(15)?;

        Some(InvestInstructionAccounts {
            payer: payer.pubkey,
            payer_token_account: payer_token_account.pubkey,
            vault_state: vault_state.pubkey,
            token_vault: token_vault.pubkey,
            token_mint: token_mint.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            ctoken_vault: ctoken_vault.pubkey,
            reserve: reserve.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            klend_program: klend_program.pubkey,
            reserve_collateral_token_program: reserve_collateral_token_program.pubkey,
            token_program: token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}