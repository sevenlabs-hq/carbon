use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe4c452a562d2eb98")]
pub struct VaultState {
    pub admin_authority: solana_pubkey::Pubkey,
    pub base_vault_authority: solana_pubkey::Pubkey,
    pub base_vault_authority_bump: u64,
    pub token_mint: solana_pubkey::Pubkey,
    pub token_mint_decimals: u64,
    pub token_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub shares_mint: solana_pubkey::Pubkey,
    pub shares_mint_decimals: u64,
    pub token_available: u64,
    pub shares_issued: u64,
    pub available_crank_funds: u64,
    pub padding0: u64,
    pub performance_fee_bps: u64,
    pub management_fee_bps: u64,
    pub last_fee_charge_timestamp: u64,
    pub prev_aum_sf: u128,
    pub pending_fees_sf: u128,
    pub vault_allocation_strategy: [VaultAllocation; 10],
    pub min_deposit_amount: u64,
    pub min_withdraw_amount: u64,
    pub min_invest_amount: u64,
    pub min_invest_delay_slots: u64,
    pub crank_fund_fee_per_reserve: u64,
    pub pending_admin: solana_pubkey::Pubkey,
    pub cumulative_earned_interest_sf: u128,
    pub cumulative_mgmt_fees_sf: u128,
    pub cumulative_perf_fees_sf: u128,
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 40],
    pub vault_lookup_table: solana_pubkey::Pubkey,
    pub vault_farm: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding2: [u128; 245],
}
