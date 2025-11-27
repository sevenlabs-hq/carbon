use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dceafa9d0f1d223dd")]
pub struct InitializeEvent {
    pub state: solana_pubkey::Pubkey,
    pub params: InitializeData,
    pub stake_list: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub msol_mint: solana_pubkey::Pubkey,
    pub operational_sol_account: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub lp_msol_leg: solana_pubkey::Pubkey,
    pub treasury_msol_account: solana_pubkey::Pubkey,
}
