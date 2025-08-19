use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x72b86625f6bab463")]
pub struct UpdateSpotMarketOracle {
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_source: OracleSource,
    pub skip_invariant_check: bool,
}

pub struct UpdateSpotMarketOracleInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub old_oracle: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotMarketOracle {
    type ArrangedAccounts = UpdateSpotMarketOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, oracle, old_oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketOracleInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            oracle: oracle.pubkey,
            old_oracle: old_oracle.pubkey,
        })
    }
}
