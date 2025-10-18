use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum GraduationMethod {
    None,
    Whirlpool {
        split_bps: u16,
        fee_tier_index: u16,
        destination: solana_pubkey::Pubkey,
        unlocked: bool,
    },
    Manual {
        split_bps: u16,
        destination: solana_pubkey::Pubkey,
    },
}
