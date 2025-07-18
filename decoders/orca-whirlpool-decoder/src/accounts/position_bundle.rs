use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x81a9af41b95f2064")]
pub struct PositionBundle {
    pub position_bundle_mint: solana_pubkey::Pubkey,
    pub position_bitmap: [u8; 32],
}
