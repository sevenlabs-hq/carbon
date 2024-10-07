
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4605845756ebb122")]
pub struct CollectReward{
    pub reward_index: u8,
}

pub struct CollectRewardInstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub reward_owner_account: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CollectReward {
    type ArrangedAccounts = CollectRewardInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let position_authority = accounts.get(1)?;
        let position = accounts.get(2)?;
        let position_token_account = accounts.get(3)?;
        let reward_owner_account = accounts.get(4)?;
        let reward_vault = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(CollectRewardInstructionAccounts {
            whirlpool: *whirlpool,
            position_authority: *position_authority,
            position: *position,
            position_token_account: *position_token_account,
            reward_owner_account: *reward_owner_account,
            reward_vault: *reward_vault,
            token_program: *token_program,
        })
    }
}