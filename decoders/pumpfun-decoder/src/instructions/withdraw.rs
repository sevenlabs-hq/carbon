use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw;

pub struct WithdrawInstructionAccounts {
    pub global: Pubkey,
    pub last_withdraw: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<Pubkey>) -> Option<Self::ArrangedAccounts> {
        let global = accounts.get(0)?;
        let last_withdraw = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let bonding_curve = accounts.get(3)?;
        let associated_bonding_curve = accounts.get(4)?;
        let associated_user = accounts.get(5)?;
        let user = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let rent = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;
        Some(WithdrawInstructionAccounts {
            global: *global,
            last_withdraw: *last_withdraw,
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
        })
    }
}
