use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8a8ba786d05b8a9e")]
pub struct PumpdotfunWrappedBuy {}

pub struct PumpdotfunWrappedBuyInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub global: solana_sdk::pubkey::Pubkey,
    pub fee_recipient: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_user: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub user_wsol_token_account: solana_sdk::pubkey::Pubkey,
    pub temp_wsol_token_account: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for PumpdotfunWrappedBuy {
    type ArrangedAccounts = PumpdotfunWrappedBuyInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let global = accounts.get(1)?;
        let fee_recipient = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let bonding_curve = accounts.get(4)?;
        let associated_bonding_curve = accounts.get(5)?;
        let associated_user = accounts.get(6)?;
        let user = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let rent = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;
        let user_wsol_token_account = accounts.get(13)?;
        let temp_wsol_token_account = accounts.get(14)?;
        let wsol_mint = accounts.get(15)?;

        Some(PumpdotfunWrappedBuyInstructionAccounts {
            swap_program: *swap_program,
            global: *global,
            fee_recipient: *fee_recipient,
            mint: *mint,
            bonding_curve: *bonding_curve,
            associated_bonding_curve: *associated_bonding_curve,
            associated_user: *associated_user,
            user: *user,
            system_program: *system_program,
            token_program: *token_program,
            rent: *rent,
            event_authority: *event_authority,
            program: *program,
            user_wsol_token_account: *user_wsol_token_account,
            temp_wsol_token_account: *temp_wsol_token_account,
            wsol_mint: *wsol_mint,
        })
    }
}
