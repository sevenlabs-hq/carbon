use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7b45a8c3b7d5c7d6")]
pub struct EmergencyUnstake {
    pub stake_index: u32,
    pub validator_index: u32,
}

pub struct EmergencyUnstakeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub validator_manager_authority: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub stake_list: solana_pubkey::Pubkey,
    pub stake_account: solana_pubkey::Pubkey,
    pub stake_deposit_authority: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub stake_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EmergencyUnstake {
    type ArrangedAccounts = EmergencyUnstakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            validator_manager_authority,
            validator_list,
            stake_list,
            stake_account,
            stake_deposit_authority,
            clock,
            stake_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(EmergencyUnstakeInstructionAccounts {
            state: state.pubkey,
            validator_manager_authority: validator_manager_authority.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_account: stake_account.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
