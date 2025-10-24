use crate::types::SplitAmountInfo;
use crate::types::SplitPositionInfo;
use crate::types::SplitPositionParameters2;
use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da520cbae4864e967")]
pub struct EvtSplitPosition2Event {
    pub pool: Pubkey,
    pub first_owner: Pubkey,
    pub second_owner: Pubkey,
    pub first_position: Pubkey,
    pub second_position: Pubkey,
    pub current_sqrt_price: u128,
    pub amount_splits: SplitAmountInfo,
    pub first_position_info: SplitPositionInfo,
    pub second_position_info: SplitPositionInfo,
    pub split_position_parameters: SplitPositionParameters2,
}
