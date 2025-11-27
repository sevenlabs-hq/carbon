use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa99004260a8dbcff")]
pub struct Unpause {}

pub struct UnpauseInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Unpause {
    type ArrangedAccounts = UnpauseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UnpauseInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
