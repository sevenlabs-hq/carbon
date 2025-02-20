use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe2fd4c471102aba9")]
pub struct UpdateSpotMarketFuel {
    pub fuel_boost_deposits: Option<u8>,
    pub fuel_boost_borrows: Option<u8>,
    pub fuel_boost_taker: Option<u8>,
    pub fuel_boost_maker: Option<u8>,
    pub fuel_boost_insurance: Option<u8>,
}

pub struct UpdateSpotMarketFuelInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub spot_market: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotMarketFuel {
    type ArrangedAccounts = UpdateSpotMarketFuelInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketFuelInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
        })
    }
}
