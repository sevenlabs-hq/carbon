use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw {
    pub amount: u64,
    pub minimum_amounts_out: Vec<u64>,
}

pub struct WithdrawInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_pool_token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub withdraw_authority: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub vault_authority: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program_2022: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, user_pool_token, mint, pool, withdraw_authority, vault, vault_authority, vault_program, token_program, token_program_2022, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            user: user.pubkey,
            user_pool_token: user_pool_token.pubkey,
            mint: mint.pubkey,
            pool: pool.pubkey,
            withdraw_authority: withdraw_authority.pubkey,
            vault: vault.pubkey,
            vault_authority: vault_authority.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
        })
    }
}
