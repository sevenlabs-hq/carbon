use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ProtocolFeeRecipientParams {
    pub recipient: solana_pubkey::Pubkey,
    pub shares: u64,
}
