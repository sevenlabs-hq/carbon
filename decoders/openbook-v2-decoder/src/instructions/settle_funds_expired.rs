use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6b123845e43837a4")]
pub struct SettleFundsExpired {}

pub struct SettleFundsExpiredInstructionAccounts {
    pub close_market_admin: solana_sdk::pubkey::Pubkey,
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

impl carbon_core::deserialize::ArrangeAccounts for SettleFundsExpired {
    type ArrangedAccounts = SettleFundsExpiredInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let close_market_admin = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let penalty_payer = accounts.get(2)?;
        let open_orders_account = accounts.get(3)?;
        let market = accounts.get(4)?;
        let market_authority = accounts.get(5)?;
        let market_base_vault = accounts.get(6)?;
        let market_quote_vault = accounts.get(7)?;
        let user_base_account = accounts.get(8)?;
        let user_quote_account = accounts.get(9)?;
        let referrer_account = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let system_program = accounts.get(12)?;

        Some(SettleFundsExpiredInstructionAccounts {
            close_market_admin: close_market_admin.pubkey,
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
