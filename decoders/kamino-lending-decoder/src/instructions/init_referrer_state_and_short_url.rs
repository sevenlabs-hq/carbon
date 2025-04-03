use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa513197f64371f5a")]
pub struct InitReferrerStateAndShortUrl {
    pub short_url: String,
}

pub struct InitReferrerStateAndShortUrlInstructionAccounts {
    pub referrer: solana_pubkey::Pubkey,
    pub referrer_state: solana_pubkey::Pubkey,
    pub referrer_short_url: solana_pubkey::Pubkey,
    pub referrer_user_metadata: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitReferrerStateAndShortUrl {
    type ArrangedAccounts = InitReferrerStateAndShortUrlInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [referrer, referrer_state, referrer_short_url, referrer_user_metadata, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitReferrerStateAndShortUrlInstructionAccounts {
            referrer: referrer.pubkey,
            referrer_state: referrer_state.pubkey,
            referrer_short_url: referrer_short_url.pubkey,
            referrer_user_metadata: referrer_user_metadata.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
