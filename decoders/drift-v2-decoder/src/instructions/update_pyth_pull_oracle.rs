use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe6bfbd5e6c3b4ac5")]
pub struct UpdatePythPullOracle {
    pub feed_id: [u8; 32],
    pub params: Vec<u8>,
}

pub struct UpdatePythPullOracleInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub pyth_solana_receiver: solana_sdk::pubkey::Pubkey,
    pub encoded_vaa: solana_sdk::pubkey::Pubkey,
    pub price_feed: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePythPullOracle {
    type ArrangedAccounts = UpdatePythPullOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, pyth_solana_receiver, encoded_vaa, price_feed, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(UpdatePythPullOracleInstructionAccounts {
            keeper: keeper.pubkey,
            pyth_solana_receiver: pyth_solana_receiver.pubkey,
            encoded_vaa: encoded_vaa.pubkey,
            price_feed: price_feed.pubkey,
        })
    }
}
