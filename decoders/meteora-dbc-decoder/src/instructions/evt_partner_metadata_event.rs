use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc87f06370d200896")]
pub struct EvtPartnerMetadataEvent {
    pub partner_metadata: Pubkey,
    pub fee_claimer: Pubkey,
}
