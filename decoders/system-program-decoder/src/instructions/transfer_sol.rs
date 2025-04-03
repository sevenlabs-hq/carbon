use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02000000")]
pub struct TransferSol {
    pub amount: u64,
}

pub struct TransferSolInstructionAccounts {
    pub source: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferSol {
    type ArrangedAccounts = TransferSolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, destination, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferSolInstructionAccounts {
            source: source.pubkey,
            destination: destination.pubkey,
        })
    }
}
