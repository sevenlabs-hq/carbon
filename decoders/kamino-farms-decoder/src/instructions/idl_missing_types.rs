
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8250269950d4b6fd")]
pub struct IdlMissingTypes{
    pub global_config_option_kind: GlobalConfigOption,
    pub farm_config_option_kind: FarmConfigOption,
    pub time_unit: TimeUnit,
    pub locking_mode: LockingMode,
    pub reward_type: RewardType,
}

pub struct IdlMissingTypesInstructionAccounts {
    pub global_admin: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IdlMissingTypes {
    type ArrangedAccounts = IdlMissingTypesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            global_admin,
            global_config,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(IdlMissingTypesInstructionAccounts {
            global_admin: global_admin.pubkey,
            global_config: global_config.pubkey,
        })
    }
}