use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x333091737b5f478a")]
pub struct TokenSwapV2 {}

pub struct TokenSwapV2InstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub swap: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source: solana_sdk::pubkey::Pubkey,
    pub swap_source: solana_sdk::pubkey::Pubkey,
    pub swap_destination: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub pool_fee: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub source_token_program: solana_sdk::pubkey::Pubkey,
    pub destination_token_program: solana_sdk::pubkey::Pubkey,
    pub pool_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TokenSwapV2 {
    type ArrangedAccounts = TokenSwapV2InstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let swap = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let user_transfer_authority = accounts.get(3)?;
        let source = accounts.get(4)?;
        let swap_source = accounts.get(5)?;
        let swap_destination = accounts.get(6)?;
        let destination = accounts.get(7)?;
        let pool_mint = accounts.get(8)?;
        let pool_fee = accounts.get(9)?;
        let source_mint = accounts.get(10)?;
        let destination_mint = accounts.get(11)?;
        let source_token_program = accounts.get(12)?;
        let destination_token_program = accounts.get(13)?;
        let pool_token_program = accounts.get(14)?;

        Some(TokenSwapV2InstructionAccounts {
            swap_program: swap_program.pubkey,
            swap: swap.pubkey,
            authority: authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            source: source.pubkey,
            swap_source: swap_source.pubkey,
            swap_destination: swap_destination.pubkey,
            destination: destination.pubkey,
            pool_mint: pool_mint.pubkey,
            pool_fee: pool_fee.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            source_token_program: source_token_program.pubkey,
            destination_token_program: destination_token_program.pubkey,
            pool_token_program: pool_token_program.pubkey,
        })
    }
}
