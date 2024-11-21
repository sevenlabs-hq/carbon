use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x55c99a5c851f8e55")]
pub struct CloneSwap {}

pub struct CloneSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub clone: solana_sdk::pubkey::Pubkey,
    pub pools: solana_sdk::pubkey::Pubkey,
    pub oracles: solana_sdk::pubkey::Pubkey,
    pub user_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub user_onasset_token_account: solana_sdk::pubkey::Pubkey,
    pub onasset_mint: solana_sdk::pubkey::Pubkey,
    pub collateral_mint: solana_sdk::pubkey::Pubkey,
    pub collateral_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_onasset_token_account: solana_sdk::pubkey::Pubkey,
    pub treasury_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub clone_staking: solana_sdk::pubkey::Pubkey,
    pub user_staking_account: solana_sdk::pubkey::Pubkey,
    pub clone_staking_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloneSwap {
    type ArrangedAccounts = CloneSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let user = accounts.get(1)?;
        let clone = accounts.get(2)?;
        let pools = accounts.get(3)?;
        let oracles = accounts.get(4)?;
        let user_collateral_token_account = accounts.get(5)?;
        let user_onasset_token_account = accounts.get(6)?;
        let onasset_mint = accounts.get(7)?;
        let collateral_mint = accounts.get(8)?;
        let collateral_vault = accounts.get(9)?;
        let treasury_onasset_token_account = accounts.get(10)?;
        let treasury_collateral_token_account = accounts.get(11)?;
        let token_program = accounts.get(12)?;
        let clone_staking = accounts.get(13)?;
        let user_staking_account = accounts.get(14)?;
        let clone_staking_program = accounts.get(15)?;

        Some(CloneSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            user: user.pubkey,
            clone: clone.pubkey,
            pools: pools.pubkey,
            oracles: oracles.pubkey,
            user_collateral_token_account: user_collateral_token_account.pubkey,
            user_onasset_token_account: user_onasset_token_account.pubkey,
            onasset_mint: onasset_mint.pubkey,
            collateral_mint: collateral_mint.pubkey,
            collateral_vault: collateral_vault.pubkey,
            treasury_onasset_token_account: treasury_onasset_token_account.pubkey,
            treasury_collateral_token_account: treasury_collateral_token_account.pubkey,
            token_program: token_program.pubkey,
            clone_staking: clone_staking.pubkey,
            user_staking_account: user_staking_account.pubkey,
            clone_staking_program: clone_staking_program.pubkey,
        })
    }
}
