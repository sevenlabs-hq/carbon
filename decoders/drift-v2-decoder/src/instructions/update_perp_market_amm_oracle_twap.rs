use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf14a727bce9918ca")]
pub struct UpdatePerpMarketAmmOracleTwap {}

pub struct UpdatePerpMarketAmmOracleTwapInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub perp_market: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePerpMarketAmmOracleTwap {
    type ArrangedAccounts = UpdatePerpMarketAmmOracleTwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, perp_market, oracle, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketAmmOracleTwapInstructionAccounts {
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            oracle: oracle.pubkey,
            admin: admin.pubkey,
        })
    }
}
