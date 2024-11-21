

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa513197f64371f5a")]
pub struct InitReferrerStateAndShortUrl{
    pub short_url: String,
}

pub struct InitReferrerStateAndShortUrlInstructionAccounts {
    pub referrer: solana_sdk::pubkey::Pubkey,
    pub referrer_state: solana_sdk::pubkey::Pubkey,
    pub referrer_short_url: solana_sdk::pubkey::Pubkey,
    pub referrer_user_metadata: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitReferrerStateAndShortUrl {
    type ArrangedAccounts = InitReferrerStateAndShortUrlInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let referrer = accounts.get(0)?;
        let referrer_state = accounts.get(1)?;
        let referrer_short_url = accounts.get(2)?;
        let referrer_user_metadata = accounts.get(3)?;
        let rent = accounts.get(4)?;
        let system_program = accounts.get(5)?;

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