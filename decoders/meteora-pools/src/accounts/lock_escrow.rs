use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xbe6a7906c8b6154b")]
pub struct LockEscrow {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub escrow_vault: solana_sdk::pubkey::Pubkey,
    pub bump: u8,
    pub total_locked_amount: u64,
    pub lp_per_token: u128,
    pub unclaimed_fee_pending: u64,
    pub a_fee: u64,
    pub b_fee: u64,
}
