use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbf038a4772c6ca64")]
pub struct PlaceAndTakeSpotOrder {
    pub params: OrderParams,
    pub fulfillment_type: Option<SpotFulfillmentType>,
    pub maker_order_id: Option<u32>,
}

pub struct PlaceAndTakeSpotOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceAndTakeSpotOrder {
    type ArrangedAccounts = PlaceAndTakeSpotOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, user_stats, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PlaceAndTakeSpotOrderInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
        })
    }
}
