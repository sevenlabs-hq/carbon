use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6c66d555fb033515")]
pub struct InitializePermissionLbPair {
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

impl carbon_core::deserialize::ArrangeAccounts for InitializePermissionLbPair {
    type ArrangedAccounts = InitializePermissionLbPairInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
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
            base: base.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            token_mint_x: token_mint_x.pubkey,
            token_mint_y: token_mint_y.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            oracle: oracle.pubkey,
            admin: admin.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
