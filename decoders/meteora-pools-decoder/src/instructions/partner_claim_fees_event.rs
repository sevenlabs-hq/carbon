use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d87830a5e77d1ca30")]
pub struct PartnerClaimFeesEvent {
    pub pool: solana_pubkey::Pubkey,
    pub fee_a: u64,
    pub fee_b: u64,
    pub partner: solana_pubkey::Pubkey,
}
