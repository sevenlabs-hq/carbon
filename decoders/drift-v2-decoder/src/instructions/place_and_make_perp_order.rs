use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x95750bed2f5f59ed")]
pub struct PlaceAndMakePerpOrder {
    pub params: OrderParams,
    pub taker_order_id: u32,
}

pub struct PlaceAndMakePerpOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub taker: solana_pubkey::Pubkey,
    pub taker_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceAndMakePerpOrder {
    type ArrangedAccounts = PlaceAndMakePerpOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, user_stats, taker, taker_stats, authority, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(PlaceAndMakePerpOrderInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            taker: taker.pubkey,
            taker_stats: taker_stats.pubkey,
            authority: authority.pubkey,
        })
    }
}
