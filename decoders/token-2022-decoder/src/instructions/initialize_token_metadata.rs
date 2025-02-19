use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd2e11ea258b84d8d")]
pub struct InitializeTokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct InitializeTokenMetadataInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTokenMetadata {
    type ArrangedAccounts = InitializeTokenMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, update_authority, mint, mint_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeTokenMetadataInstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
        })
    }
}
