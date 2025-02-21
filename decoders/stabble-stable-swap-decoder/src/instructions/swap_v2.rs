use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2b04ed0b1ac91e62")]
pub struct SwapV2 {
    pub amount_in: Option<u64>,
    pub minimum_amount_out: u64,
}

pub struct SwapV2InstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub mint_in: solana_sdk::pubkey::Pubkey,
    pub mint_out: solana_sdk::pubkey::Pubkey,
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
    pub token_2022_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapV2 {
    type ArrangedAccounts = SwapV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, mint_in, mint_out, user_token_in, user_token_out, vault_token_in, vault_token_out, beneficiary_token_out, pool, withdraw_authority, vault, vault_authority, vault_program, token_program, token_2022_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapV2InstructionAccounts {
            user: user.pubkey,
            mint_in: mint_in.pubkey,
            mint_out: mint_out.pubkey,
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
            token_2022_program: token_2022_program.pubkey,
        })
    }
}
