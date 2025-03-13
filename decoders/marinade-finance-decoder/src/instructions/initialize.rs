use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub data: InitializeData,
}

pub struct InitializeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub reserve_pda: solana_sdk::pubkey::Pubkey,
    pub stake_list: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub operational_sol_account: solana_sdk::pubkey::Pubkey,
    pub liq_pool: solana_sdk::pubkey::Pubkey,
    pub treasury_msol_account: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, reserve_pda, stake_list, validator_list, msol_mint, operational_sol_account, liq_pool, treasury_msol_account, clock, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            state: state.pubkey,
            reserve_pda: reserve_pda.pubkey,
            stake_list: stake_list.pubkey,
            validator_list: validator_list.pubkey,
            msol_mint: msol_mint.pubkey,
            operational_sol_account: operational_sol_account.pubkey,
            liq_pool: liq_pool.pubkey,
            treasury_msol_account: treasury_msol_account.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
        })
    }
}
