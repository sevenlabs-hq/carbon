use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x84e66678cd09edbe")]
pub struct DeltafiSwap {}

pub struct DeltafiSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub market_config: solana_sdk::pubkey::Pubkey,
    pub swap_info: solana_sdk::pubkey::Pubkey,
    pub user_source_token: solana_sdk::pubkey::Pubkey,
    pub user_destination_token: solana_sdk::pubkey::Pubkey,
    pub swap_source_token: solana_sdk::pubkey::Pubkey,
    pub swap_destination_token: solana_sdk::pubkey::Pubkey,
    pub deltafi_user: solana_sdk::pubkey::Pubkey,
    pub admin_destination_token: solana_sdk::pubkey::Pubkey,
    pub pyth_price_base: solana_sdk::pubkey::Pubkey,
    pub pyth_price_quote: solana_sdk::pubkey::Pubkey,
    pub user_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeltafiSwap {
    type ArrangedAccounts = DeltafiSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let market_config = accounts.get(1)?;
        let swap_info = accounts.get(2)?;
        let user_source_token = accounts.get(3)?;
        let user_destination_token = accounts.get(4)?;
        let swap_source_token = accounts.get(5)?;
        let swap_destination_token = accounts.get(6)?;
        let deltafi_user = accounts.get(7)?;
        let admin_destination_token = accounts.get(8)?;
        let pyth_price_base = accounts.get(9)?;
        let pyth_price_quote = accounts.get(10)?;
        let user_authority = accounts.get(11)?;
        let token_program = accounts.get(12)?;

        Some(DeltafiSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            market_config: market_config.pubkey,
            swap_info: swap_info.pubkey,
            user_source_token: user_source_token.pubkey,
            user_destination_token: user_destination_token.pubkey,
            swap_source_token: swap_source_token.pubkey,
            swap_destination_token: swap_destination_token.pubkey,
            deltafi_user: deltafi_user.pubkey,
            admin_destination_token: admin_destination_token.pubkey,
            pyth_price_base: pyth_price_base.pubkey,
            pyth_price_quote: pyth_price_quote.pubkey,
            user_authority: user_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
