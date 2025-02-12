use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07160c53f22b3079")]
pub struct TransferRewardOwner {
    pub new_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferRewardOwnerInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferRewardOwner {
    type ArrangedAccounts = TransferRewardOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let pool_state = accounts.get(1)?;

        Some(TransferRewardOwnerInstructionAccounts {
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
        })
    }
}
