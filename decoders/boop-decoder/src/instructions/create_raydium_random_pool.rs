use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4e2cad1d84b404ac")]
pub struct CreateRaydiumRandomPool {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateRaydiumRandomPoolInstructionAccounts {
    pub cpmm_program: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub token_0_mint: solana_pubkey::Pubkey,
    pub token_1_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub bonding_curve_wsol_vault: solana_pubkey::Pubkey,
    pub creator_lp_token: solana_pubkey::Pubkey,
    pub token_0_vault: solana_pubkey::Pubkey,
    pub token_1_vault: solana_pubkey::Pubkey,
    pub create_pool_fee: solana_pubkey::Pubkey,
    pub observation_state: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateRaydiumRandomPool {
    type ArrangedAccounts = CreateRaydiumRandomPoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cpmm_program, amm_config, authority, pool_state, token_0_mint, token_1_mint, lp_mint, vault_authority, bonding_curve, bonding_curve_vault, bonding_curve_wsol_vault, creator_lp_token, token_0_vault, token_1_vault, create_pool_fee, observation_state, operator, config, token_program, associated_token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateRaydiumRandomPoolInstructionAccounts {
            cpmm_program: cpmm_program.pubkey,
            amm_config: amm_config.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            token_0_mint: token_0_mint.pubkey,
            token_1_mint: token_1_mint.pubkey,
            lp_mint: lp_mint.pubkey,
            vault_authority: vault_authority.pubkey,
            bonding_curve: bonding_curve.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            bonding_curve_wsol_vault: bonding_curve_wsol_vault.pubkey,
            creator_lp_token: creator_lp_token.pubkey,
            token_0_vault: token_0_vault.pubkey,
            token_1_vault: token_1_vault.pubkey,
            create_pool_fee: create_pool_fee.pubkey,
            observation_state: observation_state.pubkey,
            operator: operator.pubkey,
            config: config.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
