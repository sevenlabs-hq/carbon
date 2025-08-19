use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x61d8698371f68e8d")]
pub struct UpdateSpotFeeStructure {
    pub fee_structure: FeeStructure,
}

pub struct UpdateSpotFeeStructureInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotFeeStructure {
    type ArrangedAccounts = UpdateSpotFeeStructureInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotFeeStructureInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
