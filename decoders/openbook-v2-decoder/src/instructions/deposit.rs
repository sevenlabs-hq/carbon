use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub base_amount: u64,
    pub quote_amount: u64,
}

pub struct DepositInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub user_base_account: solana_sdk::pubkey::Pubkey,
    pub user_quote_account: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub market_base_vault: solana_sdk::pubkey::Pubkey,
    pub market_quote_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let user_base_account = accounts.get(1)?;
        let user_quote_account = accounts.get(2)?;
        let open_orders_account = accounts.get(3)?;
        let market = accounts.get(4)?;
        let market_base_vault = accounts.get(5)?;
        let market_quote_vault = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(DepositInstructionAccounts {
            owner: owner.pubkey,
            user_base_account: user_base_account.pubkey,
            user_quote_account: user_quote_account.pubkey,
            open_orders_account: open_orders_account.pubkey,
            market: market.pubkey,
            market_base_vault: market_base_vault.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
