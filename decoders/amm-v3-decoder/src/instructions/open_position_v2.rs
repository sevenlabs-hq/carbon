
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xda2da2af56115379")]
pub struct OpenPositionV2{
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
    pub with_matedata: bool,
    pub base_flag: Option<bool>,
}

pub struct OpenPositionV2InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub position_nft_owner: solana_sdk::pubkey::Pubkey,
    pub position_nft_mint: solana_sdk::pubkey::Pubkey,
    pub position_nft_account: solana_sdk::pubkey::Pubkey,
    pub metadata_account: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_position: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub token_account0: solana_sdk::pubkey::Pubkey,
    pub token_account1: solana_sdk::pubkey::Pubkey,
    pub token_vault0: solana_sdk::pubkey::Pubkey,
    pub token_vault1: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub vault0_mint: solana_sdk::pubkey::Pubkey,
    pub vault1_mint: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for OpenPositionV2 {
    type ArrangedAccounts = OpenPositionV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let position_nft_owner = accounts.get(1)?;
        let position_nft_mint = accounts.get(2)?;
        let position_nft_account = accounts.get(3)?;
        let metadata_account = accounts.get(4)?;
        let pool_state = accounts.get(5)?;
        let protocol_position = accounts.get(6)?;
        let tick_array_lower = accounts.get(7)?;
        let tick_array_upper = accounts.get(8)?;
        let personal_position = accounts.get(9)?;
        let token_account0 = accounts.get(10)?;
        let token_account1 = accounts.get(11)?;
        let token_vault0 = accounts.get(12)?;
        let token_vault1 = accounts.get(13)?;
        let rent = accounts.get(14)?;
        let system_program = accounts.get(15)?;
        let token_program = accounts.get(16)?;
        let associated_token_program = accounts.get(17)?;
        let metadata_program = accounts.get(18)?;
        let token_program2022 = accounts.get(19)?;
        let vault0_mint = accounts.get(20)?;
        let vault1_mint = accounts.get(21)?;

        Some(OpenPositionV2InstructionAccounts {
            payer: *payer,
            position_nft_owner: *position_nft_owner,
            position_nft_mint: *position_nft_mint,
            position_nft_account: *position_nft_account,
            metadata_account: *metadata_account,
            pool_state: *pool_state,
            protocol_position: *protocol_position,
            tick_array_lower: *tick_array_lower,
            tick_array_upper: *tick_array_upper,
            personal_position: *personal_position,
            token_account0: *token_account0,
            token_account1: *token_account1,
            token_vault0: *token_vault0,
            token_vault1: *token_vault1,
            rent: *rent,
            system_program: *system_program,
            token_program: *token_program,
            associated_token_program: *associated_token_program,
            metadata_program: *metadata_program,
            token_program2022: *token_program2022,
            vault0_mint: *vault0_mint,
            vault1_mint: *vault1_mint,
        })
    }
}