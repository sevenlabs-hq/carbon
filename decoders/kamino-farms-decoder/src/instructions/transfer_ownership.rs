

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x41b1d749352d632f")]
pub struct TransferOwnership{
    pub new_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferOwnershipInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub user_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferOwnership {
    type ArrangedAccounts = TransferOwnershipInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            owner,
            user_state,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(TransferOwnershipInstructionAccounts {
            owner: owner.pubkey,
            user_state: user_state.pubkey,
        })
    }
}