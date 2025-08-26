use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

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
        let mut iter = accounts.iter();
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_a_mint = next_account(&mut iter)?;
        let token_b_mint = next_account(&mut iter)?;
        let user_token_a_vault = next_account(&mut iter)?;
        let token_a_vault = next_account(&mut iter)?;
        let token_b_vault = next_account(&mut iter)?;
        let liquidity_pool_state = next_account(&mut iter)?;
        let protocol_config = next_account(&mut iter)?;
        let token_a_program = next_account(&mut iter)?;

        Some(CreateStandardLiquidityPoolInstructionAccounts {
            token_program,
            associated_token_program,
            system_program,
            user,
            payer,
            token_a_mint,
            token_b_mint,
            user_token_a_vault,
            token_a_vault,
            token_b_vault,
            liquidity_pool_state,
            protocol_config,
            token_a_program,
        })
    }
}
