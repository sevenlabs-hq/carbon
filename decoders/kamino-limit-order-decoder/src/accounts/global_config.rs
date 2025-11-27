use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x95089ccaa0fcb0d9")]
pub struct GlobalConfig {
    pub emergency_mode: u8,
    pub flash_take_order_blocked: u8,
    pub new_orders_blocked: u8,
    pub orders_taking_blocked: u8,
    pub host_fee_bps: u16,
    pub is_order_taking_permissionless: u8,
    pub padding0: [u8; 1],
    pub order_close_delay_seconds: u64,
    pub padding1: [u64; 9],
    pub pda_authority_previous_lamports_balance: u64,
    pub total_tip_amount: u64,
    pub host_tip_amount: u64,
    pub pda_authority: solana_pubkey::Pubkey,
    pub pda_authority_bump: u64,
    pub admin_authority: solana_pubkey::Pubkey,
    pub admin_authority_cached: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding2: [u64; 243],
}
