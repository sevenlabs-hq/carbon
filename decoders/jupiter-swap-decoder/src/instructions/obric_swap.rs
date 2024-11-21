use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x415d60a9bed65f03")]
pub struct ObricSwap {}

pub struct ObricSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub trading_pair: solana_sdk::pubkey::Pubkey,
    pub mint_x: solana_sdk::pubkey::Pubkey,
    pub mint_y: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub user_token_account_x: solana_sdk::pubkey::Pubkey,
    pub user_token_account_y: solana_sdk::pubkey::Pubkey,
    pub protocol_fee: solana_sdk::pubkey::Pubkey,
    pub x_price_feed: solana_sdk::pubkey::Pubkey,
    pub y_price_feed: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ObricSwap {
    type ArrangedAccounts = ObricSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let trading_pair = accounts.get(1)?;
        let mint_x = accounts.get(2)?;
        let mint_y = accounts.get(3)?;
        let reserve_x = accounts.get(4)?;
        let reserve_y = accounts.get(5)?;
        let user_token_account_x = accounts.get(6)?;
        let user_token_account_y = accounts.get(7)?;
        let protocol_fee = accounts.get(8)?;
        let x_price_feed = accounts.get(9)?;
        let y_price_feed = accounts.get(10)?;
        let user = accounts.get(11)?;
        let token_program = accounts.get(12)?;

        Some(ObricSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            trading_pair: trading_pair.pubkey,
            mint_x: mint_x.pubkey,
            mint_y: mint_y.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            user_token_account_x: user_token_account_x.pubkey,
            user_token_account_y: user_token_account_y.pubkey,
            protocol_fee: protocol_fee.pubkey,
            x_price_feed: x_price_feed.pubkey,
            y_price_feed: y_price_feed.pubkey,
            user: user.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
