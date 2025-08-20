use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf52c091fecf91df")]
pub struct MigrateToAmm {
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub market_vault_signer_nonce: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateToAmmInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub openbook_program: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub request_queue: solana_pubkey::Pubkey,
    pub event_queue: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub market_vault_signer: solana_pubkey::Pubkey,
    pub market_base_vault: solana_pubkey::Pubkey,
    pub market_quote_vault: solana_pubkey::Pubkey,
    pub amm_program: solana_pubkey::Pubkey,
    pub amm_pool: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_lp_mint: solana_pubkey::Pubkey,
    pub amm_base_vault: solana_pubkey::Pubkey,
    pub amm_quote_vault: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub amm_create_fee_destination: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub pool_lp_token: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateToAmm {
    type ArrangedAccounts = MigrateToAmmInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let openbook_program = next_account(&mut iter)?;
        let market = next_account(&mut iter)?;
        let request_queue = next_account(&mut iter)?;
        let event_queue = next_account(&mut iter)?;
        let bids = next_account(&mut iter)?;
        let asks = next_account(&mut iter)?;
        let market_vault_signer = next_account(&mut iter)?;
        let market_base_vault = next_account(&mut iter)?;
        let market_quote_vault = next_account(&mut iter)?;
        let amm_program = next_account(&mut iter)?;
        let amm_pool = next_account(&mut iter)?;
        let amm_authority = next_account(&mut iter)?;
        let amm_open_orders = next_account(&mut iter)?;
        let amm_lp_mint = next_account(&mut iter)?;
        let amm_base_vault = next_account(&mut iter)?;
        let amm_quote_vault = next_account(&mut iter)?;
        let amm_target_orders = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let amm_create_fee_destination = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let pool_lp_token = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent_program = next_account(&mut iter)?;

        Some(MigrateToAmmInstructionAccounts {
            payer,
            base_mint,
            quote_mint,
            openbook_program,
            market,
            request_queue,
            event_queue,
            bids,
            asks,
            market_vault_signer,
            market_base_vault,
            market_quote_vault,
            amm_program,
            amm_pool,
            amm_authority,
            amm_open_orders,
            amm_lp_mint,
            amm_base_vault,
            amm_quote_vault,
            amm_target_orders,
            amm_config,
            amm_create_fee_destination,
            authority,
            pool_state,
            global_config,
            base_vault,
            quote_vault,
            pool_lp_token,
            spl_token_program,
            associated_token_program,
            system_program,
            rent_program,
        })
    }
}
