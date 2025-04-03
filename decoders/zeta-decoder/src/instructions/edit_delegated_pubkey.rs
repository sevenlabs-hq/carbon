use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x89f547592ef91635")]
pub struct EditDelegatedPubkey {
    pub new_key: solana_pubkey::Pubkey,
}

pub struct EditDelegatedPubkeyInstructionAccounts {
    pub margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditDelegatedPubkey {
    type ArrangedAccounts = EditDelegatedPubkeyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [margin_account, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EditDelegatedPubkeyInstructionAccounts {
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
        })
    }
}
