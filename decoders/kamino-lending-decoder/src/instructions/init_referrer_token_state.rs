use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x742d42943a0dda73")]
pub struct InitReferrerTokenState {}

pub struct InitReferrerTokenStateInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub referrer: solana_sdk::pubkey::Pubkey,
    pub referrer_token_state: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitReferrerTokenState {
    type ArrangedAccounts = InitReferrerTokenStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, lending_market, reserve, referrer, referrer_token_state, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitReferrerTokenStateInstructionAccounts {
            payer: payer.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            referrer: referrer.pubkey,
            referrer_token_state: referrer_token_state.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
