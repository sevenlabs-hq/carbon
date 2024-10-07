use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x181ec828051c0777")]
pub struct Create {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct CreateInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_bonding_curve: solana_sdk::pubkey::Pubkey,
    pub global: solana_sdk::pubkey::Pubkey,
    pub mpl_token_metadata: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let mint_authority = accounts.get(1)?;
        let bonding_curve = accounts.get(2)?;
        let associated_bonding_curve = accounts.get(3)?;
        let global = accounts.get(4)?;
        let mpl_token_metadata = accounts.get(5)?;
        let metadata = accounts.get(6)?;
        let user = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let rent = accounts.get(11)?;
        let event_authority = accounts.get(12)?;
        let program = accounts.get(13)?;

        Some(CreateInstructionAccounts {
            mint: *mint,
            mint_authority: *mint_authority,
            bonding_curve: *bonding_curve,
            associated_bonding_curve: *associated_bonding_curve,
            global: *global,
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
