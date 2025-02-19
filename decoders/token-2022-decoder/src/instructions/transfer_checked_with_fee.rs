use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1a")]
pub struct TransferCheckedWithFee {
    pub transfer_fee_discriminator: u8,
    pub amount: u64,
    pub decimals: u8,
    pub fee: u64,
}

pub struct TransferCheckedWithFeeInstructionAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferCheckedWithFee {
    type ArrangedAccounts = TransferCheckedWithFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, mint, destination, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferCheckedWithFeeInstructionAccounts {
            source: source.pubkey,
            mint: mint.pubkey,
            destination: destination.pubkey,
            authority: authority.pubkey,
        })
    }
}
