use carbon_core::{borsh, deserialize::U64PrefixString, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone, Eq, Hash,
)]
#[carbon(discriminator = "0x03000000")]
pub struct CreateAccountWithSeed {
    pub base: solana_pubkey::Pubkey,
    pub seed: U64PrefixString,
    pub amount: u64,
    pub space: u64,
    pub program_address: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq)]
pub struct CreateAccountWithSeedInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub new_account: solana_pubkey::Pubkey,
    pub base_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateAccountWithSeed {
    type ArrangedAccounts = CreateAccountWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, new_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateAccountWithSeedInstructionAccounts {
            payer: payer.pubkey,
            new_account: new_account.pubkey,
            base_account: accounts.get(2).unwrap_or(payer).pubkey,
        })
    }
}
