use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d21156c14f1f4a783")]
pub struct UserDefinedEvent {
    pub liquidity_pool_id: solana_pubkey::Pubkey,
    pub instruction_name: String,
    pub base64_data: String,
}
