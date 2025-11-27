use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd8ac82942262d7a3")]
pub struct SwapTokensForSolOnRaydium {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SwapTokensForSolOnRaydiumInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub input_vault: solana_pubkey::Pubkey,
    pub output_vault: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub bonding_curve_wsol_vault: solana_pubkey::Pubkey,
    pub input_token_mint: solana_pubkey::Pubkey,
    pub output_token_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub cp_swap_program: solana_pubkey::Pubkey,
    pub observation_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapTokensForSolOnRaydium {
    type ArrangedAccounts = SwapTokensForSolOnRaydiumInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, bonding_curve, amm_config, operator, vault_authority, authority, pool_state, input_vault, output_vault, bonding_curve_vault, bonding_curve_wsol_vault, input_token_mint, output_token_mint, token_program, cp_swap_program, observation_state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapTokensForSolOnRaydiumInstructionAccounts {
            config: config.pubkey,
            bonding_curve: bonding_curve.pubkey,
            amm_config: amm_config.pubkey,
            operator: operator.pubkey,
            vault_authority: vault_authority.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            input_vault: input_vault.pubkey,
            output_vault: output_vault.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            bonding_curve_wsol_vault: bonding_curve_wsol_vault.pubkey,
            input_token_mint: input_token_mint.pubkey,
            output_token_mint: output_token_mint.pubkey,
            token_program: token_program.pubkey,
            cp_swap_program: cp_swap_program.pubkey,
            observation_state: observation_state.pubkey,
        })
    }
}
