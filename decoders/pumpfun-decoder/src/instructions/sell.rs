use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x33e685a4017f83ad")]
pub struct Sell {
    pub amount: u64,
    pub min_sol_output: u64,
}

pub struct SellInstructionAccounts {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl ArrangeAccounts for Sell {
    type ArrangedAccounts = SellInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<Pubkey>) -> Option<Self::ArrangedAccounts> {
        let global = accounts.get(0)?;
        let fee_recipient = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let bonding_curve = accounts.get(3)?;
        let associated_bonding_curve = accounts.get(4)?;
        let associated_user = accounts.get(5)?;
        let user = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let associated_token_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let rent = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;
        Some(SellInstructionAccounts {
            global: *global,
            fee_recipient: *fee_recipient,
            mint: *mint,
            bonding_curve: *bonding_curve,
            associated_bonding_curve: *associated_bonding_curve,
            associated_user: *associated_user,
            user: *user,
            system_program: *system_program,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
            rent: *rent,
            event_authority: *event_authority,
            program: *program,
        })
    }
}
