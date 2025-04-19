use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b013016b43f76d9")]
pub struct MigrateMeteoraDamm {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateMeteoraDammInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub migration_metadata: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub damm_config: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub a_vault: solana_pubkey::Pubkey,
    pub b_vault: solana_pubkey::Pubkey,
    pub a_token_vault: solana_pubkey::Pubkey,
    pub b_token_vault: solana_pubkey::Pubkey,
    pub a_vault_lp_mint: solana_pubkey::Pubkey,
    pub b_vault_lp_mint: solana_pubkey::Pubkey,
    pub a_vault_lp: solana_pubkey::Pubkey,
    pub b_vault_lp: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub virtual_pool_lp: solana_pubkey::Pubkey,
    pub protocol_token_a_fee: solana_pubkey::Pubkey,
    pub protocol_token_b_fee: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub mint_metadata: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub amm_program: solana_pubkey::Pubkey,
    pub vault_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateMeteoraDamm {
    type ArrangedAccounts = MigrateMeteoraDammInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [virtual_pool, migration_metadata, config, pool_authority, pool, damm_config, lp_mint, token_a_mint, token_b_mint, a_vault, b_vault, a_token_vault, b_token_vault, a_vault_lp_mint, b_vault_lp_mint, a_vault_lp, b_vault_lp, base_vault, quote_vault, virtual_pool_lp, protocol_token_a_fee, protocol_token_b_fee, payer, rent, mint_metadata, metadata_program, amm_program, vault_program, token_program, associated_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateMeteoraDammInstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            migration_metadata: migration_metadata.pubkey,
            config: config.pubkey,
            pool_authority: pool_authority.pubkey,
            pool: pool.pubkey,
            damm_config: damm_config.pubkey,
            lp_mint: lp_mint.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_token_vault: a_token_vault.pubkey,
            b_token_vault: b_token_vault.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            virtual_pool_lp: virtual_pool_lp.pubkey,
            protocol_token_a_fee: protocol_token_a_fee.pubkey,
            protocol_token_b_fee: protocol_token_b_fee.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            mint_metadata: mint_metadata.pubkey,
            metadata_program: metadata_program.pubkey,
            amm_program: amm_program.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
