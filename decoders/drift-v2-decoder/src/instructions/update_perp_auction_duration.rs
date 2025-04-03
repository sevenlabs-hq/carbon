use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7e6e34ae1eced75a")]
pub struct UpdatePerpAuctionDuration {
    pub min_perp_auction_duration: u8,
}

pub struct UpdatePerpAuctionDurationInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePerpAuctionDuration {
    type ArrangedAccounts = UpdatePerpAuctionDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpAuctionDurationInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
