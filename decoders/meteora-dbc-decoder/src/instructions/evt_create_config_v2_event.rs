use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

use crate::types::ConfigParameters;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da34a42bb77c31a90")]
pub struct EvtCreateConfigV2Event {
    pub config: Pubkey,
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub leftover_receiver: Pubkey,
    pub config_parameters: ConfigParameters,
}
