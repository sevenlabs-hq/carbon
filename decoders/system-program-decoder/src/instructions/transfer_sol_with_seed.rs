use carbon_core::{borsh, deserialize::U64PrefixString, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b000000")]
pub struct TransferSolWithSeed {
    pub amount: u64,
    pub from_seed: U64PrefixString,
    pub from_owner: solana_pubkey::Pubkey,
}

pub struct TransferSolWithSeedInstructionAccounts {
    pub source: solana_pubkey::Pubkey,
    pub base_account: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferSolWithSeed {
    type ArrangedAccounts = TransferSolWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, base_account, destination, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferSolWithSeedInstructionAccounts {
            source: source.pubkey,
            base_account: base_account.pubkey,
            destination: destination.pubkey,
        })
    }
}
