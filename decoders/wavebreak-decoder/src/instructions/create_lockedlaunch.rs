use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x28")]
pub struct CreateLockedlaunch {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateLockedlaunchInstructionAccounts {
    pub creator: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub mint_config: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
    pub metaplex_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateLockedlaunch {
    type ArrangedAccounts = CreateLockedlaunchInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let creator = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let mint_config = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;
        let metaplex_program = next_account(&mut iter)?;

        Some(CreateLockedlaunchInstructionAccounts {
            creator,
            bonding_curve,
            base_mint,
            quote_mint,
            quote_vault,
            mint_config,
            metadata,
            system_program,
            base_token_program,
            quote_token_program,
            ata_program,
            metaplex_program,
        })
    }
}
