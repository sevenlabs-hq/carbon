use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b")]
pub struct ThawAccount {}

pub struct ThawAccountAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for ThawAccount {
    type ArrangedAccounts = ThawAccountAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, authority, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(ThawAccountAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
