use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6f11b9fa3c7a26fe")]
pub struct InitializeUser {}

pub struct InitializeUserInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub delegatee: solana_sdk::pubkey::Pubkey,
    pub user_state: solana_sdk::pubkey::Pubkey,
    pub farm_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeUser {
    type ArrangedAccounts = InitializeUserInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, payer, owner, delegatee, user_state, farm_state, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeUserInstructionAccounts {
            authority: authority.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            delegatee: delegatee.pubkey,
            user_state: user_state.pubkey,
            farm_state: farm_state.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
