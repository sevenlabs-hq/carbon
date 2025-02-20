
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize{
    pub fees: Fees,
    pub swap_curve: SwapCurve,
}

pub struct InitializeInstructionAccounts {
    pub swap: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub token_a: solana_sdk::pubkey::Pubkey,
    pub token_b: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub fee: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            swap,
            authority,
            token_a,
            token_b,
            pool,
            fee,
            destination,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(InitializeInstructionAccounts {
            swap: swap.pubkey,
            authority: authority.pubkey,
            token_a: token_a.pubkey,
            token_b: token_b.pubkey,
            pool: pool.pubkey,
            fee: fee.pubkey,
            destination: destination.pubkey,
            token_program: token_program.pubkey,
        })
    }
}