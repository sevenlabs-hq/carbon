use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LpChangeEvent {
    pub pool_id: solana_pubkey::Pubkey,
    pub lp_amount_before: u64,
    pub token_0_vault_before: u64,
    pub token_1_vault_before: u64,
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub token_0_transfer_fee: u64,
    pub token_1_transfer_fee: u64,
    pub change_type: u8,
}
