use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09")]
pub struct DeprecatedMintPrintingTokens {}

pub struct DeprecatedMintPrintingTokensInstructionAccounts {
    pub destination: solana_sdk::pubkey::Pubkey,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedMintPrintingTokens {
    type ArrangedAccounts = DeprecatedMintPrintingTokensInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [destination, printing_mint, update_authority, metadata, master_edition, token_program, rent] =
            accounts
        else {
            return None;
        };

        Some(DeprecatedMintPrintingTokensInstructionAccounts {
            destination: destination.pubkey,
            printing_mint: printing_mint.pubkey,
            update_authority: update_authority.pubkey,
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
