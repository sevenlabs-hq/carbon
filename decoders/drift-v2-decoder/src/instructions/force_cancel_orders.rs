use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x40b5c43fde4840e8")]
pub struct ForceCancelOrders {}

pub struct ForceCancelOrdersInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub filler: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForceCancelOrders {
    type ArrangedAccounts = ForceCancelOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, filler, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ForceCancelOrdersInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            filler: filler.pubkey,
            user: user.pubkey,
        })
    }
}
