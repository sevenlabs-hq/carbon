use alloc::format;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum MarginAccountType {
    Normal,
    MarketMaker,
    MarketMakerT1,
    MarketMakerT0,
    MarketMakerT2,
    MarketMakerT3,
    MarketMakerT4,
    MarketMakerT5,
    MarketMakerT6,
    MarketMakerT7,
    MarketMakerT8,
    MarketMakerT9,
    NormalT1,
    NormalT2,
    NormalT3,
    NormalT4,
    NormalT5,
    NormalT6,
    NormalT7,
    NormalT8,
    NormalT9,
    WithdrawOnly,
}
