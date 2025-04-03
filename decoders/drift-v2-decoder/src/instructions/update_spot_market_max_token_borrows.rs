use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3966ccd4fd5f0dc7")]
pub struct UpdateSpotMarketMaxTokenBorrows {
    pub max_token_borrows_fraction: u16,
}

pub struct UpdateSpotMarketMaxTokenBorrowsInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotMarketMaxTokenBorrows {
    type ArrangedAccounts = UpdateSpotMarketMaxTokenBorrowsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketMaxTokenBorrowsInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
        })
    }
}
