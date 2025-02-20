use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11a4522db756bfc7")]
pub struct AdminDisableUpdatePerpBidAskTwap {
    pub disable: bool,
}

pub struct AdminDisableUpdatePerpBidAskTwapInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminDisableUpdatePerpBidAskTwap {
    type ArrangedAccounts = AdminDisableUpdatePerpBidAskTwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, user_stats, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AdminDisableUpdatePerpBidAskTwapInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
