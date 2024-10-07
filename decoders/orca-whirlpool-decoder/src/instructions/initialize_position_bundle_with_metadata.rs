
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5d7c10b3f98373f5")]
pub struct InitializePositionBundleWithMetadata{
}

pub struct InitializePositionBundleWithMetadataInstructionAccounts {
    pub position_bundle: solana_sdk::pubkey::Pubkey,
    pub position_bundle_mint: solana_sdk::pubkey::Pubkey,
    pub position_bundle_metadata: solana_sdk::pubkey::Pubkey,
    pub position_bundle_token_account: solana_sdk::pubkey::Pubkey,
    pub position_bundle_owner: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub metadata_update_auth: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializePositionBundleWithMetadata {
    type ArrangedAccounts = InitializePositionBundleWithMetadataInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let position_bundle = accounts.get(0)?;
        let position_bundle_mint = accounts.get(1)?;
        let position_bundle_metadata = accounts.get(2)?;
        let position_bundle_token_account = accounts.get(3)?;
        let position_bundle_owner = accounts.get(4)?;
        let funder = accounts.get(5)?;
        let metadata_update_auth = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let rent = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let metadata_program = accounts.get(11)?;

        Some(InitializePositionBundleWithMetadataInstructionAccounts {
            position_bundle: *position_bundle,
            position_bundle_mint: *position_bundle_mint,
            position_bundle_metadata: *position_bundle_metadata,
            position_bundle_token_account: *position_bundle_token_account,
            position_bundle_owner: *position_bundle_owner,
            funder: *funder,
            metadata_update_auth: *metadata_update_auth,
            token_program: *token_program,
            system_program: *system_program,
            rent: *rent,
            associated_token_program: *associated_token_program,
            metadata_program: *metadata_program,
        })
    }
}