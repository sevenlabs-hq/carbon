use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafe1624776422294")]
pub struct SweepFees {}

pub struct SweepFeesInstructionAccounts {
    pub collect_fee_admin: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub market_authority: solana_sdk::pubkey::Pubkey,
    pub market_quote_vault: solana_sdk::pubkey::Pubkey,
    pub token_receiver_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SweepFees {
    type ArrangedAccounts = SweepFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collect_fee_admin, market, market_authority, market_quote_vault, token_receiver_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SweepFeesInstructionAccounts {
            collect_fee_admin: collect_fee_admin.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            token_receiver_account: token_receiver_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
