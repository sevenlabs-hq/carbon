
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x04695ca7841c095a")]
pub struct UpdateWhitelistedWallet{
    pub wallet: solana_sdk::pubkey::Pubkey,
}

pub struct UpdateWhitelistedWalletInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub creator: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpdateWhitelistedWallet {
    type ArrangedAccounts = UpdateWhitelistedWalletInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let creator = accounts.get(1)?;

        Some(UpdateWhitelistedWalletInstructionAccounts {
            lb_pair: *lb_pair,
            creator: *creator,
        })
    }
}