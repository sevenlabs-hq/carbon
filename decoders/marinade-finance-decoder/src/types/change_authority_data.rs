

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ChangeAuthorityData {
    pub admin: Option<solana_sdk::pubkey::Pubkey>,
    pub validator_manager: Option<solana_sdk::pubkey::Pubkey>,
    pub operational_sol_account: Option<solana_sdk::pubkey::Pubkey>,
    pub treasury_msol_account: Option<solana_sdk::pubkey::Pubkey>,
    pub pause_authority: Option<solana_sdk::pubkey::Pubkey>,
}
