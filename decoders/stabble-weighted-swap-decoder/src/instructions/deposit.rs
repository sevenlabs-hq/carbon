use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub amounts: Vec<u64>,
    pub minimum_amount_out: u64,
}

pub struct DepositInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_pool_token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub pool_authority: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program_2022: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, user_pool_token, mint, pool, pool_authority, vault, vault_authority, token_program, token_program_2022, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInstructionAccounts {
            user: user.pubkey,
            user_pool_token: user_pool_token.pubkey,
            mint: mint.pubkey,
            pool: pool.pubkey,
            pool_authority: pool_authority.pubkey,
            vault: vault.pubkey,
            vault_authority: vault_authority.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
        })
    }
}
