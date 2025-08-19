use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x898c5e12e7e8d9cc")]
pub struct CleanZetaMarketHalted {
    pub asset: Asset,
}

pub struct CleanZetaMarketHaltedInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CleanZetaMarketHalted {
    type ArrangedAccounts = CleanZetaMarketHaltedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, market, bids, asks, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CleanZetaMarketHaltedInstructionAccounts {
            state: state.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
        })
    }
}
