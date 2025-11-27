use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x142588137aef2482")]
pub struct UpdateFarmAdmin {}

pub struct UpdateFarmAdminInstructionAccounts {
    pub pending_farm_admin: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFarmAdmin {
    type ArrangedAccounts = UpdateFarmAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pending_farm_admin, farm_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFarmAdminInstructionAccounts {
            pending_farm_admin: pending_farm_admin.pubkey,
            farm_state: farm_state.pubkey,
        })
    }
}
