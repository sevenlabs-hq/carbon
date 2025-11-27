use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xba3ad4ef66839d92")]
pub struct DenylistedAccount {
    pub account: solana_pubkey::Pubkey,
}
