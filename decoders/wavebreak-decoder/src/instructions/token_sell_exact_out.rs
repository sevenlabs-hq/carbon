use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b")]
pub struct TokenSellExactOut {
    pub amount_out: u64,
    pub allow_partial_fill: bool,
    pub price_threshold: Option<(u64, u64)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TokenSellExactOutInstructionAccounts {
    pub seller: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub base_ata: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub quote_ata: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TokenSellExactOut {
    type ArrangedAccounts = TokenSellExactOutInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let seller = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let base_ata = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let quote_ata = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;

        Some(TokenSellExactOutInstructionAccounts {
            seller,
            bonding_curve,
            base_mint,
            base_ata,
            quote_mint,
            quote_vault,
            quote_ata,
            system_program,
            ata_program,
            base_token_program,
            quote_token_program,
        })
    }
}
