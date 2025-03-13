use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3ec7514e210dec3d")]
pub struct MarginfiGroupConfigure {
    pub config: GroupConfig,
}

pub struct MarginfiGroupConfigureInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MarginfiGroupConfigure {
    type ArrangedAccounts = MarginfiGroupConfigureInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MarginfiGroupConfigureInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
        })
    }
}
