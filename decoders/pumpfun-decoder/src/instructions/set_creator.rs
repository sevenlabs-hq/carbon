use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfe94ff70cf8eaaa5")]
pub struct SetCreator {
    pub creator: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetCreatorInstructionAccounts {
    pub set_creator_authority: solana_pubkey::Pubkey,
    pub global: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCreator {
    type ArrangedAccounts = SetCreatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let set_creator_authority = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SetCreatorInstructionAccounts {
            set_creator_authority,
            global,
            mint,
            metadata,
            bonding_curve,
            event_authority,
            program,
        })
    }
}
