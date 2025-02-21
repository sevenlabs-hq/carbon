use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf8c69e91e17587c8")]
pub struct Swap {
    pub amount_in: Option<u64>,
    pub minimum_amount_out: u64,
}

pub struct SwapInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_token_in: solana_sdk::pubkey::Pubkey,
    pub user_token_out: solana_sdk::pubkey::Pubkey,
    pub vault_token_in: solana_sdk::pubkey::Pubkey,
    pub vault_token_out: solana_sdk::pubkey::Pubkey,
    pub beneficiary_token_out: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub withdraw_authority: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub vault_authority: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, user_token_in, user_token_out, vault_token_in, vault_token_out, beneficiary_token_out, pool, withdraw_authority, vault, vault_authority, vault_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapInstructionAccounts {
            user: user.pubkey,
            user_token_in: user_token_in.pubkey,
            user_token_out: user_token_out.pubkey,
            vault_token_in: vault_token_in.pubkey,
            vault_token_out: vault_token_out.pubkey,
            beneficiary_token_out: beneficiary_token_out.pubkey,
            pool: pool.pubkey,
            withdraw_authority: withdraw_authority.pubkey,
            vault: vault.pubkey,
            vault_authority: vault_authority.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
