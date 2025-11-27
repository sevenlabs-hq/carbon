use alloc::format;
use carbon_core::{CarbonDeserialize, borsh};

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
