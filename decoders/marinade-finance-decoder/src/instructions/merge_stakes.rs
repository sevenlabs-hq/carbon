use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd8248de1f34e7ded")]
pub struct MergeStakes {
    pub destination_stake_index: u32,
    pub source_stake_index: u32,
    pub validator_index: u32,
}

pub struct MergeStakesInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub stake_list: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub destination_stake: solana_sdk::pubkey::Pubkey,
    pub source_stake: solana_sdk::pubkey::Pubkey,
    pub stake_deposit_authority: solana_sdk::pubkey::Pubkey,
    pub stake_withdraw_authority: solana_sdk::pubkey::Pubkey,
    pub operational_sol_account: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MergeStakes {
    type ArrangedAccounts = MergeStakesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, stake_list, validator_list, destination_stake, source_stake, stake_deposit_authority, stake_withdraw_authority, operational_sol_account, clock, stake_history, stake_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MergeStakesInstructionAccounts {
            state: state.pubkey,
            stake_list: stake_list.pubkey,
            validator_list: validator_list.pubkey,
            destination_stake: destination_stake.pubkey,
            source_stake: source_stake.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            stake_withdraw_authority: stake_withdraw_authority.pubkey,
            operational_sol_account: operational_sol_account.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
