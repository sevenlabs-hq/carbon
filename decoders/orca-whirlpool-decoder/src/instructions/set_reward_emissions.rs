
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0dc556a86db01bf4")]
pub struct SetRewardEmissions{
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

pub struct SetRewardEmissionsInstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub reward_authority: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetRewardEmissions {
    type ArrangedAccounts = SetRewardEmissionsInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let reward_authority = accounts.get(1)?;
        let reward_vault = accounts.get(2)?;

        Some(SetRewardEmissionsInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            reward_authority: reward_authority.pubkey,
            reward_vault: reward_vault.pubkey,
        })
    }
}