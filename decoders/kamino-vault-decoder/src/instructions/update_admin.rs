

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa1b028d53cb8b3e4")]
pub struct UpdateAdmin{
}

pub struct UpdateAdminInstructionAccounts {
    pub pending_admin: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAdmin {
    type ArrangedAccounts = UpdateAdminInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let pending_admin = accounts.get(0)?;
        let vault_state = accounts.get(1)?;

        Some(UpdateAdminInstructionAccounts {
            pending_admin: pending_admin.pubkey,
            vault_state: vault_state.pubkey,
        })
    }
}