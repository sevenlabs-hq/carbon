use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw {
    pub lp_token_amount: u64,
    pub minimum_token0_amount: u64,
    pub minimum_token1_amount: u64,
}

pub struct WithdrawInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub owner_lp_token: solana_sdk::pubkey::Pubkey,
    pub token0_account: solana_sdk::pubkey::Pubkey,
    pub token1_account: solana_sdk::pubkey::Pubkey,
    pub token0_vault: solana_sdk::pubkey::Pubkey,
    pub token1_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub vault0_mint: solana_sdk::pubkey::Pubkey,
    pub vault1_mint: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, authority, pool_state, owner_lp_token, token0_account, token1_account, token0_vault, token1_vault, token_program, token_program2022, vault0_mint, vault1_mint, lp_mint, memo_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            owner: owner.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            owner_lp_token: owner_lp_token.pubkey,
            token0_account: token0_account.pubkey,
            token1_account: token1_account.pubkey,
            token0_vault: token0_vault.pubkey,
            token1_vault: token1_vault.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            vault0_mint: vault0_mint.pubkey,
            vault1_mint: vault1_mint.pubkey,
            lp_mint: lp_mint.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
