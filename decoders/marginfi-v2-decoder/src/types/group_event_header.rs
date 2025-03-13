use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GroupEventHeader {
    pub signer: Option<solana_sdk::pubkey::Pubkey>,
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
}
