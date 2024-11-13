use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x29c28cd95aa08b06")]
pub struct RemoveCreatorVerification {}

pub struct RemoveCreatorVerificationInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub creator: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCreatorVerification {
    type ArrangedAccounts = RemoveCreatorVerificationInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let creator = accounts.get(1)?;

        Some(RemoveCreatorVerificationInstructionAccounts {
            metadata: metadata.pubkey,
            creator: creator.pubkey,
        })
    }
}
