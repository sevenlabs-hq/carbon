
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct MicroShare {
    pub watch: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub microshare_asset: solana_sdk::pubkey::Pubkey,
    pub bump: u8,
}
