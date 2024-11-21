
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfb0ae74c1b0b9f60")]
pub struct InitObligation{
    pub args: InitObligationArgs,
}

pub struct InitObligationInstructionAccounts {
    pub obligation_owner: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub seed1_account: solana_sdk::pubkey::Pubkey,
    pub seed2_account: solana_sdk::pubkey::Pubkey,
    pub owner_user_metadata: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitObligation {
    type ArrangedAccounts = InitObligationInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let obligation_owner = accounts.get(0)?;
        let fee_payer = accounts.get(1)?;
        let obligation = accounts.get(2)?;
        let lending_market = accounts.get(3)?;
        let seed1_account = accounts.get(4)?;
        let seed2_account = accounts.get(5)?;
        let owner_user_metadata = accounts.get(6)?;
        let rent = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(InitObligationInstructionAccounts {
            obligation_owner: obligation_owner.pubkey,
            fee_payer: fee_payer.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            seed1_account: seed1_account.pubkey,
            seed2_account: seed2_account.pubkey,
            owner_user_metadata: owner_user_metadata.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}