use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5db000cd453f5750")]
pub struct ChangeMaxSupply {
    pub new_max_supply: u64,
}

pub struct ChangeMaxSupplyInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangeMaxSupply {
    type ArrangedAccounts = ChangeMaxSupplyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ChangeMaxSupplyInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
