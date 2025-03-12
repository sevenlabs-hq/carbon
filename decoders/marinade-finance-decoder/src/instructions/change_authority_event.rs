
use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1de46f2318bb4ee08a")]
pub struct ChangeAuthorityEvent{
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin_change: Option<PubkeyValueChange>,
    pub validator_manager_change: Option<PubkeyValueChange>,
    pub operational_sol_account_change: Option<PubkeyValueChange>,
    pub treasury_msol_account_change: Option<PubkeyValueChange>,
    pub pause_authority_change: Option<PubkeyValueChange>,
}
