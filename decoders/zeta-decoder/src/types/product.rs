use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Product {
    pub market: solana_sdk::pubkey::Pubkey,
    pub strike: Strike,
    pub dirty: bool,
    pub kind: Kind,
}
