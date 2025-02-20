use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa21523fb2039a1d2")]
pub struct LogUserBalances {}

pub struct LogUserBalancesInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LogUserBalances {
    type ArrangedAccounts = LogUserBalancesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LogUserBalancesInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            user: user.pubkey,
        })
    }
}
