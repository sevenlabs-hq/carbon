use carbon_core::{borsh, deserialize::PrefixString, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Default,
)]
pub struct TokenMintParams {
    pub name: PrefixString,
    pub symbol: PrefixString,
    pub uri: PrefixString,
    pub decimals: u8,
    pub collateral_currency: u8,
    pub amount: u64,
    pub curve_type: u8,
    pub migration_target: u8,
    pub _padding: [u8; 10],
}
