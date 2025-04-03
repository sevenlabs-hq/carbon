use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x57d917b3cd197181")]
pub struct StakeReserve {
    pub validator_index: u32,
}

pub struct StakeReserveInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub stake_list: solana_pubkey::Pubkey,
    pub validator_vote: solana_pubkey::Pubkey,
    pub reserve_pda: solana_pubkey::Pubkey,
    pub stake_account: solana_pubkey::Pubkey,
    pub stake_deposit_authority: solana_pubkey::Pubkey,
    pub rent_payer: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub epoch_schedule: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub stake_history: solana_pubkey::Pubkey,
    pub stake_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub stake_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StakeReserve {
    type ArrangedAccounts = StakeReserveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, validator_list, stake_list, validator_vote, reserve_pda, stake_account, stake_deposit_authority, rent_payer, clock, epoch_schedule, rent, stake_history, stake_config, system_program, stake_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(StakeReserveInstructionAccounts {
            state: state.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            validator_vote: validator_vote.pubkey,
            reserve_pda: reserve_pda.pubkey,
            stake_account: stake_account.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            rent_payer: rent_payer.pubkey,
            clock: clock.pubkey,
            epoch_schedule: epoch_schedule.pubkey,
            rent: rent.pubkey,
            stake_history: stake_history.pubkey,
            stake_config: stake_config.pubkey,
            system_program: system_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
