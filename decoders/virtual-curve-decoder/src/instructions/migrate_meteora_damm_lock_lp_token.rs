use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb137ee9dfb58a52a")]
pub struct MigrateMeteoraDammLockLpToken {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateMeteoraDammLockLpTokenInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub migration_metadata: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub lock_escrow: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub source_tokens: solana_pubkey::Pubkey,
    pub escrow_vault: solana_pubkey::Pubkey,
    pub amm_program: solana_pubkey::Pubkey,
    pub a_vault: solana_pubkey::Pubkey,
    pub b_vault: solana_pubkey::Pubkey,
    pub a_vault_lp: solana_pubkey::Pubkey,
    pub b_vault_lp: solana_pubkey::Pubkey,
    pub a_vault_lp_mint: solana_pubkey::Pubkey,
    pub b_vault_lp_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateMeteoraDammLockLpToken {
    type ArrangedAccounts = MigrateMeteoraDammLockLpTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [virtual_pool, migration_metadata, pool_authority, pool, lp_mint, lock_escrow, owner, source_tokens, escrow_vault, amm_program, a_vault, b_vault, a_vault_lp, b_vault_lp, a_vault_lp_mint, b_vault_lp_mint, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateMeteoraDammLockLpTokenInstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            migration_metadata: migration_metadata.pubkey,
            pool_authority: pool_authority.pubkey,
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            lock_escrow: lock_escrow.pubkey,
            owner: owner.pubkey,
            source_tokens: source_tokens.pubkey,
            escrow_vault: escrow_vault.pubkey,
            amm_program: amm_program.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
