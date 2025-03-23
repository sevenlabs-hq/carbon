use carbon_core::{borsh, deserialize::PrefixString, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x03000000")]
pub struct CreateAccountWithSeed {
    pub base: solana_sdk::pubkey::Pubkey,
    pub seed: PrefixString,
    pub amount: u64,
    pub space: u64,
    pub program_address: solana_sdk::pubkey::Pubkey,
}

#[derive(Debug, PartialEq)]
pub struct CreateAccountWithSeedInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub new_account: solana_sdk::pubkey::Pubkey,
    pub base_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateAccountWithSeed {
    type ArrangedAccounts = CreateAccountWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, new_account, base_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateAccountWithSeedInstructionAccounts {
            payer: payer.pubkey,
            new_account: new_account.pubkey,
            base_account: base_account.pubkey,
        })
    }
}
