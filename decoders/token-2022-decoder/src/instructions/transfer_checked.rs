use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c")]
pub struct TransferChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct TransferCheckedInstructionAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferChecked {
    type ArrangedAccounts = TransferCheckedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
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
