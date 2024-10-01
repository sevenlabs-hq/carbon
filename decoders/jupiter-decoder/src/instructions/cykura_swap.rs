
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x26f1156b783bb8f9")]
pub struct CykuraSwap{
}

pub struct CykuraSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub factory_state: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub input_vault: solana_sdk::pubkey::Pubkey,
    pub output_vault: solana_sdk::pubkey::Pubkey,
    pub last_observation_state: solana_sdk::pubkey::Pubkey,
    pub core_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CykuraSwap {
    type ArrangedAccounts = CykuraSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let signer = accounts.get(1)?;
        let factory_state = accounts.get(2)?;
        let pool_state = accounts.get(3)?;
        let input_token_account = accounts.get(4)?;
        let output_token_account = accounts.get(5)?;
        let input_vault = accounts.get(6)?;
        let output_vault = accounts.get(7)?;
        let last_observation_state = accounts.get(8)?;
        let core_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(CykuraSwapInstructionAccounts {
            swap_program: *swap_program,
            signer: *signer,
            factory_state: *factory_state,
            pool_state: *pool_state,
            input_token_account: *input_token_account,
            output_token_account: *output_token_account,
            input_vault: *input_vault,
            output_vault: *output_vault,
            last_observation_state: *last_observation_state,
            core_program: *core_program,
            token_program: *token_program,
        })
    }
}