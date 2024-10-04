use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x10")]
pub struct InitializeAccount2 {
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct InitializeAccount2Accounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializeAccount2 {
    type ArrangedAccounts = InitializeAccount2Accounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;
        let mint = accounts.get(1)?;

        Some(InitializeAccount2Accounts {
            account: *account,
            mint: *mint,
        })
    }
}
