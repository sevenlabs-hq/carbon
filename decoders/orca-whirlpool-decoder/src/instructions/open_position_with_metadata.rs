
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;


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

impl carbon_core::deserialize::ArrangeAccounts for OpenPositionWithMetadata {
    type ArrangedAccounts = OpenPositionWithMetadataInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
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
            funder: funder.pubkey,
            owner: owner.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_metadata_account: position_metadata_account.pubkey,
            position_token_account: position_token_account.pubkey,
            whirlpool: whirlpool.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            associated_token_program: associated_token_program.pubkey,
            metadata_program: metadata_program.pubkey,
            metadata_update_auth: metadata_update_auth.pubkey,
        })
    }
}
