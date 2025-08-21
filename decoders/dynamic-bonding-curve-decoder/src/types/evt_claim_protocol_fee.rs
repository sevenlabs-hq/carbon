use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtClaimProtocolFee {
    pub pool: solana_pubkey::Pubkey,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}
