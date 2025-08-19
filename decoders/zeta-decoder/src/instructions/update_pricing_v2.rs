use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeb6d8aad0f2533f4")]
pub struct UpdatePricingV2 {
    pub asset: Asset,
}

pub struct UpdatePricingV2InstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub perp_bids: solana_pubkey::Pubkey,
    pub perp_asks: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePricingV2 {
    type ArrangedAccounts = UpdatePricingV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, oracle, oracle_backup_feed, oracle_backup_program, perp_market, perp_bids, perp_asks, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdatePricingV2InstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            perp_market: perp_market.pubkey,
            perp_bids: perp_bids.pubkey,
            perp_asks: perp_asks.pubkey,
        })
    }
}
