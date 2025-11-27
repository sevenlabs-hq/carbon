use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1d37607f524897c5")]
pub struct ReferrerPubkeyAccount {
    pub referrer_id: [u8; 6],
}
