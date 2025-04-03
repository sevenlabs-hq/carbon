use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa59ee561a8dcbbe1")]
pub struct DeactivateStake {
    pub stake_index: u32,
    pub validator_index: u32,
}

pub struct DeactivateStakeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub reserve_pda: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub stake_list: solana_pubkey::Pubkey,
    pub stake_account: solana_pubkey::Pubkey,
    pub stake_deposit_authority: solana_pubkey::Pubkey,
    pub split_stake_account: solana_pubkey::Pubkey,
    pub split_stake_rent_payer: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub epoch_schedule: solana_pubkey::Pubkey,
    pub stake_history: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub stake_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeactivateStake {
    type ArrangedAccounts = DeactivateStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, reserve_pda, validator_list, stake_list, stake_account, stake_deposit_authority, split_stake_account, split_stake_rent_payer, clock, rent, epoch_schedule, stake_history, system_program, stake_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DeactivateStakeInstructionAccounts {
            state: state.pubkey,
            reserve_pda: reserve_pda.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_account: stake_account.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            split_stake_account: split_stake_account.pubkey,
            split_stake_rent_payer: split_stake_rent_payer.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            epoch_schedule: epoch_schedule.pubkey,
            stake_history: stake_history.pubkey,
            system_program: system_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
