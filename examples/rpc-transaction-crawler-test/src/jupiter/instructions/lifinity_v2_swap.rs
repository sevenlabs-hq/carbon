
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1398c3f5bb904ae3")]
pub struct LifinityV2Swap{
}

pub struct LifinityV2SwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source_info: solana_sdk::pubkey::Pubkey,
    pub destination_info: solana_sdk::pubkey::Pubkey,
    pub swap_source: solana_sdk::pubkey::Pubkey,
    pub swap_destination: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub fee_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub oracle_main_account: solana_sdk::pubkey::Pubkey,
    pub oracle_sub_account: solana_sdk::pubkey::Pubkey,
    pub oracle_pc_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for LifinityV2Swap {
    type ArrangedAccounts = LifinityV2SwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let amm = accounts.get(2)?;
        let user_transfer_authority = accounts.get(3)?;
        let source_info = accounts.get(4)?;
        let destination_info = accounts.get(5)?;
        let swap_source = accounts.get(6)?;
        let swap_destination = accounts.get(7)?;
        let pool_mint = accounts.get(8)?;
        let fee_account = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let oracle_main_account = accounts.get(11)?;
        let oracle_sub_account = accounts.get(12)?;
        let oracle_pc_account = accounts.get(13)?;

        Some(LifinityV2SwapInstructionAccounts {
            swap_program: *swap_program,
            authority: *authority,
            amm: *amm,
            user_transfer_authority: *user_transfer_authority,
            source_info: *source_info,
            destination_info: *destination_info,
            swap_source: *swap_source,
            swap_destination: *swap_destination,
            pool_mint: *pool_mint,
            fee_account: *fee_account,
            token_program: *token_program,
            oracle_main_account: *oracle_main_account,
            oracle_sub_account: *oracle_sub_account,
            oracle_pc_account: *oracle_pc_account,
        })
    }
}