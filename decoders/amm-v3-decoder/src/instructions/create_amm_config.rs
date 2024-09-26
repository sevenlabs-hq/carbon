
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8254041a4cac4bc9")]
pub struct CreateAmmConfig{
    pub index: u16,
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
}

pub struct CreateAmmConfigInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CreateAmmConfig {
    type ArrangedAccounts = CreateAmmConfigInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let amm_config = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(CreateAmmConfigInstructionAccounts {
            owner: *owner,
            amm_config: *amm_config,
            system_program: *system_program,
        })
    }
}