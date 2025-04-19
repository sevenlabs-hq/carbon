use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ca9e66735e45040")]
pub struct MigrationDammV2 {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrationDammV2InstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub migration_metadata: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub first_position_nft_mint: solana_pubkey::Pubkey,
    pub first_position_nft_account: solana_pubkey::Pubkey,
    pub first_position: solana_pubkey::Pubkey,
    pub second_position_nft_mint: solana_pubkey::Pubkey,
    pub second_position_nft_account: solana_pubkey::Pubkey,
    pub second_position: solana_pubkey::Pubkey,
    pub damm_pool_authority: solana_pubkey::Pubkey,
    pub amm_program: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub damm_event_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrationDammV2 {
    type ArrangedAccounts = MigrationDammV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [virtual_pool, migration_metadata, config, pool_authority, pool, first_position_nft_mint, first_position_nft_account, first_position, second_position_nft_mint, second_position_nft_account, second_position, damm_pool_authority, amm_program, base_mint, quote_mint, token_a_vault, token_b_vault, base_vault, quote_vault, payer, token_base_program, token_quote_program, token_2022_program, damm_event_authority, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrationDammV2InstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            migration_metadata: migration_metadata.pubkey,
            config: config.pubkey,
            pool_authority: pool_authority.pubkey,
            pool: pool.pubkey,
            first_position_nft_mint: first_position_nft_mint.pubkey,
            first_position_nft_account: first_position_nft_account.pubkey,
            first_position: first_position.pubkey,
            second_position_nft_mint: second_position_nft_mint.pubkey,
            second_position_nft_account: second_position_nft_account.pubkey,
            second_position: second_position.pubkey,
            damm_pool_authority: damm_pool_authority.pubkey,
            amm_program: amm_program.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            payer: payer.pubkey,
            token_base_program: token_base_program.pubkey,
            token_quote_program: token_quote_program.pubkey,
            token_2022_program: token_2022_program.pubkey,
            damm_event_authority: damm_event_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
