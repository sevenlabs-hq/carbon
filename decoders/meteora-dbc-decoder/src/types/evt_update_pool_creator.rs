use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtUpdatePoolCreator {
    pub pool: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub new_creator: solana_pubkey::Pubkey,
}
