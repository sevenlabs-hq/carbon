
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe8fdc3f794d449de")]
pub struct UpdateFee{
    pub params: UpdateFeeParams,
}

pub struct UpdateFeeInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpdateFee {
    type ArrangedAccounts = UpdateFeeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(UpdateFeeInstructionAccounts {
            admin: *admin,
            fee_authority: *fee_authority,
            system_program: *system_program,
        })
    }
}