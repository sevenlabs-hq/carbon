use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UserDefinedEvent {
    pub liquidity_pool_id: solana_pubkey::Pubkey,
    pub instruction_name: String,
    pub base64_data: String,
}
