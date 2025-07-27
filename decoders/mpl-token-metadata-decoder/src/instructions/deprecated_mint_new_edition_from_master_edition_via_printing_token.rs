use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub master_edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub printing_mint: solana_pubkey::Pubkey,
    pub master_token_account: solana_pubkey::Pubkey,
    pub edition_marker: solana_pubkey::Pubkey,
    pub burn_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub master_update_authority: solana_pubkey::Pubkey,
    pub master_metadata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub reservation_list: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts
    for DeprecatedMintNewEditionFromMasterEditionViaPrintingToken
{
    type ArrangedAccounts =
        DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let edition = next_account(&mut iter)?;
        let master_edition = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let mint_authority = next_account(&mut iter)?;
        let printing_mint = next_account(&mut iter)?;
        let master_token_account = next_account(&mut iter)?;
        let edition_marker = next_account(&mut iter)?;
        let burn_authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let master_update_authority = next_account(&mut iter)?;
        let master_metadata = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let reservation_list = next_account(&mut iter);

        Some(
            DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts {
                metadata,
                edition,
                master_edition,
                mint,
                mint_authority,
                printing_mint,
                master_token_account,
                edition_marker,
                burn_authority,
                payer,
                master_update_authority,
                master_metadata,
                token_program,
                system_program,
                rent,
                reservation_list,
            },
        )
    }
}
