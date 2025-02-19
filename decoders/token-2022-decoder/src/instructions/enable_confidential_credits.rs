use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct EnableConfidentialCredits {
    pub confidential_transfer_discriminator: u8,
}

pub struct EnableConfidentialCreditsInstructionAccounts {
    pub token: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EnableConfidentialCredits {
    type ArrangedAccounts = EnableConfidentialCreditsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EnableConfidentialCreditsInstructionAccounts {
            token: token.pubkey,
            authority: authority.pubkey,
        })
    }
}
