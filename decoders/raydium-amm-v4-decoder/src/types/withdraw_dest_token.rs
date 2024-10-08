
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WithdrawDestToken {
    pub withdraw_amount: u64,
    pub coin_amount: u64,
    pub pc_amount: u64,
    pub dest_token_coin: solana_sdk::pubkey::Pubkey,
    pub dest_token_pc: solana_sdk::pubkey::Pubkey,
}
