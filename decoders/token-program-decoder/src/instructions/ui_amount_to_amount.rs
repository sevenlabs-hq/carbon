use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x18")]
pub struct UiAmountToAmount {
    // TODO: Check what to do here, on github it's: &'a str
    pub ui_amount: String,
}

pub struct UiAmountToAmountAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UiAmountToAmount {
    type ArrangedAccounts = UiAmountToAmountAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(UiAmountToAmountAccounts { mint: *mint })
    }
}
