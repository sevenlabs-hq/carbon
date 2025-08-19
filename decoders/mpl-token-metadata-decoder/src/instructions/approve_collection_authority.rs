use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17")]
pub struct ApproveCollectionAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ApproveCollectionAuthorityInstructionAccounts {
    pub collection_authority_record: solana_pubkey::Pubkey,
    pub new_collection_authority: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for ApproveCollectionAuthority {
    type ArrangedAccounts = ApproveCollectionAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let collection_authority_record = next_account(&mut iter)?;
        let new_collection_authority = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter);

        Some(ApproveCollectionAuthorityInstructionAccounts {
            collection_authority_record,
            new_collection_authority,
            update_authority,
            payer,
            metadata,
            mint,
            system_program,
            rent,
        })
    }
}
