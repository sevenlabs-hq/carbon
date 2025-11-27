use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub max_amount: u64,
}

pub struct DepositInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub vault_state: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub base_vault_authority: solana_pubkey::Pubkey,
    pub shares_mint: solana_pubkey::Pubkey,
    pub user_token_ata: solana_pubkey::Pubkey,
    pub user_shares_ata: solana_pubkey::Pubkey,
    pub klend_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub shares_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, vault_state, token_vault, token_mint, base_vault_authority, shares_mint, user_token_ata, user_shares_ata, klend_program, token_program, shares_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
        })
    }
}
