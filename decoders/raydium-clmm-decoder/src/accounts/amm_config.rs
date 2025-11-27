use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdaf42168cbcb2b6f")]
pub struct AmmConfig {
    pub bump: u8,
    pub index: u16,
    pub owner: solana_pubkey::Pubkey,
    pub protocol_fee_rate: u32,
    pub trade_fee_rate: u32,
    pub tick_spacing: u16,
    pub fund_fee_rate: u32,
    pub padding_u32: u32,
    pub fund_owner: solana_pubkey::Pubkey,
    pub padding: [u64; 3],
}
