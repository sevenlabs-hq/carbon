use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x38bf4f121a7950d0")]
pub struct UpdateSpotMarketMaxTokenDeposits {
    pub max_token_deposits: u64,
}

pub struct UpdateSpotMarketMaxTokenDepositsInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotMarketMaxTokenDeposits {
    type ArrangedAccounts = UpdateSpotMarketMaxTokenDepositsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketMaxTokenDepositsInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
        })
    }
}
