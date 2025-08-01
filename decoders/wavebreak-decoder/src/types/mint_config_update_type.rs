use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum MintConfigUpdateType {
    DefaultCreatorReward(u64),
    DefaultGraduationReward(u64),
    DefaultGraduationTarget(u64),
    DefaultMaxBuyAmount(u64),
    DefaultMaxSellAmount(u64),
    DefaultBuyPermissionBitmap(Option<[u8; 32]>),
    DefaultSellPermissionBitmap(Option<[u8; 32]>),
    CreatePermissionBitmap(Option<[u8; 32]>),
    PriceCurve(u128, u128, [u16; 4]),
    DefaultSwapFeeBps(u16),
    DefaultBaseFeeBps(u16),
    DefaultQuoteFeeBps(u16),
}
