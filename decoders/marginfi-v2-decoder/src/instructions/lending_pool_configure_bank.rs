
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x79ad9c285d9438ed")]
pub struct LendingPoolConfigureBank{
    pub bank_config_opt: BankConfigOpt,
}

pub struct LendingPoolConfigureBankInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub bank: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolConfigureBank {
    type ArrangedAccounts = LendingPoolConfigureBankInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            admin,
            bank,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolConfigureBankInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            bank: bank.pubkey,
        })
    }
}