use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x38a681099dcd76d9")]
pub struct WhirlpoolSwapV2 {}

pub struct WhirlpoolSwapV2InstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub token_program_a: solana_sdk::pubkey::Pubkey,
    pub token_program_b: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub token_authority: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_mint_a: solana_sdk::pubkey::Pubkey,
    pub token_mint_b: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array0: solana_sdk::pubkey::Pubkey,
    pub tick_array1: solana_sdk::pubkey::Pubkey,
    pub tick_array2: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WhirlpoolSwapV2 {
    type ArrangedAccounts = WhirlpoolSwapV2InstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let token_program_a = accounts.get(1)?;
        let token_program_b = accounts.get(2)?;
        let memo_program = accounts.get(3)?;
        let token_authority = accounts.get(4)?;
        let whirlpool = accounts.get(5)?;
        let token_mint_a = accounts.get(6)?;
        let token_mint_b = accounts.get(7)?;
        let token_owner_account_a = accounts.get(8)?;
        let token_vault_a = accounts.get(9)?;
        let token_owner_account_b = accounts.get(10)?;
        let token_vault_b = accounts.get(11)?;
        let tick_array0 = accounts.get(12)?;
        let tick_array1 = accounts.get(13)?;
        let tick_array2 = accounts.get(14)?;
        let oracle = accounts.get(15)?;

        Some(WhirlpoolSwapV2InstructionAccounts {
            swap_program: swap_program.pubkey,
            token_program_a: token_program_a.pubkey,
            token_program_b: token_program_b.pubkey,
            memo_program: memo_program.pubkey,
            token_authority: token_authority.pubkey,
            whirlpool: whirlpool.pubkey,
            token_mint_a: token_mint_a.pubkey,
            token_mint_b: token_mint_b.pubkey,
            token_owner_account_a: token_owner_account_a.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_owner_account_b: token_owner_account_b.pubkey,
            token_vault_b: token_vault_b.pubkey,
            tick_array0: tick_array0.pubkey,
            tick_array1: tick_array1.pubkey,
            tick_array2: tick_array2.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
