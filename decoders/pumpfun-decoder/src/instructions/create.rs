use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x181ec828051c0777")]
pub struct Create {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct CreateInstructionAccounts {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub mpl_token_metadata: Pubkey,
    pub metadata: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<Pubkey>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let mint_authority = accounts.get(1)?;
        let bonding_curve = accounts.get(2)?;
        let associated_bonding_curve = accounts.get(3)?;
        let mpl_token_metadata = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let user = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let rent = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;
        Some(CreateInstructionAccounts {
            mint: *mint,
            mint_authority: *mint_authority,
            bonding_curve: *bonding_curve,
            associated_bonding_curve: *associated_bonding_curve,
            mpl_token_metadata: *mpl_token_metadata,
            metadata: *metadata,
            user: *user,
            system_program: *system_program,
            token_program: *token_program,
            associated_token_program: *associated_token_program,
            rent: *rent,
            event_authority: *event_authority,
            program: *program,
        })
    }
}
