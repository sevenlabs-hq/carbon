use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9d1431e0d957c1fe")]
pub struct WhirlpoolsConfig {
    pub fee_authority: solana_pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_pubkey::Pubkey,
    pub default_protocol_fee_rate: u16,
}
