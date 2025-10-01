use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RemoteTokenMessengerAdded {
    pub domain: u32,
    pub token_messenger: solana_pubkey::Pubkey,
}
