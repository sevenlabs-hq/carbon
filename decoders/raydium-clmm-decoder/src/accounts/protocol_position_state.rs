use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x64e2916392daa06a")]
pub struct ProtocolPositionState {
    pub bump: u8,
    pub pool_id: solana_pubkey::Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub fee_growth_inside0_last_x64: u128,
    pub fee_growth_inside1_last_x64: u128,
    pub token_fees_owed0: u64,
    pub token_fees_owed1: u64,
    pub reward_growth_inside: [u128; 3],
    pub recent_epoch: u64,
    pub padding: [u64; 7],
}
