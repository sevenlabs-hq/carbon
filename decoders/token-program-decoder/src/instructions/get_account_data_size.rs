use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x15")]
pub struct GetAccountDataSize {}

pub struct GetAccountDataSizeAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for GetAccountDataSize {
    type ArrangedAccounts = GetAccountDataSizeAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(GetAccountDataSizeAccounts { mint: *mint })
    }
}
