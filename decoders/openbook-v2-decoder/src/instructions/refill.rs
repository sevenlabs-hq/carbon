use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x80cf8e0b36e826c9")]
pub struct Refill {
    pub base_amount: u64,
    pub quote_amount: u64,
}

pub struct RefillInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub user_base_account: solana_pubkey::Pubkey,
    pub user_quote_account: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub market_base_vault: solana_pubkey::Pubkey,
    pub market_quote_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Refill {
    type ArrangedAccounts = RefillInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, user_base_account, user_quote_account, open_orders_account, market, market_base_vault, market_quote_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RefillInstructionAccounts {
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
