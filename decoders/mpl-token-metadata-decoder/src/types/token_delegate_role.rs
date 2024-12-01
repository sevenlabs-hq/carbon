

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum TokenDelegateRole {
    Sale,
    Transfer,
    Utility,
    Staking,
    Standard,
    LockedTransfer,
    Migration,
}


