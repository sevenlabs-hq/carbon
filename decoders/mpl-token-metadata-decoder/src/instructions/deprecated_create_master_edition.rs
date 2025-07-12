use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct DeprecatedCreateMasterEdition {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeprecatedCreateMasterEditionInstructionAccounts {
    pub edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub printing_mint: solana_pubkey::Pubkey,
    pub one_time_printing_authorization_mint: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub printing_mint_authority: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub one_time_printing_authorization_mint_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedCreateMasterEdition {
    type ArrangedAccounts = DeprecatedCreateMasterEditionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let edition = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let printing_mint = next_account(&mut iter)?;
        let one_time_printing_authorization_mint = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let printing_mint_authority = next_account(&mut iter)?;
        let mint_authority = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let one_time_printing_authorization_mint_authority = next_account(&mut iter)?;

        Some(DeprecatedCreateMasterEditionInstructionAccounts {
            edition,
            mint,
            printing_mint,
            one_time_printing_authorization_mint,
            update_authority,
            printing_mint_authority,
            mint_authority,
            metadata,
            payer,
            token_program,
            system_program,
            rent,
            one_time_printing_authorization_mint_authority,
        })
    }
}
