use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf21d86303a6e0e3c")]
pub struct OpenPositionWithMetadata {
    pub bumps: OpenPositionWithMetadataBumps,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct OpenPositionWithMetadataInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_metadata_account: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub metadata_update_auth: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenPositionWithMetadata {
    type ArrangedAccounts = OpenPositionWithMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_mint = next_account(&mut iter)?;
        let position_metadata_account = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;
        let metadata_update_auth = next_account(&mut iter)?;

        Some(OpenPositionWithMetadataInstructionAccounts {
            funder,
            owner,
            position,
            position_mint,
            position_metadata_account,
            position_token_account,
            whirlpool,
            token_program,
            system_program,
            rent,
            associated_token_program,
            metadata_program,
            metadata_update_auth,
        })
    }
}
