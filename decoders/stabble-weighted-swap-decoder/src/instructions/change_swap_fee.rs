use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe70f843384a540aa")]
pub struct ChangeSwapFee {
    pub new_swap_fee: u64,
}

pub struct ChangeSwapFeeInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangeSwapFee {
    type ArrangedAccounts = ChangeSwapFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ChangeSwapFeeInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
