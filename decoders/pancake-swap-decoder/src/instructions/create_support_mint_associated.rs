use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11fb415c88f20ea9")]
pub struct CreateSupportMintAssociated {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateSupportMintAssociatedInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub support_mint_associated: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateSupportMintAssociated {
    type ArrangedAccounts = CreateSupportMintAssociatedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, token_mint, support_mint_associated, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateSupportMintAssociatedInstructionAccounts {
            owner: owner.pubkey,
            token_mint: token_mint.pubkey,
            support_mint_associated: support_mint_associated.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
