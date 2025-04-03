use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbee7aaf40ee38142")]
pub struct AddValidatorEvent {
    pub state: solana_pubkey::Pubkey,
    pub validator: solana_pubkey::Pubkey,
    pub index: u32,
    pub score: u32,
}
