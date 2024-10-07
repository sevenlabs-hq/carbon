
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xcf05c8d17a3852b7")]
pub struct SetRewardEmissionsSuperAuthority{
}

pub struct SetRewardEmissionsSuperAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_sdk::pubkey::Pubkey,
    pub new_reward_emissions_super_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetRewardEmissionsSuperAuthority {
    type ArrangedAccounts = SetRewardEmissionsSuperAuthorityInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let reward_emissions_super_authority = accounts.get(1)?;
        let new_reward_emissions_super_authority = accounts.get(2)?;

        Some(SetRewardEmissionsSuperAuthorityInstructionAccounts {
            whirlpools_config: *whirlpools_config,
            reward_emissions_super_authority: *reward_emissions_super_authority,
            new_reward_emissions_super_authority: *new_reward_emissions_super_authority,
        })
    }
}