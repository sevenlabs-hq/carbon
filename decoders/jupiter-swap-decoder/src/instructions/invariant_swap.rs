use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbbc128792f4990b1")]
pub struct InvariantSwap {}

pub struct InvariantSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub tickmap: solana_sdk::pubkey::Pubkey,
    pub account_x: solana_sdk::pubkey::Pubkey,
    pub account_y: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InvariantSwap {
    type ArrangedAccounts = InvariantSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let state = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let tickmap = accounts.get(3)?;
        let account_x = accounts.get(4)?;
        let account_y = accounts.get(5)?;
        let reserve_x = accounts.get(6)?;
        let reserve_y = accounts.get(7)?;
        let owner = accounts.get(8)?;
        let program_authority = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(InvariantSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            state: state.pubkey,
            pool: pool.pubkey,
            tickmap: tickmap.pubkey,
            account_x: account_x.pubkey,
            account_y: account_y.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            owner: owner.pubkey,
            program_authority: program_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
