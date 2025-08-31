use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2a2b7e38e70ad035")]
pub struct CreateStandardLiquidityPool {
    pub protocol_config_version: u16,
    pub params: CreateStandardLiquidityPoolParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateStandardLiquidityPoolInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub user_token_a_vault: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub liquidity_pool_state: solana_pubkey::Pubkey,
    pub protocol_config: solana_pubkey::Pubkey,
    pub token_a_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateStandardLiquidityPool {
    type ArrangedAccounts = CreateStandardLiquidityPoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, associated_token_program, system_program, user, payer, token_a_mint, token_b_mint, user_token_a_vault, token_a_vault, token_b_vault, liquidity_pool_state, protocol_config, token_a_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateStandardLiquidityPoolInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            user: user.pubkey,
            payer: payer.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            user_token_a_vault: user_token_a_vault.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            liquidity_pool_state: liquidity_pool_state.pubkey,
            protocol_config: protocol_config.pubkey,
            token_a_program: token_a_program.pubkey,
        })
    }
}
