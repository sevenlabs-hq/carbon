

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4d4f559621d9346a")]
pub struct InitVault{
}

pub struct InitVaultInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub base_token_mint: solana_sdk::pubkey::Pubkey,
    pub shares_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub shares_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitVault {
    type ArrangedAccounts = InitVaultInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let vault_state = accounts.get(1)?;
        let base_vault_authority = accounts.get(2)?;
        let token_vault = accounts.get(3)?;
        let base_token_mint = accounts.get(4)?;
        let shares_mint = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let shares_token_program = accounts.get(9)?;

        Some(InitVaultInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            token_vault: token_vault.pubkey,
            base_token_mint: base_token_mint.pubkey,
            shares_mint: shares_mint.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
            shares_token_program: shares_token_program.pubkey,
        })
    }
}