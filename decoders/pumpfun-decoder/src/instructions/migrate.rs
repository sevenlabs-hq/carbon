use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9beae792ec9ea21e")]
pub struct Migrate {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateInstructionAccounts {
    pub global: solana_pubkey::Pubkey,
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub associated_bonding_curve: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub pump_amm: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool_authority_mint_account: solana_pubkey::Pubkey,
    pub pool_authority_wsol_account: solana_pubkey::Pubkey,
    pub amm_global_config: solana_pubkey::Pubkey,
    pub wsol_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub user_pool_token_account: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub pump_amm_event_authority: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Migrate {
    type ArrangedAccounts = MigrateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global, withdraw_authority, mint, bonding_curve, associated_bonding_curve, user, system_program, token_program, pump_amm, pool, pool_authority, pool_authority_mint_account, pool_authority_wsol_account, amm_global_config, wsol_mint, lp_mint, user_pool_token_account, pool_base_token_account, pool_quote_token_account, token_2022_program, associated_token_program, pump_amm_event_authority, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateInstructionAccounts {
            global: global.pubkey,
            withdraw_authority: withdraw_authority.pubkey,
            mint: mint.pubkey,
            bonding_curve: bonding_curve.pubkey,
            associated_bonding_curve: associated_bonding_curve.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            pump_amm: pump_amm.pubkey,
            pool: pool.pubkey,
            pool_authority: pool_authority.pubkey,
            pool_authority_mint_account: pool_authority_mint_account.pubkey,
            pool_authority_wsol_account: pool_authority_wsol_account.pubkey,
            amm_global_config: amm_global_config.pubkey,
            wsol_mint: wsol_mint.pubkey,
            lp_mint: lp_mint.pubkey,
            user_pool_token_account: user_pool_token_account.pubkey,
            pool_base_token_account: pool_base_token_account.pubkey,
            pool_quote_token_account: pool_quote_token_account.pubkey,
            token_2022_program: token_2022_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            pump_amm_event_authority: pump_amm_event_authority.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
