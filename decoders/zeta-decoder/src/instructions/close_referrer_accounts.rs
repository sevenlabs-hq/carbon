use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe04e378bcbec3e4e")]
pub struct CloseReferrerAccounts {}

pub struct CloseReferrerAccountsInstructionAccounts {
    pub referrer_id_account: solana_pubkey::Pubkey,
    pub referrer_pubkey_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseReferrerAccounts {
    type ArrangedAccounts = CloseReferrerAccountsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [referrer_id_account, referrer_pubkey_account, authority, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(CloseReferrerAccountsInstructionAccounts {
            referrer_id_account: referrer_id_account.pubkey,
            referrer_pubkey_account: referrer_pubkey_account.pubkey,
            authority: authority.pubkey,
        })
    }
}
