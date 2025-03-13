use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6cc91e572f4161bc")]
pub struct LendingPoolAccrueBankInterest {}

pub struct LendingPoolAccrueBankInterestInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub bank: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolAccrueBankInterest {
    type ArrangedAccounts = LendingPoolAccrueBankInterestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, bank, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LendingPoolAccrueBankInterestInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            bank: bank.pubkey,
        })
    }
}
