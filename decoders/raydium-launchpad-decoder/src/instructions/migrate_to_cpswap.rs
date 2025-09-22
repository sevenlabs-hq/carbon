use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x885cc8671cda908c")]
pub struct MigrateToCpswap {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateToCpswapInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub cpswap_program: solana_pubkey::Pubkey,
    pub cpswap_pool: solana_pubkey::Pubkey,
    pub cpswap_authority: solana_pubkey::Pubkey,
    pub cpswap_lp_mint: solana_pubkey::Pubkey,
    pub cpswap_base_vault: solana_pubkey::Pubkey,
    pub cpswap_quote_vault: solana_pubkey::Pubkey,
    pub cpswap_config: solana_pubkey::Pubkey,
    pub cpswap_create_pool_fee: solana_pubkey::Pubkey,
    pub cpswap_observation: solana_pubkey::Pubkey,
    pub lock_program: solana_pubkey::Pubkey,
    pub lock_authority: solana_pubkey::Pubkey,
    pub lock_lp_vault: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub pool_lp_token: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateToCpswap {
    type ArrangedAccounts = MigrateToCpswapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let cpswap_program = next_account(&mut iter)?;
        let cpswap_pool = next_account(&mut iter)?;
        let cpswap_authority = next_account(&mut iter)?;
        let cpswap_lp_mint = next_account(&mut iter)?;
        let cpswap_base_vault = next_account(&mut iter)?;
        let cpswap_quote_vault = next_account(&mut iter)?;
        let cpswap_config = next_account(&mut iter)?;
        let cpswap_create_pool_fee = next_account(&mut iter)?;
        let cpswap_observation = next_account(&mut iter)?;
        let lock_program = next_account(&mut iter)?;
        let lock_authority = next_account(&mut iter)?;
        let lock_lp_vault = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let pool_lp_token = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent_program = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;

        Some(MigrateToCpswapInstructionAccounts {
            payer,
            base_mint,
            quote_mint,
            platform_config,
            cpswap_program,
            cpswap_pool,
            cpswap_authority,
            cpswap_lp_mint,
            cpswap_base_vault,
            cpswap_quote_vault,
            cpswap_config,
            cpswap_create_pool_fee,
            cpswap_observation,
            lock_program,
            lock_authority,
            lock_lp_vault,
            authority,
            pool_state,
            global_config,
            base_vault,
            quote_vault,
            pool_lp_token,
            base_token_program,
            quote_token_program,
            associated_token_program,
            system_program,
            rent_program,
            metadata_program,
        })
    }
}
