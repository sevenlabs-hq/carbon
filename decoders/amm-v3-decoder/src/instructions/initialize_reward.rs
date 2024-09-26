
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5655831b80c5def8")]
pub struct InitializeReward{
    pub param: InitializeRewardParam,
}

pub struct InitializeRewardInstructionAccounts {
    pub reward_funder: solana_sdk::pubkey::Pubkey,
    pub funder_token_account: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub operation_state: solana_sdk::pubkey::Pubkey,
    pub reward_token_mint: solana_sdk::pubkey::Pubkey,
    pub reward_token_vault: solana_sdk::pubkey::Pubkey,
    pub reward_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let reward_funder = accounts.get(0)?;
        let funder_token_account = accounts.get(1)?;
        let amm_config = accounts.get(2)?;
        let pool_state = accounts.get(3)?;
        let operation_state = accounts.get(4)?;
        let reward_token_mint = accounts.get(5)?;
        let reward_token_vault = accounts.get(6)?;
        let reward_token_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let rent = accounts.get(9)?;

        Some(InitializeRewardInstructionAccounts {
            reward_funder: *reward_funder,
            funder_token_account: *funder_token_account,
            amm_config: *amm_config,
            pool_state: *pool_state,
            operation_state: *operation_state,
            reward_token_mint: *reward_token_mint,
            reward_token_vault: *reward_token_vault,
            reward_token_program: *reward_token_program,
            system_program: *system_program,
            rent: *rent,
        })
    }
}