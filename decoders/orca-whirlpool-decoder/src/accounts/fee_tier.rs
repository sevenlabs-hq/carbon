use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x384b9f4c8e44be69")]
pub struct FeeTier {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}
