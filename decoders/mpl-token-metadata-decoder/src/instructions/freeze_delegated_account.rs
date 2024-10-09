
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0e10bdb47413607f")]
pub struct FreezeDelegatedAccount{
}

pub struct FreezeDelegatedAccountInstructionAccounts {
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for FreezeDelegatedAccount {
    type ArrangedAccounts = FreezeDelegatedAccountInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let delegate = accounts.get(0)?;
        let token_account = accounts.get(1)?;
        let edition = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let token_program = accounts.get(4)?;

        Some(FreezeDelegatedAccountInstructionAccounts {
            delegate: delegate.pubkey,
            token_account: token_account.pubkey,
            edition: edition.pubkey,
            mint: mint.pubkey,
            token_program: token_program.pubkey,
        })
    }
}