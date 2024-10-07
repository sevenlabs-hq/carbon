
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6c66d555fb033515")]
pub struct InitializePermissionLbPair{
    pub ix_data: InitPermissionPairIx,
}

pub struct InitializePermissionLbPairInstructionAccounts {
    pub base: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub token_mint_x: solana_sdk::pubkey::Pubkey,
    pub token_mint_y: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializePermissionLbPair {
    type ArrangedAccounts = InitializePermissionLbPairInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let base = accounts.get(0)?;
        let lb_pair = accounts.get(1)?;
        let bin_array_bitmap_extension = accounts.get(2)?;
        let token_mint_x = accounts.get(3)?;
        let token_mint_y = accounts.get(4)?;
        let reserve_x = accounts.get(5)?;
        let reserve_y = accounts.get(6)?;
        let oracle = accounts.get(7)?;
        let admin = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let rent = accounts.get(11)?;
        let event_authority = accounts.get(12)?;
        let program = accounts.get(13)?;

        Some(InitializePermissionLbPairInstructionAccounts {
            base: *base,
            lb_pair: *lb_pair,
            bin_array_bitmap_extension: *bin_array_bitmap_extension,
            token_mint_x: *token_mint_x,
            token_mint_y: *token_mint_y,
            reserve_x: *reserve_x,
            reserve_y: *reserve_y,
            oracle: *oracle,
            admin: *admin,
            token_program: *token_program,
            system_program: *system_program,
            rent: *rent,
            event_authority: *event_authority,
            program: *program,
        })
    }
}