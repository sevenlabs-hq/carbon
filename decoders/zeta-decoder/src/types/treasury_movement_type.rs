use alloc::format;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum TreasuryMovementType {
    Undefined,
    ToTreasuryFromInsurance,
    ToInsuranceFromTreasury,
    ToTreasuryFromReferralsRewards,
    ToReferralsRewardsFromTreasury,
}
