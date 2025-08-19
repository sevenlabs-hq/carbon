use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x068724e82327fa47")]
pub struct InitializeZetaGroup {
    pub args: InitializeZetaGroupArgs,
}

pub struct InitializeZetaGroupInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub underlying_mint: solana_pubkey::Pubkey,
    pub zeta_program: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub perp_sync_queue: solana_pubkey::Pubkey,
    pub underlying: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub socialized_loss_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub usdc_mint: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeZetaGroup {
    type ArrangedAccounts = InitializeZetaGroupInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, system_program, underlying_mint, zeta_program, oracle, oracle_backup_feed, oracle_backup_program, zeta_group, greeks, perp_sync_queue, underlying, vault, insurance_vault, socialized_loss_account, token_program, usdc_mint, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeZetaGroupInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
            underlying_mint: underlying_mint.pubkey,
            zeta_program: zeta_program.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            zeta_group: zeta_group.pubkey,
            greeks: greeks.pubkey,
            perp_sync_queue: perp_sync_queue.pubkey,
            underlying: underlying.pubkey,
            vault: vault.pubkey,
            insurance_vault: insurance_vault.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
            token_program: token_program.pubkey,
            usdc_mint: usdc_mint.pubkey,
            rent: rent.pubkey,
        })
    }
}
