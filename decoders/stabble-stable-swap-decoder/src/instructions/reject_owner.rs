use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeecec6d733b285e4")]
pub struct RejectOwner {}

pub struct RejectOwnerInstructionAccounts {
    pub pending_owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RejectOwner {
    type ArrangedAccounts = RejectOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pending_owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RejectOwnerInstructionAccounts {
            pending_owner: pending_owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
