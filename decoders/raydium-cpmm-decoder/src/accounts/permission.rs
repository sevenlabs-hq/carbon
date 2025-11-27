use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe0531c4f0afda11c")]
pub struct Permission {
    pub authority: solana_pubkey::Pubkey,
    pub padding: [u64; 30],
}
