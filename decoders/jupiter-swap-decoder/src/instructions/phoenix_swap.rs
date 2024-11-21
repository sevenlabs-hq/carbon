use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6342df5fec831a8c")]
pub struct PhoenixSwap {}

pub struct PhoenixSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub log_authority: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub trader: solana_sdk::pubkey::Pubkey,
    pub base_account: solana_sdk::pubkey::Pubkey,
    pub quote_account: solana_sdk::pubkey::Pubkey,
    pub base_vault: solana_sdk::pubkey::Pubkey,
    pub quote_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PhoenixSwap {
    type ArrangedAccounts = PhoenixSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let log_authority = accounts.get(1)?;
        let market = accounts.get(2)?;
        let trader = accounts.get(3)?;
        let base_account = accounts.get(4)?;
        let quote_account = accounts.get(5)?;
        let base_vault = accounts.get(6)?;
        let quote_vault = accounts.get(7)?;
        let token_program = accounts.get(8)?;

        Some(PhoenixSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            trader: trader.pubkey,
            base_account: base_account.pubkey,
            quote_account: quote_account.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
