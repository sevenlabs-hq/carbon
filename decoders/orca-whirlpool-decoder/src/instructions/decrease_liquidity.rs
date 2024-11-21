use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa026d06f685b2c01")]
pub struct DecreaseLiquidity {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

pub struct DecreaseLiquidityInstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecreaseLiquidity {
    type ArrangedAccounts = DecreaseLiquidityInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let token_program = accounts.get(1)?;
        let position_authority = accounts.get(2)?;
        let position = accounts.get(3)?;
        let position_token_account = accounts.get(4)?;
        let token_owner_account_a = accounts.get(5)?;
        let token_owner_account_b = accounts.get(6)?;
        let token_vault_a = accounts.get(7)?;
        let token_vault_b = accounts.get(8)?;
        let tick_array_lower = accounts.get(9)?;
        let tick_array_upper = accounts.get(10)?;

        Some(DecreaseLiquidityInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            token_program: token_program.pubkey,
            position_authority: position_authority.pubkey,
            position: position.pubkey,
            position_token_account: position_token_account.pubkey,
            token_owner_account_a: token_owner_account_a.pubkey,
            token_owner_account_b: token_owner_account_b.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_vault_b: token_vault_b.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
        })
    }
}
