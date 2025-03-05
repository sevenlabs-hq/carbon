use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc541074997698569")]
pub struct GetMinimumDelegation {}

pub struct GetMinimumDelegationInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for GetMinimumDelegation {
    type ArrangedAccounts = GetMinimumDelegationInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(GetMinimumDelegationInstructionAccounts {})
    }
}
