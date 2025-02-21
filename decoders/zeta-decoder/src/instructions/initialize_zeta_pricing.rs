use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x23d1b41df5c77d10")]
pub struct InitializeZetaPricing {
    pub args: InitializeZetaPricingArgs,
}

pub struct InitializeZetaPricingInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeZetaPricing {
    type ArrangedAccounts = InitializeZetaPricingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, rent, system_program, token_program, admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeZetaPricingInstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            admin: admin.pubkey,
        })
    }
}
