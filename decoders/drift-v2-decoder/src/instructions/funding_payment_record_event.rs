use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d083b601489c9385f")]
pub struct FundingPaymentRecordEvent {
    pub ts: i64,
    pub user_authority: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub market_index: u16,
    pub funding_payment: i64,
    pub base_asset_amount: i64,
    pub user_last_cumulative_funding: i64,
    pub amm_cumulative_funding_long: i128,
    pub amm_cumulative_funding_short: i128,
}
