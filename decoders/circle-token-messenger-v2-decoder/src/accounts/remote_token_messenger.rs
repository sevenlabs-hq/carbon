use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6973ae225fe98afc")]
pub struct RemoteTokenMessenger {
    pub domain: u32,
    pub token_messenger: solana_pubkey::Pubkey,
}
