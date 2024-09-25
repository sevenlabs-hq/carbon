
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa015bd07dd7f35e4")]
pub struct SetTokenLedger{
}

pub struct SetTokenLedgerInstructionAccounts {
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetTokenLedger {
    type ArrangedAccounts = SetTokenLedgerInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_ledger = accounts.get(0)?;
        let token_account = accounts.get(1)?;

        Some(SetTokenLedgerInstructionAccounts {
            token_ledger: *token_ledger,
            token_account: *token_account,
        })
    }
}