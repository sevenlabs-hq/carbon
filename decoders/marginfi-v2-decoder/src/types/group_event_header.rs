use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GroupEventHeader {
    pub signer: Option<solana_pubkey::Pubkey>,
    pub marginfi_group: solana_pubkey::Pubkey,
}
