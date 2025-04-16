use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x885cc8671cda908c")]
pub struct MigrateToCpswap {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateToCpswapInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub cpswap_program: solana_pubkey::Pubkey,
    pub cpswap_pool: solana_pubkey::Pubkey,
    pub cpswap_authority: solana_pubkey::Pubkey,
    pub cpswap_lp_mint: solana_pubkey::Pubkey,
    pub cpswap_base_vault: solana_pubkey::Pubkey,
    pub cpswap_quote_vault: solana_pubkey::Pubkey,
    pub cpswap_config: solana_pubkey::Pubkey,
    pub cpswap_create_pool_fee: solana_pubkey::Pubkey,
    pub cpswap_observation: solana_pubkey::Pubkey,
    pub lock_program: solana_pubkey::Pubkey,
    pub lock_authority: solana_pubkey::Pubkey,
    pub lock_lp_vault: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub pool_lp_token: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateToCpswap {
    type ArrangedAccounts = MigrateToCpswapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, base_mint, quote_mint, platform_config, cpswap_program, cpswap_pool, cpswap_authority, cpswap_lp_mint, cpswap_base_vault, cpswap_quote_vault, cpswap_config, cpswap_create_pool_fee, cpswap_observation, lock_program, lock_authority, lock_lp_vault, authority, pool_state, global_config, base_vault, quote_vault, pool_lp_token, base_token_program, quote_token_program, associated_token_program, system_program, rent_program, metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateToCpswapInstructionAccounts {
            payer: payer.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            platform_config: platform_config.pubkey,
            cpswap_program: cpswap_program.pubkey,
            cpswap_pool: cpswap_pool.pubkey,
            cpswap_authority: cpswap_authority.pubkey,
            cpswap_lp_mint: cpswap_lp_mint.pubkey,
            cpswap_base_vault: cpswap_base_vault.pubkey,
            cpswap_quote_vault: cpswap_quote_vault.pubkey,
            cpswap_config: cpswap_config.pubkey,
            cpswap_create_pool_fee: cpswap_create_pool_fee.pubkey,
            cpswap_observation: cpswap_observation.pubkey,
            lock_program: lock_program.pubkey,
            lock_authority: lock_authority.pubkey,
            lock_lp_vault: lock_lp_vault.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            global_config: global_config.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            pool_lp_token: pool_lp_token.pubkey,
            base_token_program: base_token_program.pubkey,
            quote_token_program: quote_token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            rent_program: rent_program.pubkey,
            metadata_program: metadata_program.pubkey,
        })
    }
}
