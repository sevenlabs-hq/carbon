use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x967036e9eea10756")]
pub struct TogglePermissionlessFarmSwitch {
    pub is_on: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TogglePermissionlessFarmSwitchInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub permissionless_farm_switch: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TogglePermissionlessFarmSwitch {
    type ArrangedAccounts = TogglePermissionlessFarmSwitchInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, permissionless_farm_switch, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TogglePermissionlessFarmSwitchInstructionAccounts {
            owner: owner.pubkey,
            permissionless_farm_switch: permissionless_farm_switch.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
