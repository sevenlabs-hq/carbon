
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x106762636a240569")]
pub struct PerpsV2RemoveLiquidity{
}

pub struct PerpsV2RemoveLiquidityInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub funding_or_receiving_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_token_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for PerpsV2RemoveLiquidity {
    type ArrangedAccounts = PerpsV2RemoveLiquidityInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let funding_or_receiving_account = accounts.get(2)?;
        let lp_token_account = accounts.get(3)?;
        let transfer_authority = accounts.get(4)?;
        let perpetuals = accounts.get(5)?;
        let pool = accounts.get(6)?;
        let custody = accounts.get(7)?;
        let custody_doves_price_account = accounts.get(8)?;
        let custody_pythnet_price_account = accounts.get(9)?;
        let custody_token_account = accounts.get(10)?;
        let lp_token_mint = accounts.get(11)?;
        let token_program = accounts.get(12)?;
        let event_authority = accounts.get(13)?;
        let program = accounts.get(14)?;

        Some(PerpsV2RemoveLiquidityInstructionAccounts {
            swap_program: *swap_program,
            owner: *owner,
            funding_or_receiving_account: *funding_or_receiving_account,
            lp_token_account: *lp_token_account,
            transfer_authority: *transfer_authority,
            perpetuals: *perpetuals,
            pool: *pool,
            custody: *custody,
            custody_doves_price_account: *custody_doves_price_account,
            custody_pythnet_price_account: *custody_pythnet_price_account,
            custody_token_account: *custody_token_account,
            lp_token_mint: *lp_token_mint,
            token_program: *token_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}