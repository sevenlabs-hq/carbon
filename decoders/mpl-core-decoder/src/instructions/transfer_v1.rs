
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdb5f400a3789856d")]
pub struct TransferV1{
    pub transfer_v1_args: TransferV1Args,
}

pub struct TransferV1InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub new_owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for TransferV1 {
    type ArrangedAccounts = TransferV1InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let asset = accounts.get(0)?;
        let collection = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let new_owner = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let log_wrapper = accounts.get(6)?;

        Some(TransferV1InstructionAccounts {
            asset: *asset,
            collection: *collection,
            payer: *payer,
            authority: *authority,
            new_owner: *new_owner,
            system_program: *system_program,
            log_wrapper: *log_wrapper,
        })
    }
}