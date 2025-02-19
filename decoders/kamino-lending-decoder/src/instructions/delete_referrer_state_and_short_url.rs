use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x99b9631ce4b3bb96")]
pub struct DeleteReferrerStateAndShortUrl {}

pub struct DeleteReferrerStateAndShortUrlInstructionAccounts {
    pub referrer: solana_sdk::pubkey::Pubkey,
    pub referrer_state: solana_sdk::pubkey::Pubkey,
    pub short_url: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeleteReferrerStateAndShortUrl {
    type ArrangedAccounts = DeleteReferrerStateAndShortUrlInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [referrer, referrer_state, short_url, rent, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(DeleteReferrerStateAndShortUrlInstructionAccounts {
            referrer: referrer.pubkey,
            referrer_state: referrer_state.pubkey,
            short_url: short_url.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
