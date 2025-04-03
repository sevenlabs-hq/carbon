use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb85717c19ceeaf77")]
pub struct UpdateGlobalConfigAdmin {}

pub struct UpdateGlobalConfigAdminInstructionAccounts {
    pub admin_authority_cached: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateGlobalConfigAdmin {
    type ArrangedAccounts = UpdateGlobalConfigAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority_cached, global_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateGlobalConfigAdminInstructionAccounts {
            admin_authority_cached: admin_authority_cached.pubkey,
            global_config: global_config.pubkey,
        })
    }
}
