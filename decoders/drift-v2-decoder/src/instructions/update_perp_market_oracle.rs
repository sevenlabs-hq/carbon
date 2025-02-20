use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb6716fa043ae59bf")]
pub struct UpdatePerpMarketOracle {
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub oracle_source: OracleSource,
    pub skip_invariant_check: bool,
}

pub struct UpdatePerpMarketOracleInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub perp_market: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub old_oracle: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePerpMarketOracle {
    type ArrangedAccounts = UpdatePerpMarketOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, oracle, old_oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketOracleInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            oracle: oracle.pubkey,
            old_oracle: old_oracle.pubkey,
        })
    }
}
