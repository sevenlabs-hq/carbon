use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9a249ec420a37b7e")]
pub struct ReportLossEvent {
    pub strategy: solana_pubkey::Pubkey,
    pub loss: u64,
}
