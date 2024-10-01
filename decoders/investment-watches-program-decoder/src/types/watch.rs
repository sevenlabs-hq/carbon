
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Watch {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub redeem_price: Option<u64>,
    pub redeem_price_per_share: u64,
    pub supply: u64,
    pub redeemed_at: Option<i64>,
    pub bump: u8,
}
