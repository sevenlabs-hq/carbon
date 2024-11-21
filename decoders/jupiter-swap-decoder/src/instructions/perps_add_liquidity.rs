use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaaeeded6f5ca6c9b")]
pub struct PerpsAddLiquidity {}

pub struct PerpsAddLiquidityInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub funding_or_receiving_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub custody_token_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PerpsAddLiquidity {
    type ArrangedAccounts = PerpsAddLiquidityInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let funding_or_receiving_account = accounts.get(2)?;
        let lp_token_account = accounts.get(3)?;
        let transfer_authority = accounts.get(4)?;
        let perpetuals = accounts.get(5)?;
        let pool = accounts.get(6)?;
        let custody = accounts.get(7)?;
        let custody_oracle_account = accounts.get(8)?;
        let custody_token_account = accounts.get(9)?;
        let lp_token_mint = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let event_authority = accounts.get(12)?;
        let program = accounts.get(13)?;

        Some(PerpsAddLiquidityInstructionAccounts {
            swap_program: swap_program.pubkey,
            owner: owner.pubkey,
            funding_or_receiving_account: funding_or_receiving_account.pubkey,
            lp_token_account: lp_token_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_oracle_account: custody_oracle_account.pubkey,
            custody_token_account: custody_token_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
