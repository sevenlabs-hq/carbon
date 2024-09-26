
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x98baf4f4218f2838")]
pub struct UpdateAmmConfig{
    pub param: u8,
    pub value: u32,
}

pub struct UpdateAmmConfigInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpdateAmmConfig {
    type ArrangedAccounts = UpdateAmmConfigInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let amm_config = accounts.get(1)?;

        Some(UpdateAmmConfigInstructionAccounts {
            owner: *owner,
            amm_config: *amm_config,
        })
    }
}