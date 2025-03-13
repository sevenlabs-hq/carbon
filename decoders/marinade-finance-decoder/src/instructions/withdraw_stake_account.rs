use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd355b841b7b1e9d9")]
pub struct WithdrawStakeAccount {
    pub stake_index: u32,
    pub validator_index: u32,
    pub msol_amount: u64,
    pub beneficiary: solana_sdk::pubkey::Pubkey,
}

pub struct WithdrawStakeAccountInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub burn_msol_from: solana_sdk::pubkey::Pubkey,
    pub burn_msol_authority: solana_sdk::pubkey::Pubkey,
    pub treasury_msol_account: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub stake_list: solana_sdk::pubkey::Pubkey,
    pub stake_withdraw_authority: solana_sdk::pubkey::Pubkey,
    pub stake_deposit_authority: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub split_stake_account: solana_sdk::pubkey::Pubkey,
    pub split_stake_rent_payer: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawStakeAccount {
    type ArrangedAccounts = WithdrawStakeAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, msol_mint, burn_msol_from, burn_msol_authority, treasury_msol_account, validator_list, stake_list, stake_withdraw_authority, stake_deposit_authority, stake_account, split_stake_account, split_stake_rent_payer, clock, system_program, token_program, stake_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawStakeAccountInstructionAccounts {
            state: state.pubkey,
            msol_mint: msol_mint.pubkey,
            burn_msol_from: burn_msol_from.pubkey,
            burn_msol_authority: burn_msol_authority.pubkey,
            treasury_msol_account: treasury_msol_account.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_withdraw_authority: stake_withdraw_authority.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            stake_account: stake_account.pubkey,
            split_stake_account: split_stake_account.pubkey,
            split_stake_rent_payer: split_stake_rent_payer.pubkey,
            clock: clock.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
