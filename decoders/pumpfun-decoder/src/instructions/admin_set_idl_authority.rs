use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08d960e79068c005")]
pub struct AdminSetIdlAuthority {
    pub idl_authority: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminSetIdlAuthorityInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub global: solana_pubkey::Pubkey,
    pub idl_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub program_signer: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminSetIdlAuthority {
    type ArrangedAccounts = AdminSetIdlAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let idl_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let program_signer = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(AdminSetIdlAuthorityInstructionAccounts {
            authority,
            global,
            idl_account,
            system_program,
            program_signer,
            event_authority,
            program,
        })
    }
}
