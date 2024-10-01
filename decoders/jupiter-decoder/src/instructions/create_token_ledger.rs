
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe8f2c5fdf08f8134")]
pub struct CreateTokenLedger{
}

pub struct CreateTokenLedgerInstructionAccounts {
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CreateTokenLedger {
    type ArrangedAccounts = CreateTokenLedgerInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_ledger = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(CreateTokenLedgerInstructionAccounts {
            token_ledger: *token_ledger,
            payer: *payer,
            system_program: *system_program,
        })
    }
}