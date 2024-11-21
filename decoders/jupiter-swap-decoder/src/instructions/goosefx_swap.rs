use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xde882e7bbd7d7c7a")]
pub struct GoosefxSwap {}

pub struct GoosefxSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub controller: solana_sdk::pubkey::Pubkey,
    pub pair: solana_sdk::pubkey::Pubkey,
    pub ssl_in: solana_sdk::pubkey::Pubkey,
    pub ssl_out: solana_sdk::pubkey::Pubkey,
    pub liability_vault_in: solana_sdk::pubkey::Pubkey,
    pub swapped_liability_vault_in: solana_sdk::pubkey::Pubkey,
    pub liability_vault_out: solana_sdk::pubkey::Pubkey,
    pub swapped_liability_vault_out: solana_sdk::pubkey::Pubkey,
    pub user_in_ata: solana_sdk::pubkey::Pubkey,
    pub user_out_ata: solana_sdk::pubkey::Pubkey,
    pub fee_collector_ata: solana_sdk::pubkey::Pubkey,
    pub user_wallet: solana_sdk::pubkey::Pubkey,
    pub fee_collector: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GoosefxSwap {
    type ArrangedAccounts = GoosefxSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let controller = accounts.get(1)?;
        let pair = accounts.get(2)?;
        let ssl_in = accounts.get(3)?;
        let ssl_out = accounts.get(4)?;
        let liability_vault_in = accounts.get(5)?;
        let swapped_liability_vault_in = accounts.get(6)?;
        let liability_vault_out = accounts.get(7)?;
        let swapped_liability_vault_out = accounts.get(8)?;
        let user_in_ata = accounts.get(9)?;
        let user_out_ata = accounts.get(10)?;
        let fee_collector_ata = accounts.get(11)?;
        let user_wallet = accounts.get(12)?;
        let fee_collector = accounts.get(13)?;
        let token_program = accounts.get(14)?;

        Some(GoosefxSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            controller: controller.pubkey,
            pair: pair.pubkey,
            ssl_in: ssl_in.pubkey,
            ssl_out: ssl_out.pubkey,
            liability_vault_in: liability_vault_in.pubkey,
            swapped_liability_vault_in: swapped_liability_vault_in.pubkey,
            liability_vault_out: liability_vault_out.pubkey,
            swapped_liability_vault_out: swapped_liability_vault_out.pubkey,
            user_in_ata: user_in_ata.pubkey,
            user_out_ata: user_out_ata.pubkey,
            fee_collector_ata: fee_collector_ata.pubkey,
            user_wallet: user_wallet.pubkey,
            fee_collector: fee_collector.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
