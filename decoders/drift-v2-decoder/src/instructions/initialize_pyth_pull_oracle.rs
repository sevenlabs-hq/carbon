use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf98cfdf3f84af0ee")]
pub struct InitializePythPullOracle {
    pub feed_id: [u8; 32],
}

pub struct InitializePythPullOracleInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub pyth_solana_receiver: solana_pubkey::Pubkey,
    pub price_feed: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePythPullOracle {
    type ArrangedAccounts = InitializePythPullOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, pyth_solana_receiver, price_feed, system_program, state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePythPullOracleInstructionAccounts {
            admin: admin.pubkey,
            pyth_solana_receiver: pyth_solana_receiver.pubkey,
            price_feed: price_feed.pubkey,
            system_program: system_program.pubkey,
            state: state.pubkey,
        })
    }
}
