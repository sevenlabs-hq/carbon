use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x27")]
pub struct InitializeMetadataPointer {
    pub metadata_pointer_discriminator: u8,
    pub authority: Option<solana_sdk::pubkey::Pubkey>,
    pub metadata_address: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeMetadataPointerInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMetadataPointer {
    type ArrangedAccounts = InitializeMetadataPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMetadataPointerInstructionAccounts { mint: mint.pubkey })
    }
}
