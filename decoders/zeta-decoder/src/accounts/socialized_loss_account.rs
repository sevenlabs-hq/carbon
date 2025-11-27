use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x41fe8deb3c546889")]
pub struct SocializedLossAccount {
    pub nonce: u8,
    pub overbankrupt_amount: u64,
}
