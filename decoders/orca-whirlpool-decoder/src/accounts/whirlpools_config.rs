use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x9d1431e0d957c1fe")]
pub struct WhirlpoolsConfig {
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_sdk::pubkey::Pubkey,
    pub default_protocol_fee_rate: u16,
}
