use carbon_core::{borsh, deserialize::PrefixString, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
#[carbon(discriminator = "0x0b000000")]
pub struct TransferSolWithSeed {
    pub amount: u64,
    pub from_seed: PrefixString,
    pub from_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferSolWithSeedInstructionAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub base_account: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferSolWithSeed {
    type ArrangedAccounts = TransferSolWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
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
