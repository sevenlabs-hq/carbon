use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf19a6d0411b16dbc")]
pub struct Pool {
    pub lp_mint: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub a_vault: solana_pubkey::Pubkey,
    pub b_vault: solana_pubkey::Pubkey,
    pub a_vault_lp: solana_pubkey::Pubkey,
    pub b_vault_lp: solana_pubkey::Pubkey,
    pub a_vault_lp_bump: u8,
    pub enabled: bool,
    pub protocol_token_a_fee: solana_pubkey::Pubkey,
    pub protocol_token_b_fee: solana_pubkey::Pubkey,
    pub fee_last_updated_at: u64,
    pub padding0: [u8; 24],
    pub fees: PoolFees,
    pub pool_type: PoolType,
    pub stake: solana_pubkey::Pubkey,
    pub total_locked_lp: u64,
    pub bootstrapping: Bootstrapping,
    pub partner_info: PartnerInfo,
    pub padding: Padding,
    pub curve_type: CurveType,
}
