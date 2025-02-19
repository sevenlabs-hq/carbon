use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0deca4ad6afda4b9")]
pub struct ConfigInit {
    pub data: ConfigParams,
}

pub struct ConfigInitInstructionAccounts {
    pub config_authority: solana_sdk::pubkey::Pubkey,
    pub config_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigInit {
    type ArrangedAccounts = ConfigInitInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config_authority, config_account, system_program] = accounts else {
            return None;
        };

        Some(ConfigInitInstructionAccounts {
            config_authority: config_authority.pubkey,
            config_account: config_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
