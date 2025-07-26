use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09")]
pub struct DeprecatedMintPrintingTokens {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeprecatedMintPrintingTokensInstructionAccounts {
    pub destination: solana_pubkey::Pubkey,
    pub printing_mint: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub master_edition: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedMintPrintingTokens {
    type ArrangedAccounts = DeprecatedMintPrintingTokensInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let destination = next_account(&mut iter)?;
        let printing_mint = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let master_edition = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(DeprecatedMintPrintingTokensInstructionAccounts {
            destination,
            printing_mint,
            update_authority,
            metadata,
            master_edition,
            token_program,
            rent,
        })
    }
}
