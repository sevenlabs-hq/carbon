use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x37f1cddd2d72cda3")]
pub struct PartialUnstake {
    pub stake_index: u32,
    pub validator_index: u32,
    pub desired_unstake_amount: u64,
}

pub struct PartialUnstakeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub validator_manager_authority: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub stake_list: solana_pubkey::Pubkey,
    pub stake_account: solana_pubkey::Pubkey,
    pub stake_deposit_authority: solana_pubkey::Pubkey,
    pub reserve_pda: solana_pubkey::Pubkey,
    pub split_stake_account: solana_pubkey::Pubkey,
    pub split_stake_rent_payer: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub stake_history: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub stake_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PartialUnstake {
    type ArrangedAccounts = PartialUnstakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, validator_manager_authority, validator_list, stake_list, stake_account, stake_deposit_authority, reserve_pda, split_stake_account, split_stake_rent_payer, clock, rent, stake_history, system_program, stake_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PartialUnstakeInstructionAccounts {
            state: state.pubkey,
            validator_manager_authority: validator_manager_authority.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_account: stake_account.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            reserve_pda: reserve_pda.pubkey,
            split_stake_account: split_stake_account.pubkey,
            split_stake_rent_payer: split_stake_rent_payer.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            stake_history: stake_history.pubkey,
            system_program: system_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
