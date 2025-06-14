use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0bbcc1d68d5b95b8")]
pub struct InitializeTickArray {
    pub start_tick_index: i32,
}

#[derive(Debug, PartialEq)]
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
        let [whirlpool, funder, tick_array, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeTickArrayInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            funder: funder.pubkey,
            tick_array: tick_array.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
