use {
    alloc::vec::Vec,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7f467728bce33d07")]
pub struct UpdateOperationAccount {
    pub param: u8,
    pub keys: Vec<solana_pubkey::Pubkey>,
}

pub struct UpdateOperationAccountInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub operation_state: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateOperationAccount {
    type ArrangedAccounts = UpdateOperationAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, operation_state, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOperationAccountInstructionAccounts {
            owner: owner.pubkey,
            operation_state: operation_state.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
