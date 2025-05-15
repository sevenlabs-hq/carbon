use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d8ecb06207f69bfa2")]
pub struct SetMetaplexCreatorEvent {
    pub timestamp: i64,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}
