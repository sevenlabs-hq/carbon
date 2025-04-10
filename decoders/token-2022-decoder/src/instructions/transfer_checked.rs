use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c")]
pub struct TransferChecked {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize,)]
pub struct TransferCheckedInstructionAccounts {
    pub source: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferChecked {
    type ArrangedAccounts = TransferCheckedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, mint, destination, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferCheckedInstructionAccounts {
            source: source.pubkey,
            mint: mint.pubkey,
            destination: destination.pubkey,
            authority: authority.pubkey,
        })
    }
}
