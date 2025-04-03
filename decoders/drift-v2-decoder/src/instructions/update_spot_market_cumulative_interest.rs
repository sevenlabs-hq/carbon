use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x27a68bf39ea59be1")]
pub struct UpdateSpotMarketCumulativeInterest {}

pub struct UpdateSpotMarketCumulativeInterestInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotMarketCumulativeInterest {
    type ArrangedAccounts = UpdateSpotMarketCumulativeInterestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, oracle, spot_market_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketCumulativeInterestInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            oracle: oracle.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
        })
    }
}
