use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e")]
pub struct PuffMetadata {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PuffMetadataInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PuffMetadata {
    type ArrangedAccounts = PuffMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;

        Some(PuffMetadataInstructionAccounts { metadata })
    }
}
