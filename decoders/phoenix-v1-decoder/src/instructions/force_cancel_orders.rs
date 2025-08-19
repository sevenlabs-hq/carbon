use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6b")]
pub struct ForceCancelOrders {
    pub params: CancelUpToParams,
}

pub struct ForceCancelOrdersInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub market_authority: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub seat: solana_pubkey::Pubkey,
    pub base_account: solana_pubkey::Pubkey,
    pub quote_account: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForceCancelOrders {
    type ArrangedAccounts = ForceCancelOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, market_authority, trader, seat, base_account, quote_account, base_vault, quote_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ForceCancelOrdersInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            trader: trader.pubkey,
            seat: seat.pubkey,
            base_account: base_account.pubkey,
            quote_account: quote_account.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
