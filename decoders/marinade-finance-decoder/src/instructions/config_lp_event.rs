use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9fccc08a4491e094")]
pub struct ConfigLpEvent {
    pub state: solana_pubkey::Pubkey,
    pub min_fee_change: Option<FeeValueChange>,
    pub max_fee_change: Option<FeeValueChange>,
    pub liquidity_target_change: Option<U64ValueChange>,
    pub treasury_cut_change: Option<FeeValueChange>,
}
