use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfa7135368d75d7b9")]
pub struct AddValidator {
    pub score: u32,
}

pub struct AddValidatorInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub manager_authority: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub validator_vote: solana_sdk::pubkey::Pubkey,
    pub duplication_flag: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddValidator {
    type ArrangedAccounts = AddValidatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, manager_authority, validator_list, validator_vote, duplication_flag, rent_payer, clock, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddValidatorInstructionAccounts {
            state: state.pubkey,
            manager_authority: manager_authority.pubkey,
            validator_list: validator_list.pubkey,
            validator_vote: validator_vote.pubkey,
            duplication_flag: duplication_flag.pubkey,
            rent_payer: rent_payer.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
