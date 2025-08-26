use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum AdminUpdateLiquidityPoolState {
    CreatorTradingFeeClaimStatus(CreatorTradingFeeClaimStatus),
    CreatorTradingFeeDistribution(CreatorTradingFeeDistribution),
    CreatorTradingFeeReceiver(solana_pubkey::Pubkey),
    FeeConfigurationMode(FeeConfigurationMode),
    SlotOffsetBasedFees {
        fee_type: FeeType,
        fees: SlotFeeBrackets,
    },
    MarketCapBasedFees {
        fee_type: FeeType,
        fees: FeeBrackets,
    },
    ToggleSwapPermission(bool),
}
