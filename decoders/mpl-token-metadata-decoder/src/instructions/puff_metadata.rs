use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e")]
pub struct PuffMetadata {}

pub struct PuffMetadataInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PuffMetadata {
    type ArrangedAccounts = PuffMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PuffMetadataInstructionAccounts {
            metadata: metadata.pubkey,
        })
    }
}
