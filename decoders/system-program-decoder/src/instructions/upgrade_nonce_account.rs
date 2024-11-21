use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0C")]
pub struct UpgradeNonceAccount;

pub struct UpgradeNonceAccountAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpgradeNonceAccount {
    type ArrangedAccounts = UpgradeNonceAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let nonce_account = accounts.get(0)?;

        Some(UpgradeNonceAccountAccounts {
            nonce_account: nonce_account.pubkey,
        })
    }
}
