use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x52fbe99c0c34b8ca")]
pub struct ClaimFees {}

pub struct ClaimFeesInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub vpool: solana_pubkey::Pubkey,
    pub virtuals_mint: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub vpool_virtuals_ata: solana_pubkey::Pubkey,
    pub vpool_token_ata: solana_pubkey::Pubkey,
    pub platform: solana_pubkey::Pubkey,
    pub platform_virtuals_ata: solana_pubkey::Pubkey,
    pub platform_token_ata: solana_pubkey::Pubkey,
    pub creator_virtuals_ata: solana_pubkey::Pubkey,
    pub creator_token_ata: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub lock_escrow: solana_pubkey::Pubkey,
    pub escrow_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub virtuals_vault: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub virtuals_token_vault: solana_pubkey::Pubkey,
    pub token_token_vault: solana_pubkey::Pubkey,
    pub virtuals_vault_lp_mint: solana_pubkey::Pubkey,
    pub token_vault_lp_mint: solana_pubkey::Pubkey,
    pub virtuals_vault_lp: solana_pubkey::Pubkey,
    pub token_vault_lp: solana_pubkey::Pubkey,
    pub vault_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub dynamic_amm_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimFees {
    type ArrangedAccounts = ClaimFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            payer,
            vpool,
            virtuals_mint,
            token_mint,
            vpool_virtuals_ata,
            vpool_token_ata,
            platform,
            platform_virtuals_ata,
            platform_token_ata,
            creator_virtuals_ata,
            creator_token_ata,
            pool,
            lp_mint,
            lock_escrow,
            escrow_vault,
            token_program,
            virtuals_vault,
            token_vault,
            virtuals_token_vault,
            token_token_vault,
            virtuals_vault_lp_mint,
            token_vault_lp_mint,
            virtuals_vault_lp,
            token_vault_lp,
            vault_program,
            associated_token_program,
            system_program,
            dynamic_amm_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ClaimFeesInstructionAccounts {
            payer: payer.pubkey,
            vpool: vpool.pubkey,
            virtuals_mint: virtuals_mint.pubkey,
            token_mint: token_mint.pubkey,
            vpool_virtuals_ata: vpool_virtuals_ata.pubkey,
            vpool_token_ata: vpool_token_ata.pubkey,
            platform: platform.pubkey,
            platform_virtuals_ata: platform_virtuals_ata.pubkey,
            platform_token_ata: platform_token_ata.pubkey,
            creator_virtuals_ata: creator_virtuals_ata.pubkey,
            creator_token_ata: creator_token_ata.pubkey,
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            lock_escrow: lock_escrow.pubkey,
            escrow_vault: escrow_vault.pubkey,
            token_program: token_program.pubkey,
            virtuals_vault: virtuals_vault.pubkey,
            token_vault: token_vault.pubkey,
            virtuals_token_vault: virtuals_token_vault.pubkey,
            token_token_vault: token_token_vault.pubkey,
            virtuals_vault_lp_mint: virtuals_vault_lp_mint.pubkey,
            token_vault_lp_mint: token_vault_lp_mint.pubkey,
            virtuals_vault_lp: virtuals_vault_lp.pubkey,
            token_vault_lp: token_vault_lp.pubkey,
            vault_program: vault_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            dynamic_amm_program: dynamic_amm_program.pubkey,
        })
    }
}
