use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17b7f83760d8ac60")]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: solana_pubkey::Pubkey,
    pub is_mayhem_mode: bool,
}

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17b7f83760d8ac60")]
pub(super) struct BondingCurve81b {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: solana_pubkey::Pubkey,
}

impl From<BondingCurve81b> for BondingCurve {
    fn from(value: BondingCurve81b) -> Self {
        Self {
            virtual_token_reserves: value.virtual_token_reserves,
            virtual_sol_reserves: value.virtual_sol_reserves,
            real_token_reserves: value.real_token_reserves,
            real_sol_reserves: value.real_sol_reserves,
            token_total_supply: value.token_total_supply,
            complete: value.complete,
            creator: value.creator,
            is_mayhem_mode: false,
        }
    }
}
