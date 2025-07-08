use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf6fe2125e1b029e8")]
pub struct CreateMeteoraPool {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateMeteoraPoolInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub cp_amm_config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub bonding_curve_wsol_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub cp_amm_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMeteoraPool {
    type ArrangedAccounts = CreateMeteoraPoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [operator, config, vault_authority, cp_amm_config, pool_authority, pool, position, position_nft_mint, position_nft_account, token_a_mint, token_b_mint, token_a_vault, token_b_vault, bonding_curve, bonding_curve_vault, bonding_curve_wsol_vault, token_program, token_2022_program, system_program, event_authority, cp_amm_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateMeteoraPoolInstructionAccounts {
            operator: operator.pubkey,
            config: config.pubkey,
            vault_authority: vault_authority.pubkey,
            cp_amm_config: cp_amm_config.pubkey,
            pool_authority: pool_authority.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_nft_mint: position_nft_mint.pubkey,
            position_nft_account: position_nft_account.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            bonding_curve: bonding_curve.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            bonding_curve_wsol_vault: bonding_curve_wsol_vault.pubkey,
            token_program: token_program.pubkey,
            token_2022_program: token_2022_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            cp_amm_program: cp_amm_program.pubkey,
        })
    }
}
