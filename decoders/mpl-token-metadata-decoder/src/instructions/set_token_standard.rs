use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x23")]
pub struct SetTokenStandard {}

pub struct SetTokenStandardInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenStandard {
    type ArrangedAccounts = SetTokenStandardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, update_authority, mint, edition] = accounts else {
            return None;
        };

        Some(SetTokenStandardInstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
            mint: mint.pubkey,
            edition: edition.pubkey,
        })
    }
}
