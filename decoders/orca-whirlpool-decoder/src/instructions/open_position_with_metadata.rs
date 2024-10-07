
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf21d86303a6e0e3c")]
pub struct OpenPositionWithMetadata{
    pub bumps: OpenPositionWithMetadataBumps,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

pub struct OpenPositionWithMetadataInstructionAccounts {
    pub funder: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_metadata_account: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub metadata_update_auth: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for OpenPositionWithMetadata {
    type ArrangedAccounts = OpenPositionWithMetadataInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let funder = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let position = accounts.get(2)?;
        let position_mint = accounts.get(3)?;
        let position_metadata_account = accounts.get(4)?;
        let position_token_account = accounts.get(5)?;
        let whirlpool = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let rent = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let metadata_program = accounts.get(11)?;
        let metadata_update_auth = accounts.get(12)?;

        Some(OpenPositionWithMetadataInstructionAccounts {
            funder: *funder,
            owner: *owner,
            position: *position,
            position_mint: *position_mint,
            position_metadata_account: *position_metadata_account,
            position_token_account: *position_token_account,
            whirlpool: *whirlpool,
            token_program: *token_program,
            system_program: *system_program,
            rent: *rent,
            associated_token_program: *associated_token_program,
            metadata_program: *metadata_program,
            metadata_update_auth: *metadata_update_auth,
        })
    }
}