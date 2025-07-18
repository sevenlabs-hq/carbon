use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8ff5c8114ad6c487")]
pub struct Amm {
    pub initializer_key: solana_pubkey::Pubkey,
    pub initializer_deposit_token_account: solana_pubkey::Pubkey,
    pub initializer_receive_token_account: solana_pubkey::Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub freeze_trade: u8,
    pub freeze_deposit: u8,
    pub freeze_withdraw: u8,
    pub base_decimals: u8,
    pub token_program_id: solana_pubkey::Pubkey,
    pub token_a_account: solana_pubkey::Pubkey,
    pub token_b_account: solana_pubkey::Pubkey,
    pub pool_mint: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub fee_account: solana_pubkey::Pubkey,
    pub oracle_main_account: solana_pubkey::Pubkey,
    pub oracle_sub_account: solana_pubkey::Pubkey,
    pub oracle_pc_account: solana_pubkey::Pubkey,
    pub fees: AmmFees,
    pub curve: AmmCurve,
    pub config: AmmConfig,
    pub amm_p_temp1: solana_pubkey::Pubkey,
    pub amm_p_temp2: solana_pubkey::Pubkey,
    pub amm_p_temp3: solana_pubkey::Pubkey,
    pub amm_p_temp4: solana_pubkey::Pubkey,
    pub amm_p_temp5: solana_pubkey::Pubkey,
}
