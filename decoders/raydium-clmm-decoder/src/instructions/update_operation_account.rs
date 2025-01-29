use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7f467728bce33d07")]
pub struct UpdateOperationAccount {
    pub param: u8,
    pub keys: Vec<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateOperationAccountInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub operation_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateOperationAccount {
    type ArrangedAccounts = UpdateOperationAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let operation_state = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(UpdateOperationAccountInstructionAccounts {
            owner: owner.pubkey,
            operation_state: operation_state.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
