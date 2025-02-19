use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ConfigParams {
    pub migration_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub backend_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub config_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub helio_fee: Option<solana_sdk::pubkey::Pubkey>,
    pub dex_fee: Option<solana_sdk::pubkey::Pubkey>,
    pub fee_bps: Option<u16>,
    pub dex_fee_share: Option<u8>,
    pub migration_fee: Option<u64>,
    pub marketcap_threshold: Option<u64>,
    pub marketcap_currency: Option<u8>,
    pub min_supported_decimal_places: Option<u8>,
    pub max_supported_decimal_places: Option<u8>,
    pub min_supported_token_supply: Option<u64>,
    pub max_supported_token_supply: Option<u64>,
    pub coef_b: Option<u32>,
}
