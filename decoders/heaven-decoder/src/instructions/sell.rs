use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x33e685a4017f83ad")]
pub struct Sell {
    pub params: SellParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SellInstructionAccounts {
    pub token_a_program: solana_pubkey::Pubkey,
    pub token_b_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub liquidity_pool_state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub user_token_a_vault: solana_pubkey::Pubkey,
    pub user_token_b_vault: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub protocol_config: solana_pubkey::Pubkey,
    pub instruction_sysvar_account_info: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Sell {
    type ArrangedAccounts = SellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_a_program = next_account(&mut iter)?;
        let token_b_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let liquidity_pool_state = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let token_a_mint = next_account(&mut iter)?;
        let token_b_mint = next_account(&mut iter)?;
        let user_token_a_vault = next_account(&mut iter)?;
        let user_token_b_vault = next_account(&mut iter)?;
        let token_a_vault = next_account(&mut iter)?;
        let token_b_vault = next_account(&mut iter)?;
        let protocol_config = next_account(&mut iter)?;
        let instruction_sysvar_account_info = next_account(&mut iter)?;

        Some(SellInstructionAccounts {
            token_a_program,
            token_b_program,
            associated_token_program,
            system_program,
            liquidity_pool_state,
            user,
            token_a_mint,
            token_b_mint,
            user_token_a_vault,
            user_token_b_vault,
            token_a_vault,
            token_b_vault,
            protocol_config,
            instruction_sysvar_account_info,
        })
    }
}
