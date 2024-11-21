use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x16")]
pub struct InitializeImmutableOwner {}

pub struct InitializeImmutableOwnerAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeImmutableOwner {
    type ArrangedAccounts = InitializeImmutableOwnerAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;

        Some(InitializeImmutableOwnerAccounts {
            account: account.pubkey,
        })
    }
}
