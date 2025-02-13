use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1a526698f04a691a")]
pub struct RemoveLiquidityByRange {
    pub from_bin_id: i32,
    pub to_bin_id: i32,
    pub bps_to_remove: u16,
}

pub struct RemoveLiquidityByRangeInstructionAccounts {
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub user_token_x: solana_sdk::pubkey::Pubkey,
    pub user_token_y: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub token_x_mint: solana_sdk::pubkey::Pubkey,
    pub token_y_mint: solana_sdk::pubkey::Pubkey,
    pub bin_array_lower: solana_sdk::pubkey::Pubkey,
    pub bin_array_upper: solana_sdk::pubkey::Pubkey,
    pub sender: solana_sdk::pubkey::Pubkey,
    pub token_x_program: solana_sdk::pubkey::Pubkey,
    pub token_y_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveLiquidityByRange {
    type ArrangedAccounts = RemoveLiquidityByRangeInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let position = accounts.get(0)?;
        let lb_pair = accounts.get(1)?;
        let bin_array_bitmap_extension = accounts.get(2)?;
        let user_token_x = accounts.get(3)?;
        let user_token_y = accounts.get(4)?;
        let reserve_x = accounts.get(5)?;
        let reserve_y = accounts.get(6)?;
        let token_x_mint = accounts.get(7)?;
        let token_y_mint = accounts.get(8)?;
        let bin_array_lower = accounts.get(9)?;
        let bin_array_upper = accounts.get(10)?;
        let sender = accounts.get(11)?;
        let token_x_program = accounts.get(12)?;
        let token_y_program = accounts.get(13)?;
        let event_authority = accounts.get(14)?;
        let program = accounts.get(15)?;

        Some(RemoveLiquidityByRangeInstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            user_token_x: user_token_x.pubkey,
            user_token_y: user_token_y.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            bin_array_lower: bin_array_lower.pubkey,
            bin_array_upper: bin_array_upper.pubkey,
            sender: sender.pubkey,
            token_x_program: token_x_program.pubkey,
            token_y_program: token_y_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
