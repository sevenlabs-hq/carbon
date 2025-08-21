use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0bbcc1d68d5b95b8")]
pub struct InitializeTickArray {
    pub start_tick_index: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeTickArrayInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub tick_array: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTickArray {
    type ArrangedAccounts = InitializeTickArrayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let tick_array = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeTickArrayInstructionAccounts {
            whirlpool,
            funder,
            tick_array,
            system_program,
        })
    }
}
