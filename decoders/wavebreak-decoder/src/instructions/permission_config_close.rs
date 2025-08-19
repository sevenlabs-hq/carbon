use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04")]
pub struct PermissionConfigClose {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PermissionConfigCloseInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub permission_config: solana_pubkey::Pubkey,
    pub authority_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PermissionConfigClose {
    type ArrangedAccounts = PermissionConfigCloseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let permission_config = next_account(&mut iter)?;
        let authority_config = next_account(&mut iter)?;

        Some(PermissionConfigCloseInstructionAccounts {
            authority,
            permission_config,
            authority_config,
        })
    }
}
