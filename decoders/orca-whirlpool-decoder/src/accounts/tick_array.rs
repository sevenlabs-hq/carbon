use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4561bdbe6e0742bb")]
pub struct TickArray {
    pub start_tick_index: i32,
    #[serde(with = "serde_big_array::BigArray")]
    pub ticks: [Tick; 88],
    pub whirlpool: solana_pubkey::Pubkey,
}
