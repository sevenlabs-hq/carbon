use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d97d7e20976a173ae")]
pub struct PoolCreateEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
}
