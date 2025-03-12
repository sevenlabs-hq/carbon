

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa45482bd6f3afac8")]
pub struct UpdateGlobalConfig{
    pub mode: u16,
    pub value: [u8; 128],
}

pub struct UpdateGlobalConfigInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateGlobalConfig {
    type ArrangedAccounts = UpdateGlobalConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            admin_authority,
            global_config,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(UpdateGlobalConfigInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
        })
    }
}