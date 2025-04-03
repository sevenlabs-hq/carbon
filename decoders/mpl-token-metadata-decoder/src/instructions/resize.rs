use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x38")]
pub struct Resize {}

pub struct ResizeInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Resize {
    type ArrangedAccounts = ResizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, edition, mint, payer, authority, token, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ResizeInstructionAccounts {
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            mint: mint.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            token: token.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
