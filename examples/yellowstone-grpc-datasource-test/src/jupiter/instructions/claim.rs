
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3ec6d6c1d59f6cd2")]
pub struct Claim{
    pub id: u8,
}

pub struct ClaimInstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Claim {
    type ArrangedAccounts = ClaimInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let program_authority = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(ClaimInstructionAccounts {
            wallet: *wallet,
            program_authority: *program_authority,
            system_program: *system_program,
        })
    }
}