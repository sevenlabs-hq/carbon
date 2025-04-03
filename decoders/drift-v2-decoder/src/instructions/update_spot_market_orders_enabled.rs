use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbe4fce0f1ae5e52b")]
pub struct UpdateSpotMarketOrdersEnabled {
    pub orders_enabled: bool,
}

pub struct UpdateSpotMarketOrdersEnabledInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotMarketOrdersEnabled {
    type ArrangedAccounts = UpdateSpotMarketOrdersEnabledInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketOrdersEnabledInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
        })
    }
}
