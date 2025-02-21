use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb017291c176f0804")]
pub struct AcceptOwner {}

pub struct AcceptOwnerInstructionAccounts {
    pub pending_owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AcceptOwner {
    type ArrangedAccounts = AcceptOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pending_owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AcceptOwnerInstructionAccounts {
            pending_owner: pending_owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
