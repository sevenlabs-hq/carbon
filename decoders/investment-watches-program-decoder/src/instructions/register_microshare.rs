
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0f7ce967b184d2d8")]
pub struct RegisterMicroshare{
}

pub struct RegisterMicroshareInstructionAccounts {
    pub watch: solana_sdk::pubkey::Pubkey,
    pub microshare_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub microshare_asset: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for RegisterMicroshare {
    type ArrangedAccounts = RegisterMicroshareInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let watch = accounts.get(0)?;
        let microshare_account = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let microshare_asset = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(RegisterMicroshareInstructionAccounts {
            watch: *watch,
            microshare_account: *microshare_account,
            authority: *authority,
            payer: *payer,
            microshare_asset: *microshare_asset,
            system_program: *system_program,
        })
    }
}