use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08000000")]
pub struct Allocate {
    pub space: u64,
}

pub struct AllocateInstructionAccounts {
    pub new_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Allocate {
    type ArrangedAccounts = AllocateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [new_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AllocateInstructionAccounts {
            new_account: new_account.pubkey,
        })
    }
}
