use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7f0a37a47be22f18")]
pub struct ResetPerpMarketAmmOracleTwap {}

pub struct ResetPerpMarketAmmOracleTwapInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ResetPerpMarketAmmOracleTwap {
    type ArrangedAccounts = ResetPerpMarketAmmOracleTwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, perp_market, oracle, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ResetPerpMarketAmmOracleTwapInstructionAccounts {
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            oracle: oracle.pubkey,
            admin: admin.pubkey,
        })
    }
}
