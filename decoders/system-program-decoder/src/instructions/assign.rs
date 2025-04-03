use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01000000")]
pub struct Assign {
    pub program_address: solana_pubkey::Pubkey,
}

pub struct AssignInstructionAccounts {
    pub account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Assign {
    type ArrangedAccounts = AssignInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AssignInstructionAccounts {
            account: account.pubkey,
        })
    }
}
