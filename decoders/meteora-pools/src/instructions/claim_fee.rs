use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9204f8988e84689")]
pub struct ClaimFee {
    pub max_amount: u64,
}

pub struct ClaimFeeInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub lock_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub source_tokens: solana_sdk::pubkey::Pubkey,
    pub escrow_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub a_token_vault: solana_sdk::pubkey::Pubkey,
    pub b_token_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub user_a_token: solana_sdk::pubkey::Pubkey,
    pub user_b_token: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimFee {
    type ArrangedAccounts = ClaimFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, lp_mint, lock_escrow, owner, source_tokens, escrow_vault, token_program, a_token_vault, b_token_vault, a_vault, b_vault, a_vault_lp, b_vault_lp, a_vault_lp_mint, b_vault_lp_mint, user_a_token, user_b_token, vault_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimFeeInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            lock_escrow: lock_escrow.pubkey,
            owner: owner.pubkey,
            source_tokens: source_tokens.pubkey,
            escrow_vault: escrow_vault.pubkey,
            token_program: token_program.pubkey,
            a_token_vault: a_token_vault.pubkey,
            b_token_vault: b_token_vault.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            user_a_token: user_a_token.pubkey,
            user_b_token: user_b_token.pubkey,
            vault_program: vault_program.pubkey,
        })
    }
}
