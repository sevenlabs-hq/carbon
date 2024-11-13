use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x57d9158469ee4772")]
pub struct PuffMetadata {}

pub struct PuffMetadataInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PuffMetadata {
    type ArrangedAccounts = PuffMetadataInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;

        Some(PuffMetadataInstructionAccounts {
            metadata: metadata.pubkey,
        })
    }
}
