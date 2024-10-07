
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc1653a41784e631f")]
pub struct WithdrawSrm{
    pub amount: u64,
}

pub struct WithdrawSrmInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_owner_account: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub srm_token: solana_sdk::pubkey::Pubkey,
    pub dest_srm_token: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for WithdrawSrm {
    type ArrangedAccounts = WithdrawSrmInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let amm = accounts.get(1)?;
        let amm_owner_account = accounts.get(2)?;
        let amm_authority = accounts.get(3)?;
        let srm_token = accounts.get(4)?;
        let dest_srm_token = accounts.get(5)?;

        Some(WithdrawSrmInstructionAccounts {
            token_program: *token_program,
            amm: *amm,
            amm_owner_account: *amm_owner_account,
            amm_authority: *amm_authority,
            srm_token: *srm_token,
            dest_srm_token: *dest_srm_token,
        })
    }
}