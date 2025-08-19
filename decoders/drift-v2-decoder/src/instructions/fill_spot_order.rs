use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd4ce82ad1522c728")]
pub struct FillSpotOrder {
    pub order_id: Option<u32>,
    pub fulfillment_type: Option<SpotFulfillmentType>,
    pub maker_order_id: Option<u32>,
}

pub struct FillSpotOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub filler: solana_pubkey::Pubkey,
    pub filler_stats: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FillSpotOrder {
    type ArrangedAccounts = FillSpotOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, filler, filler_stats, user, user_stats, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(FillSpotOrderInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            filler: filler.pubkey,
            filler_stats: filler_stats.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
