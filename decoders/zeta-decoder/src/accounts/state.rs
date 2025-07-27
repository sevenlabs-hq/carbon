use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd8926b5e684bb6b1")]
pub struct State {
    pub admin: solana_pubkey::Pubkey,
    pub state_nonce: u8,
    pub serum_nonce: u8,
    pub mint_auth_nonce: u8,
    pub num_underlyings: u8,
    pub num_flex_underlyings: u8,
    pub null: [u8; 7],
    pub strike_initialization_threshold_seconds: u32,
    pub pricing_frequency_seconds: u32,
    pub liquidator_liquidation_percentage: u32,
    pub insurance_vault_liquidation_percentage: u32,
    pub deprecated_fee_values: [u64; 3],
    pub native_deposit_limit: u64,
    pub expiration_threshold_seconds: u32,
    pub position_movement_fee_bps: u8,
    pub margin_concession_percentage: u8,
    pub treasury_wallet_nonce: u8,
    pub deprecated_option_fee_values: [u64; 2],
    pub referrals_admin: solana_pubkey::Pubkey,
    pub referrals_rewards_wallet_nonce: u8,
    pub max_perp_delta_age: u16,
    pub secondary_admin: solana_pubkey::Pubkey,
    pub vault_nonce: u8,
    pub insurance_vault_nonce: u8,
    pub deprecated_total_insurance_vault_deposits: u64,
    pub native_withdraw_limit: u64,
    pub withdraw_limit_epoch_seconds: u32,
    pub native_open_interest_limit: u64,
    pub halt_states: [HaltStateV2; 25],
    pub halt_states_padding: [HaltStateV2; 0],
    pub trigger_admin: solana_pubkey::Pubkey,
    pub min_lot_sizes: [u32; 25],
    pub min_lot_sizes_padding: [u32; 0],
    pub tick_sizes: [u32; 25],
    pub tick_sizes_padding: [u32; 0],
    pub deprecated_maker_fee_value: u64,
    pub native_take_trigger_order_fee_percentage: u64,
    pub native_maker_rebate_percentage: u64,
    pub ma_type_admin: solana_pubkey::Pubkey,
    pub pricing_admin: solana_pubkey::Pubkey,
    pub treasury_split_token_account: solana_pubkey::Pubkey,
    pub treasury_split_percentage: u8,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 185],
}
