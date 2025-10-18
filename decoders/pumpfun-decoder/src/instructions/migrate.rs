use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9beae792ec9ea21e")]
pub struct Migrate {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateInstructionAccounts {
    pub global: solana_pubkey::Pubkey,
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub associated_bonding_curve: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub pump_amm: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool_authority_mint_account: solana_pubkey::Pubkey,
    pub pool_authority_wsol_account: solana_pubkey::Pubkey,
    pub amm_global_config: solana_pubkey::Pubkey,
    pub wsol_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub user_pool_token_account: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub pump_amm_event_authority: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Migrate {
    type ArrangedAccounts = MigrateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let global = next_account(&mut iter)?;
        let withdraw_authority = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let associated_bonding_curve = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let pump_amm = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool_authority_mint_account = next_account(&mut iter)?;
        let pool_authority_wsol_account = next_account(&mut iter)?;
        let amm_global_config = next_account(&mut iter)?;
        let wsol_mint = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let user_pool_token_account = next_account(&mut iter)?;
        let pool_base_token_account = next_account(&mut iter)?;
        let pool_quote_token_account = next_account(&mut iter)?;
        let token_2022_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let pump_amm_event_authority = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(MigrateInstructionAccounts {
            global,
            withdraw_authority,
            mint,
            bonding_curve,
            associated_bonding_curve,
            user,
            system_program,
            token_program,
            pump_amm,
            pool,
            pool_authority,
            pool_authority_mint_account,
            pool_authority_wsol_account,
            amm_global_config,
            wsol_mint,
            lp_mint,
            user_pool_token_account,
            pool_base_token_account,
            pool_quote_token_account,
            token_2022_program,
            associated_token_program,
            pump_amm_event_authority,
            event_authority,
            program,
        })
    }
}
