

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x83a6405e6cd572b7")]
pub struct DepositToFarmVault{
    pub amount: u64,
}

pub struct DepositToFarmVaultInstructionAccounts {
    pub depositor: solana_sdk::pubkey::Pubkey,
    pub farm_state: solana_sdk::pubkey::Pubkey,
    pub farm_vault: solana_sdk::pubkey::Pubkey,
    pub depositor_ata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositToFarmVault {
    type ArrangedAccounts = DepositToFarmVaultInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            depositor,
            farm_state,
            farm_vault,
            depositor_ata,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(DepositToFarmVaultInstructionAccounts {
            depositor: depositor.pubkey,
            farm_state: farm_state.pubkey,
            farm_vault: farm_vault.pubkey,
            depositor_ata: depositor_ata.pubkey,
            token_program: token_program.pubkey,
        })
    }
}