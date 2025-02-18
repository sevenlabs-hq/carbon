use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xee40a3604bab1021")]
pub struct SettleFunds {}

pub struct SettleFundsInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub penalty_payer: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub market_authority: solana_sdk::pubkey::Pubkey,
    pub market_base_vault: solana_sdk::pubkey::Pubkey,
    pub market_quote_vault: solana_sdk::pubkey::Pubkey,
    pub user_base_account: solana_sdk::pubkey::Pubkey,
    pub user_quote_account: solana_sdk::pubkey::Pubkey,
    pub referrer_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleFunds {
    type ArrangedAccounts = SettleFundsInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let penalty_payer = accounts.get(1)?;
        let open_orders_account = accounts.get(2)?;
        let market = accounts.get(3)?;
        let market_authority = accounts.get(4)?;
        let market_base_vault = accounts.get(5)?;
        let market_quote_vault = accounts.get(6)?;
        let user_base_account = accounts.get(7)?;
        let user_quote_account = accounts.get(8)?;
        let referrer_account = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;

        Some(SettleFundsInstructionAccounts {
            owner: owner.pubkey,
            penalty_payer: penalty_payer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            market_base_vault: market_base_vault.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            user_base_account: user_base_account.pubkey,
            user_quote_account: user_quote_account.pubkey,
            referrer_account: referrer_account.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
