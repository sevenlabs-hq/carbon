

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xcad9434aac168cd8")]
pub struct WithdrawSlashedAmount{
}

pub struct WithdrawSlashedAmountInstructionAccounts {
    pub crank: solana_sdk::pubkey::Pubkey,
    pub farm_state: solana_sdk::pubkey::Pubkey,
    pub slashed_amount_spill_address: solana_sdk::pubkey::Pubkey,
    pub farm_vault: solana_sdk::pubkey::Pubkey,
    pub farm_vaults_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawSlashedAmount {
    type ArrangedAccounts = WithdrawSlashedAmountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            crank,
            farm_state,
            slashed_amount_spill_address,
            farm_vault,
            farm_vaults_authority,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(WithdrawSlashedAmountInstructionAccounts {
            crank: crank.pubkey,
            farm_state: farm_state.pubkey,
            slashed_amount_spill_address: slashed_amount_spill_address.pubkey,
            farm_vault: farm_vault.pubkey,
            farm_vaults_authority: farm_vaults_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}