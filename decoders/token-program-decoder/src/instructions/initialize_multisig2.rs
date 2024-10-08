use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x13")]
pub struct InitializeMultisig2 {
    pub m: u8,
}

pub struct InitializeMultisig2Accounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl ArrangeAccounts for InitializeMultisig2 {
    type ArrangedAccounts = InitializeMultisig2Accounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;

        Some(InitializeMultisig2Accounts {
            account: account.pubkey,
            remaining_accounts: accounts.get(1..).unwrap_or_default().to_vec(),
        })
    }
}
