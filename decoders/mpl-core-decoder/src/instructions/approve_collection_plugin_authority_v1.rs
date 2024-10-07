
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xba633dbd6f18380a")]
pub struct ApproveCollectionPluginAuthorityV1{
    pub approve_collection_plugin_authority_v1_args: ApproveCollectionPluginAuthorityV1Args,
}

pub struct ApproveCollectionPluginAuthorityV1InstructionAccounts {
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for ApproveCollectionPluginAuthorityV1 {
    type ArrangedAccounts = ApproveCollectionPluginAuthorityV1InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let collection = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let log_wrapper = accounts.get(4)?;

        Some(ApproveCollectionPluginAuthorityV1InstructionAccounts {
            collection: *collection,
            payer: *payer,
            authority: *authority,
            system_program: *system_program,
            log_wrapper: *log_wrapper,
        })
    }
}