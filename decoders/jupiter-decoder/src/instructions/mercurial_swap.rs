
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x02054dadc500079d")]
pub struct MercurialSwap{
}

pub struct MercurialSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub swap_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub pool_authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MercurialSwap {
    type ArrangedAccounts = MercurialSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let swap_state = accounts.get(1)?;
        let token_program = accounts.get(2)?;
        let pool_authority = accounts.get(3)?;
        let user_transfer_authority = accounts.get(4)?;
        let source_token_account = accounts.get(5)?;
        let destination_token_account = accounts.get(6)?;

        Some(MercurialSwapInstructionAccounts {
            swap_program: *swap_program,
            swap_state: *swap_state,
            token_program: *token_program,
            pool_authority: *pool_authority,
            user_transfer_authority: *user_transfer_authority,
            source_token_account: *source_token_account,
            destination_token_account: *destination_token_account,
        })
    }
}