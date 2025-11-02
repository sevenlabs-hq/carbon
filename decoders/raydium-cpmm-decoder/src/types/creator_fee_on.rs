use alloc::format;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum CreatorFeeOn {
    BothToken,
    OnlyToken0,
    OnlyToken1,
}
