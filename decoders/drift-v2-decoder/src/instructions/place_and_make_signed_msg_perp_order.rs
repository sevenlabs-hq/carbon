use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x101a7b835e1daf62")]
pub struct PlaceAndMakeSignedMsgPerpOrder {
    pub params: OrderParams,
    pub signed_msg_order_uuid: [u8; 8],
}

pub struct PlaceAndMakeSignedMsgPerpOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub taker: solana_pubkey::Pubkey,
    pub taker_stats: solana_pubkey::Pubkey,
    pub taker_signed_msg_user_orders: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceAndMakeSignedMsgPerpOrder {
    type ArrangedAccounts = PlaceAndMakeSignedMsgPerpOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, user_stats, taker, taker_stats, taker_signed_msg_user_orders, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlaceAndMakeSignedMsgPerpOrderInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            taker: taker.pubkey,
            taker_stats: taker_stats.pubkey,
            taker_signed_msg_user_orders: taker_signed_msg_user_orders.pubkey,
            authority: authority.pubkey,
        })
    }
}
