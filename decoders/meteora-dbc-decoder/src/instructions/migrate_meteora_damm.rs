use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b013016b43f76d9")]
pub struct MigrateMeteoraDamm {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateMeteoraDammInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub migration_metadata: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub damm_config: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub a_vault: solana_pubkey::Pubkey,
    pub b_vault: solana_pubkey::Pubkey,
    pub a_token_vault: solana_pubkey::Pubkey,
    pub b_token_vault: solana_pubkey::Pubkey,
    pub a_vault_lp_mint: solana_pubkey::Pubkey,
    pub b_vault_lp_mint: solana_pubkey::Pubkey,
    pub a_vault_lp: solana_pubkey::Pubkey,
    pub b_vault_lp: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub virtual_pool_lp: solana_pubkey::Pubkey,
    pub protocol_token_a_fee: solana_pubkey::Pubkey,
    pub protocol_token_b_fee: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub mint_metadata: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub amm_program: solana_pubkey::Pubkey,
    pub vault_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateMeteoraDamm {
    type ArrangedAccounts = MigrateMeteoraDammInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let virtual_pool = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let damm_config = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let token_a_mint = next_account(&mut iter)?;
        let token_b_mint = next_account(&mut iter)?;
        let a_vault = next_account(&mut iter)?;
        let b_vault = next_account(&mut iter)?;
        let a_token_vault = next_account(&mut iter)?;
        let b_token_vault = next_account(&mut iter)?;
        let a_vault_lp_mint = next_account(&mut iter)?;
        let b_vault_lp_mint = next_account(&mut iter)?;
        let a_vault_lp = next_account(&mut iter)?;
        let b_vault_lp = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let virtual_pool_lp = next_account(&mut iter)?;
        let protocol_token_a_fee = next_account(&mut iter)?;
        let protocol_token_b_fee = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let mint_metadata = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;
        let amm_program = next_account(&mut iter)?;
        let vault_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(MigrateMeteoraDammInstructionAccounts {
            virtual_pool,
            migration_metadata,
            config,
            pool_authority,
            pool,
            damm_config,
            lp_mint,
            token_a_mint,
            token_b_mint,
            a_vault,
            b_vault,
            a_token_vault,
            b_token_vault,
            a_vault_lp_mint,
            b_vault_lp_mint,
            a_vault_lp,
            b_vault_lp,
            base_vault,
            quote_vault,
            virtual_pool_lp,
            protocol_token_a_fee,
            protocol_token_b_fee,
            payer,
            rent,
            mint_metadata,
            metadata_program,
            amm_program,
            vault_program,
            token_program,
            associated_token_program,
            system_program,
        })
    }
}
