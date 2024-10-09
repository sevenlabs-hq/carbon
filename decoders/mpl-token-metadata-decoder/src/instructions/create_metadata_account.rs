
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4b492db2d4c27f71")]
pub struct CreateMetadataAccount{
    pub create_metadata_account_args: CreateMetadataAccountArgs,
}

pub struct CreateMetadataAccountInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CreateMetadataAccount {
    type ArrangedAccounts = CreateMetadataAccountInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let mint_authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let update_authority = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let rent = accounts.get(6)?;

        Some(CreateMetadataAccountInstructionAccounts {
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
            payer: payer.pubkey,
            update_authority: update_authority.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}