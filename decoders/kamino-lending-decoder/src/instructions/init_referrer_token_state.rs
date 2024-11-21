

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x742d42943a0dda73")]
pub struct InitReferrerTokenState{
    pub referrer: solana_sdk::pubkey::Pubkey,
}

pub struct InitReferrerTokenStateInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub referrer_token_state: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitReferrerTokenState {
    type ArrangedAccounts = InitReferrerTokenStateInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let reserve = accounts.get(2)?;
        let referrer_token_state = accounts.get(3)?;
        let rent = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(InitReferrerTokenStateInstructionAccounts {
            payer: payer.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            referrer_token_state: referrer_token_state.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}