use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa859631e753158e0")]
pub struct DepositIntoRaydium {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositIntoRaydiumInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub operator_wsol_account: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub token_0_vault: solana_pubkey::Pubkey,
    pub token_1_vault: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub bonding_curve_wsol_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub cpmm_program: solana_pubkey::Pubkey,
    pub owner_lp_token: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub token_0_mint: solana_pubkey::Pubkey,
    pub token_1_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositIntoRaydium {
    type ArrangedAccounts = DepositIntoRaydiumInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, amm_config, operator, operator_wsol_account, vault_authority, authority, pool_state, token_0_vault, token_1_vault, bonding_curve_vault, bonding_curve_wsol_vault, token_program, token_program_2022, system_program, associated_token_program, lp_mint, cpmm_program, owner_lp_token, bonding_curve, token_0_mint, token_1_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositIntoRaydiumInstructionAccounts {
            config: config.pubkey,
            amm_config: amm_config.pubkey,
            operator: operator.pubkey,
            operator_wsol_account: operator_wsol_account.pubkey,
            vault_authority: vault_authority.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            token_0_vault: token_0_vault.pubkey,
            token_1_vault: token_1_vault.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            bonding_curve_wsol_vault: bonding_curve_wsol_vault.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            lp_mint: lp_mint.pubkey,
            cpmm_program: cpmm_program.pubkey,
            owner_lp_token: owner_lp_token.pubkey,
            bonding_curve: bonding_curve.pubkey,
            token_0_mint: token_0_mint.pubkey,
            token_1_mint: token_1_mint.pubkey,
        })
    }
}
