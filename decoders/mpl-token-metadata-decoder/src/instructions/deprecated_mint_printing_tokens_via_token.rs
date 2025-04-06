use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct DeprecatedMintPrintingTokensViaToken {}

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
        let [destination, token, one_time_printing_authorization_mint, printing_mint, burn_authority, metadata, master_edition, token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DeprecatedMintPrintingTokensViaTokenInstructionAccounts {
            destination: destination.pubkey,
            token: token.pubkey,
            one_time_printing_authorization_mint: one_time_printing_authorization_mint.pubkey,
            printing_mint: printing_mint.pubkey,
            burn_authority: burn_authority.pubkey,
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
