use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0bbcc1d68d5b95b8")]
pub struct InitializeTickArray {
    pub start_tick_index: i32,
}

pub struct InitializeTickArrayInstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub tick_array: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTickArray {
    type ArrangedAccounts = InitializeTickArrayInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let funder = accounts.get(1)?;
        let tick_array = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(InitializeTickArrayInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            funder: funder.pubkey,
            tick_array: tick_array.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
