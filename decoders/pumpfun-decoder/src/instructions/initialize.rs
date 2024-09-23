use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize;

pub struct InitializeInstructionAccounts {
    pub global: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
}

impl ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<Pubkey>) -> Option<Self::ArrangedAccounts> {
        let global = accounts.get(0)?;
        let user = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        Some(InitializeInstructionAccounts {
            global: *global,
            user: *user,
            system_program: *system_program,
        })
    }
}
