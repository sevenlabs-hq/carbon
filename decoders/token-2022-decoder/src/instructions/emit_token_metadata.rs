use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfaa6b4fa0d0cb846")]
pub struct EmitTokenMetadata {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

pub struct EmitTokenMetadataInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EmitTokenMetadata {
    type ArrangedAccounts = EmitTokenMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EmitTokenMetadataInstructionAccounts {
            metadata: metadata.pubkey,
        })
    }
}
