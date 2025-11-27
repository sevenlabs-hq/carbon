use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb85717c19ceeaf77")]
pub struct UpdateGlobalConfigAdmin {}

pub struct UpdateGlobalConfigAdminInstructionAccounts {
    pub pending_global_admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateGlobalConfigAdmin {
    type ArrangedAccounts = UpdateGlobalConfigAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pending_global_admin, global_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateGlobalConfigAdminInstructionAccounts {
            pending_global_admin: pending_global_admin.pubkey,
            global_config: global_config.pubkey,
        })
    }
}
