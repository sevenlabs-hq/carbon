use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x747a899ee0c3ad77")]
pub struct PostPythPullOracleUpdateAtomic {
    pub feed_id: [u8; 32],
    pub params: Vec<u8>,
}

pub struct PostPythPullOracleUpdateAtomicInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub pyth_solana_receiver: solana_pubkey::Pubkey,
    pub guardian_set: solana_pubkey::Pubkey,
    pub price_feed: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PostPythPullOracleUpdateAtomic {
    type ArrangedAccounts = PostPythPullOracleUpdateAtomicInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, pyth_solana_receiver, guardian_set, price_feed, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(PostPythPullOracleUpdateAtomicInstructionAccounts {
            keeper: keeper.pubkey,
            pyth_solana_receiver: pyth_solana_receiver.pubkey,
            guardian_set: guardian_set.pubkey,
            price_feed: price_feed.pubkey,
        })
    }
}
