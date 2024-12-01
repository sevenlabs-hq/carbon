

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x04")]
pub struct UpdatePrimarySaleHappenedViaToken{
}

pub struct UpdatePrimarySaleHappenedViaTokenInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePrimarySaleHappenedViaToken {
    type ArrangedAccounts = UpdatePrimarySaleHappenedViaTokenInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let token = accounts.get(2)?;

        Some(UpdatePrimarySaleHappenedViaTokenInstructionAccounts {
            metadata: metadata.pubkey,
            owner: owner.pubkey,
            token: token.pubkey,
        })
    }
}