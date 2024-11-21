use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7f40258aadf3cf54")]
pub struct MeteoraDlmmSwap {}

pub struct MeteoraDlmmSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub user_token_in: solana_sdk::pubkey::Pubkey,
    pub user_token_out: solana_sdk::pubkey::Pubkey,
    pub token_x_mint: solana_sdk::pubkey::Pubkey,
    pub token_y_mint: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub host_fee_in: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub token_x_program: solana_sdk::pubkey::Pubkey,
    pub token_y_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MeteoraDlmmSwap {
    type ArrangedAccounts = MeteoraDlmmSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let lb_pair = accounts.get(1)?;
        let bin_array_bitmap_extension = accounts.get(2)?;
        let reserve_x = accounts.get(3)?;
        let reserve_y = accounts.get(4)?;
        let user_token_in = accounts.get(5)?;
        let user_token_out = accounts.get(6)?;
        let token_x_mint = accounts.get(7)?;
        let token_y_mint = accounts.get(8)?;
        let oracle = accounts.get(9)?;
        let host_fee_in = accounts.get(10)?;
        let user = accounts.get(11)?;
        let token_x_program = accounts.get(12)?;
        let token_y_program = accounts.get(13)?;
        let event_authority = accounts.get(14)?;
        let program = accounts.get(15)?;

        Some(MeteoraDlmmSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            user_token_in: user_token_in.pubkey,
            user_token_out: user_token_out.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            oracle: oracle.pubkey,
            host_fee_in: host_fee_in.pubkey,
            user: user.pubkey,
            token_x_program: token_x_program.pubkey,
            token_y_program: token_y_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
