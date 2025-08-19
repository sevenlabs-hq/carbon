use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct DeprecatedMintPrintingTokensViaToken {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeprecatedMintPrintingTokensViaTokenInstructionAccounts {
    pub destination: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub one_time_printing_authorization_mint: solana_pubkey::Pubkey,
    pub printing_mint: solana_pubkey::Pubkey,
    pub burn_authority: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub master_edition: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedMintPrintingTokensViaToken {
    type ArrangedAccounts = DeprecatedMintPrintingTokensViaTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let destination = next_account(&mut iter)?;
        let token = next_account(&mut iter)?;
        let one_time_printing_authorization_mint = next_account(&mut iter)?;
        let printing_mint = next_account(&mut iter)?;
        let burn_authority = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let master_edition = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(DeprecatedMintPrintingTokensViaTokenInstructionAccounts {
            destination,
            token,
            one_time_printing_authorization_mint,
            printing_mint,
            burn_authority,
            metadata,
            master_edition,
            token_program,
            rent,
        })
    }
}
