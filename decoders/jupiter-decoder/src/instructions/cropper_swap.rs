use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe6d82fb6a575d267")]
pub struct CropperSwap {}

pub struct CropperSwapInstructionAccounts {
    pub token_swap_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub swap: solana_sdk::pubkey::Pubkey,
    pub swap_state: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source: solana_sdk::pubkey::Pubkey,
    pub swap_source: solana_sdk::pubkey::Pubkey,
    pub swap_destination: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub pool_fee: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CropperSwap {
    type ArrangedAccounts = CropperSwapInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_swap_program = accounts.get(0)?;
        let token_program = accounts.get(1)?;
        let swap = accounts.get(2)?;
        let swap_state = accounts.get(3)?;
        let authority = accounts.get(4)?;
        let user_transfer_authority = accounts.get(5)?;
        let source = accounts.get(6)?;
        let swap_source = accounts.get(7)?;
        let swap_destination = accounts.get(8)?;
        let destination = accounts.get(9)?;
        let pool_mint = accounts.get(10)?;
        let pool_fee = accounts.get(11)?;

        Some(CropperSwapInstructionAccounts {
            token_swap_program: *token_swap_program,
            token_program: *token_program,
            swap: *swap,
            swap_state: *swap_state,
            authority: *authority,
            user_transfer_authority: *user_transfer_authority,
            source: *source,
            swap_source: *swap_source,
            swap_destination: *swap_destination,
            destination: *destination,
            pool_mint: *pool_mint,
            pool_fee: *pool_fee,
        })
    }
}
