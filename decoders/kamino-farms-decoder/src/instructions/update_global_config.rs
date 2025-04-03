use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa45482bd6f3afac8")]
pub struct UpdateGlobalConfig {
    pub mode: u8,
    pub value: [u8; 32],
}

pub struct UpdateGlobalConfigInstructionAccounts {
    pub global_admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateGlobalConfig {
    type ArrangedAccounts = UpdateGlobalConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global_admin, global_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateGlobalConfigInstructionAccounts {
            global_admin: global_admin.pubkey,
            global_config: global_config.pubkey,
        })
    }
}
