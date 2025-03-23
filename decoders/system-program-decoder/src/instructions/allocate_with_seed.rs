use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09000000")]
pub struct AllocateWithSeed {
    pub base: solana_sdk::pubkey::Pubkey,
    pub seed: String,
    pub space: u64,
    pub program_address: solana_sdk::pubkey::Pubkey,
}

pub struct AllocateWithSeedInstructionAccounts {
    pub new_account: solana_sdk::pubkey::Pubkey,
    pub base_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AllocateWithSeed {
    type ArrangedAccounts = AllocateWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [new_account, base_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AllocateWithSeedInstructionAccounts {
            new_account: new_account.pubkey,
            base_account: base_account.pubkey,
        })
    }
}
