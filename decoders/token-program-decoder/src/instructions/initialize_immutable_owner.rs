use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x16")]
pub struct InitializeImmutableOwner {}

pub struct InitializeImmutableOwnerAccounts {
    pub account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeImmutableOwner {
    type ArrangedAccounts = InitializeImmutableOwnerAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.first()?;

        Some(InitializeImmutableOwnerAccounts {
            account: account.pubkey,
        })
    }
}
