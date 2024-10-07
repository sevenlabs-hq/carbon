use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11")]
pub struct SyncNative {}

pub struct SyncNativeAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SyncNative {
    type ArrangedAccounts = SyncNativeAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;

        Some(SyncNativeAccounts { account: *account })
    }
}
