use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf717ff41d45addc2")]
pub struct UpdatePerpBidAskTwap {}

pub struct UpdatePerpBidAskTwapInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub keeper_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePerpBidAskTwap {
    type ArrangedAccounts = UpdatePerpBidAskTwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, perp_market, oracle, keeper_stats, authority, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(UpdatePerpBidAskTwapInstructionAccounts {
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            oracle: oracle.pubkey,
            keeper_stats: keeper_stats.pubkey,
            authority: authority.pubkey,
        })
    }
}
