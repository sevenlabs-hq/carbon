
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x00")]
pub struct CreateV1{
    pub create_v1_args: CreateV1Args,
}

pub struct CreateV1InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateV1 {
    type ArrangedAccounts = CreateV1InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let asset = accounts.get(0)?;
        let collection = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let update_authority = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let log_wrapper = accounts.get(7)?;

        Some(CreateV1InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            update_authority: update_authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}