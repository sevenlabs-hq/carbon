use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CnftArgs {
    pub cnft_root: [u8; 32],
    pub cnft_data_hash: [u8; 32],
    pub cnft_creator_hash: [u8; 32],
    pub cnft_nonce: u64,
    pub cnft_index: u32,
}
