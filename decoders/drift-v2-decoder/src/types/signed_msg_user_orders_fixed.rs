use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SignedMsgUserOrdersFixed {
    pub user_pubkey: solana_pubkey::Pubkey,
    pub padding: u32,
    pub len: u32,
}
