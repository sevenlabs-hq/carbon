use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

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
        let mut iter = accounts.iter();
        let virtual_pool = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let lock_escrow = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let source_tokens = next_account(&mut iter)?;
        let escrow_vault = next_account(&mut iter)?;
        let amm_program = next_account(&mut iter)?;
        let a_vault = next_account(&mut iter)?;
        let b_vault = next_account(&mut iter)?;
        let a_vault_lp = next_account(&mut iter)?;
        let b_vault_lp = next_account(&mut iter)?;
        let a_vault_lp_mint = next_account(&mut iter)?;
        let b_vault_lp_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(MigrateMeteoraDammLockLpTokenInstructionAccounts {
            virtual_pool,
            migration_metadata,
            pool_authority,
            pool,
            lp_mint,
            lock_escrow,
            owner,
            source_tokens,
            escrow_vault,
            amm_program,
            a_vault,
            b_vault,
            a_vault_lp,
            b_vault_lp,
            a_vault_lp_mint,
            b_vault_lp_mint,
            token_program,
        })
    }
}
