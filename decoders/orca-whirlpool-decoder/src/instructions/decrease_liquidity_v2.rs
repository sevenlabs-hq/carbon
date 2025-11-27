use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3a7fbc3e4f52c460")]
pub struct DecreaseLiquidityV2 {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityV2InstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub token_mint_a: solana_pubkey::Pubkey,
    pub token_mint_b: solana_pubkey::Pubkey,
    pub token_owner_account_a: solana_pubkey::Pubkey,
    pub token_owner_account_b: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecreaseLiquidityV2 {
    type ArrangedAccounts = DecreaseLiquidityV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let token_program_a = next_account(&mut iter)?;
        let token_program_b = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;
        let position_authority = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let token_mint_a = next_account(&mut iter)?;
        let token_mint_b = next_account(&mut iter)?;
        let token_owner_account_a = next_account(&mut iter)?;
        let token_owner_account_b = next_account(&mut iter)?;
        let token_vault_a = next_account(&mut iter)?;
        let token_vault_b = next_account(&mut iter)?;
        let tick_array_lower = next_account(&mut iter)?;
        let tick_array_upper = next_account(&mut iter)?;

        Some(DecreaseLiquidityV2InstructionAccounts {
            whirlpool,
            token_program_a,
            token_program_b,
            memo_program,
            position_authority,
            position,
            position_token_account,
            token_mint_a,
            token_mint_b,
            token_owner_account_a,
            token_owner_account_b,
            token_vault_a,
            token_vault_b,
            tick_array_lower,
            tick_array_upper,
        })
    }
}
