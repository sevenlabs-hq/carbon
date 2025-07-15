use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x25")]
pub struct BurnEditionNft {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BurnEditionNftInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub print_edition_mint: solana_pubkey::Pubkey,
    pub master_edition_mint: solana_pubkey::Pubkey,
    pub print_edition_token_account: solana_pubkey::Pubkey,
    pub master_edition_token_account: solana_pubkey::Pubkey,
    pub master_edition_account: solana_pubkey::Pubkey,
    pub print_edition_account: solana_pubkey::Pubkey,
    pub edition_marker_account: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnEditionNft {
    type ArrangedAccounts = BurnEditionNftInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let print_edition_mint = next_account(&mut iter)?;
        let master_edition_mint = next_account(&mut iter)?;
        let print_edition_token_account = next_account(&mut iter)?;
        let master_edition_token_account = next_account(&mut iter)?;
        let master_edition_account = next_account(&mut iter)?;
        let print_edition_account = next_account(&mut iter)?;
        let edition_marker_account = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter)?;

        Some(BurnEditionNftInstructionAccounts {
            metadata,
            owner,
            print_edition_mint,
            master_edition_mint,
            print_edition_token_account,
            master_edition_token_account,
            master_edition_account,
            print_edition_account,
            edition_marker_account,
            spl_token_program,
        })
    }
}
