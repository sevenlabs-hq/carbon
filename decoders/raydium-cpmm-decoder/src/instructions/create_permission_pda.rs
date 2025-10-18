use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x878802d889a9b5ca")]
pub struct CreatePermissionPda {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreatePermissionPdaInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub permission_authority: solana_pubkey::Pubkey,
    pub permission: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePermissionPda {
    type ArrangedAccounts = CreatePermissionPdaInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let permission_authority = next_account(&mut iter)?;
        let permission = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreatePermissionPdaInstructionAccounts {
            owner,
            permission_authority,
            permission,
            system_program,
        })
    }
}
