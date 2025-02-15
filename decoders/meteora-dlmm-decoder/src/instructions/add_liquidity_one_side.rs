use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5e9b6797465fdca5")]
pub struct AddLiquidityOneSide {
    pub liquidity_parameter: LiquidityOneSideParameter,
}

pub struct AddLiquidityOneSideInstructionAccounts {
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub user_token: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub bin_array_lower: solana_sdk::pubkey::Pubkey,
    pub bin_array_upper: solana_sdk::pubkey::Pubkey,
    pub sender: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidityOneSide {
    type ArrangedAccounts = AddLiquidityOneSideInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let position = accounts.get(0)?;
        let lb_pair = accounts.get(1)?;
        let bin_array_bitmap_extension = accounts.get(2)?;
        let user_token = accounts.get(3)?;
        let reserve = accounts.get(4)?;
        let token_mint = accounts.get(5)?;
        let bin_array_lower = accounts.get(6)?;
        let bin_array_upper = accounts.get(7)?;
        let sender = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

        Some(AddLiquidityOneSideInstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            user_token: user_token.pubkey,
            reserve: reserve.pubkey,
            token_mint: token_mint.pubkey,
            bin_array_lower: bin_array_lower.pubkey,
            bin_array_upper: bin_array_upper.pubkey,
            sender: sender.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
