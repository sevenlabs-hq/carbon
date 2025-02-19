use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04")]
pub struct Approve {
    pub amount: u64,
}

pub struct ApproveInstructionAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Approve {
    type ArrangedAccounts = ApproveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, delegate, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ApproveInstructionAccounts {
            source: source.pubkey,
            delegate: delegate.pubkey,
            owner: owner.pubkey,
        })
    }
}
