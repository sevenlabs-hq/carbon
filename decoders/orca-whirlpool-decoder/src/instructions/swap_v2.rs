use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2b04ed0b1ac91e62")]
pub struct SwapV2 {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SwapV2InstructionAccounts {
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub token_authority: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub token_mint_a: solana_pubkey::Pubkey,
    pub token_mint_b: solana_pubkey::Pubkey,
    pub token_owner_account_a: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_owner_account_b: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub tick_array0: solana_pubkey::Pubkey,
    pub tick_array1: solana_pubkey::Pubkey,
    pub tick_array2: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapV2 {
    type ArrangedAccounts = SwapV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_program_a = next_account(&mut iter)?;
        let token_program_b = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;
        let token_authority = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let token_mint_a = next_account(&mut iter)?;
        let token_mint_b = next_account(&mut iter)?;
        let token_owner_account_a = next_account(&mut iter)?;
        let token_vault_a = next_account(&mut iter)?;
        let token_owner_account_b = next_account(&mut iter)?;
        let token_vault_b = next_account(&mut iter)?;
        let tick_array0 = next_account(&mut iter)?;
        let tick_array1 = next_account(&mut iter)?;
        let tick_array2 = next_account(&mut iter)?;
        let oracle = next_account(&mut iter)?;

        Some(SwapV2InstructionAccounts {
            token_program_a,
            token_program_b,
            memo_program,
            token_authority,
            whirlpool,
            token_mint_a,
            token_mint_b,
            token_owner_account_a,
            token_vault_a,
            token_owner_account_b,
            token_vault_b,
            tick_array0,
            tick_array1,
            tick_array2,
            oracle,
        })
    }
}
