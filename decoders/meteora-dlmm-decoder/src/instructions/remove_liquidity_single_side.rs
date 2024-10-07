
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5454b142feb90afb")]
pub struct RemoveLiquiditySingleSide{
}

pub struct RemoveLiquiditySingleSideInstructionAccounts {
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub user_token: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub bin_array_lower: solana_sdk::pubkey::Pubkey,
    pub bin_array_upper: solana_sdk::pubkey::Pubkey,
    pub sender: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for RemoveLiquiditySingleSide {
    type ArrangedAccounts = RemoveLiquiditySingleSideInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let position = accounts.get(0)?;
        let lb_pair = accounts.get(1)?;
        let bin_array_bitmap_extension = accounts.get(2)?;
        let user_token = accounts.get(3)?;
        let reserve = accounts.get(4)?;
        let token_mint = accounts.get(5)?;
        let bin_array_lower = accounts.get(6)?;
        let bin_array_upper = accounts.get(7)?;
        let sender = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

        Some(RemoveLiquiditySingleSideInstructionAccounts {
            position: *position,
            lb_pair: *lb_pair,
            bin_array_bitmap_extension: *bin_array_bitmap_extension,
            user_token: *user_token,
            reserve: *reserve,
            token_mint: *token_mint,
            bin_array_lower: *bin_array_lower,
            bin_array_upper: *bin_array_upper,
            sender: *sender,
            token_program: *token_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}