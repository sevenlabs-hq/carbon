use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct Transfer {
    pub amount: u64,
}

pub struct TransferAccounts {
    pub source: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, destination, authority, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(TransferAccounts {
            source: source.pubkey,
            destination: destination.pubkey,
            authority: authority.pubkey,
            // TODO: Check
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
