use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6b123845e43837a4")]
pub struct SettleFundsExpired {}

pub struct SettleFundsExpiredInstructionAccounts {
    pub close_market_admin: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub penalty_payer: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub market_authority: solana_pubkey::Pubkey,
    pub market_base_vault: solana_pubkey::Pubkey,
    pub market_quote_vault: solana_pubkey::Pubkey,
    pub user_base_account: solana_pubkey::Pubkey,
    pub user_quote_account: solana_pubkey::Pubkey,
    pub referrer_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleFundsExpired {
    type ArrangedAccounts = SettleFundsExpiredInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [close_market_admin, owner, penalty_payer, open_orders_account, market, market_authority, market_base_vault, market_quote_vault, user_base_account, user_quote_account, referrer_account, token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
