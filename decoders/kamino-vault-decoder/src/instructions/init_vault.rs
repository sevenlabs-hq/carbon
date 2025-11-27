use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4d4f559621d9346a")]
pub struct InitVault {}

pub struct InitVaultInstructionAccounts {
    pub admin_authority: solana_pubkey::Pubkey,
    pub vault_state: solana_pubkey::Pubkey,
    pub base_vault_authority: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub base_token_mint: solana_pubkey::Pubkey,
    pub shares_mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub shares_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitVault {
    type ArrangedAccounts = InitVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, vault_state, base_vault_authority, token_vault, base_token_mint, shares_mint, system_program, rent, token_program, shares_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
