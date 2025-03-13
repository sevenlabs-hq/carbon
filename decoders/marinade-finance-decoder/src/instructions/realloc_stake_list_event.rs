use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc18110f3b183f817")]
pub struct ReallocStakeListEvent {
    pub state: solana_sdk::pubkey::Pubkey,
    pub count: u32,
    pub new_capacity: u32,
}
