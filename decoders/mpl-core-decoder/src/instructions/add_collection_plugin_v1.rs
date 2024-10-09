
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x178eaad55ddab302")]
pub struct AddCollectionPluginV1{
    pub add_collection_plugin_v1_args: AddCollectionPluginV1Args,
}

pub struct AddCollectionPluginV1InstructionAccounts {
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for AddCollectionPluginV1 {
    type ArrangedAccounts = AddCollectionPluginV1InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let collection = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let log_wrapper = accounts.get(4)?;

        Some(AddCollectionPluginV1InstructionAccounts {
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}