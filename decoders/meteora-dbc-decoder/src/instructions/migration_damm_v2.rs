use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ca9e66735e45040")]
pub struct MigrationDammV2 {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrationDammV2InstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub migration_metadata: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub first_position_nft_mint: solana_pubkey::Pubkey,
    pub first_position_nft_account: solana_pubkey::Pubkey,
    pub first_position: solana_pubkey::Pubkey,
    pub second_position_nft_mint: solana_pubkey::Pubkey,
    pub second_position_nft_account: solana_pubkey::Pubkey,
    pub second_position: solana_pubkey::Pubkey,
    pub damm_pool_authority: solana_pubkey::Pubkey,
    pub amm_program: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub damm_event_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrationDammV2 {
    type ArrangedAccounts = MigrationDammV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let virtual_pool = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let first_position_nft_mint = next_account(&mut iter)?;
        let first_position_nft_account = next_account(&mut iter)?;
        let first_position = next_account(&mut iter)?;
        let second_position_nft_mint = next_account(&mut iter)?;
        let second_position_nft_account = next_account(&mut iter)?;
        let second_position = next_account(&mut iter)?;
        let damm_pool_authority = next_account(&mut iter)?;
        let amm_program = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_a_vault = next_account(&mut iter)?;
        let token_b_vault = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let token_2022_program = next_account(&mut iter)?;
        let damm_event_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(MigrationDammV2InstructionAccounts {
            virtual_pool,
            migration_metadata,
            config,
            pool_authority,
            pool,
            first_position_nft_mint,
            first_position_nft_account,
            first_position,
            second_position_nft_mint,
            second_position_nft_account,
            second_position,
            damm_pool_authority,
            amm_program,
            base_mint,
            quote_mint,
            token_a_vault,
            token_b_vault,
            base_vault,
            quote_vault,
            payer,
            token_base_program,
            token_quote_program,
            token_2022_program,
            damm_event_authority,
            system_program,
        })
    }
}
