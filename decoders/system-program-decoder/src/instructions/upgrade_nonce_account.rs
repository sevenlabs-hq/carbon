use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0C")]
pub struct UpgradeNonceAccount;

pub struct UpgradeNonceAccountAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpgradeNonceAccount {
    type ArrangedAccounts = UpgradeNonceAccountAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let nonce_account = accounts.get(0)?;

        Some(UpgradeNonceAccountAccounts {
            nonce_account: *nonce_account,
        })
    }
}
