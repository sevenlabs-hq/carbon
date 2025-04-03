use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct DisableNonConfidentialCredits {
    pub confidential_transfer_discriminator: u8,
}

pub struct DisableNonConfidentialCreditsInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DisableNonConfidentialCredits {
    type ArrangedAccounts = DisableNonConfidentialCreditsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DisableNonConfidentialCreditsInstructionAccounts {
            token: token.pubkey,
            authority: authority.pubkey,
        })
    }
}
