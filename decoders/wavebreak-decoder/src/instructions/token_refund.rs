use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c")]
pub struct TokenRefund {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TokenRefundInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub signer_quote_ata: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub signer_base_ata: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TokenRefund {
    type ArrangedAccounts = TokenRefundInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let signer = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let signer_quote_ata = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let signer_base_ata = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;

        Some(TokenRefundInstructionAccounts {
            signer,
            bonding_curve,
            quote_mint,
            quote_vault,
            signer_quote_ata,
            base_mint,
            signer_base_ata,
            system_program,
            base_token_program,
            quote_token_program,
            ata_program,
        })
    }
}
