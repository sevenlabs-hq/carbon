use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1a5375f05cca70fe")]
pub struct EvtClaimTradingFeeEvent {
    pub pool: solana_pubkey::Pubkey,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}
