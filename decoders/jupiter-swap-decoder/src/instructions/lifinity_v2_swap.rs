use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1398c3f5bb904ae3")]
pub struct LifinityV2Swap {}

pub struct LifinityV2SwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source_info: solana_sdk::pubkey::Pubkey,
    pub destination_info: solana_sdk::pubkey::Pubkey,
    pub swap_source: solana_sdk::pubkey::Pubkey,
    pub swap_destination: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub fee_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub oracle_main_account: solana_sdk::pubkey::Pubkey,
    pub oracle_sub_account: solana_sdk::pubkey::Pubkey,
    pub oracle_pc_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LifinityV2Swap {
    type ArrangedAccounts = LifinityV2SwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let amm = accounts.get(2)?;
        let user_transfer_authority = accounts.get(3)?;
        let source_info = accounts.get(4)?;
        let destination_info = accounts.get(5)?;
        let swap_source = accounts.get(6)?;
        let swap_destination = accounts.get(7)?;
        let pool_mint = accounts.get(8)?;
        let fee_account = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let oracle_main_account = accounts.get(11)?;
        let oracle_sub_account = accounts.get(12)?;
        let oracle_pc_account = accounts.get(13)?;

        Some(LifinityV2SwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            authority: authority.pubkey,
            amm: amm.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            source_info: source_info.pubkey,
            destination_info: destination_info.pubkey,
            swap_source: swap_source.pubkey,
            swap_destination: swap_destination.pubkey,
            pool_mint: pool_mint.pubkey,
            fee_account: fee_account.pubkey,
            token_program: token_program.pubkey,
            oracle_main_account: oracle_main_account.pubkey,
            oracle_sub_account: oracle_sub_account.pubkey,
            oracle_pc_account: oracle_pc_account.pubkey,
        })
    }
}
