use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1d")]
pub struct BurnNft {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BurnNftInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
    pub master_edition_account: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
    pub collection_metadata: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnNft {
    type ArrangedAccounts = BurnNftInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_account = next_account(&mut iter)?;
        let master_edition_account = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter)?;
        let collection_metadata = next_account(&mut iter);

        Some(BurnNftInstructionAccounts {
            metadata,
            owner,
            mint,
            token_account,
            master_edition_account,
            spl_token_program,
            collection_metadata,
        })
    }
}
