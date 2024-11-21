

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit{
    pub max_amount: u64,
}

pub struct DepositInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub shares_mint: solana_sdk::pubkey::Pubkey,
    pub user_token_ata: solana_sdk::pubkey::Pubkey,
    pub user_shares_ata: solana_sdk::pubkey::Pubkey,
    pub klend_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub shares_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let user = accounts.get(0)?;
        let vault_state = accounts.get(1)?;
        let token_vault = accounts.get(2)?;
        let token_mint = accounts.get(3)?;
        let base_vault_authority = accounts.get(4)?;
        let shares_mint = accounts.get(5)?;
        let user_token_ata = accounts.get(6)?;
        let user_shares_ata = accounts.get(7)?;
        let klend_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let shares_token_program = accounts.get(10)?;
        let instruction_sysvar_account = accounts.get(11)?;

        Some(DepositInstructionAccounts {
            user: user.pubkey,
            vault_state: vault_state.pubkey,
            token_vault: token_vault.pubkey,
            token_mint: token_mint.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            shares_mint: shares_mint.pubkey,
            user_token_ata: user_token_ata.pubkey,
            user_shares_ata: user_shares_ata.pubkey,
            klend_program: klend_program.pubkey,
            token_program: token_program.pubkey,
            shares_token_program: shares_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}