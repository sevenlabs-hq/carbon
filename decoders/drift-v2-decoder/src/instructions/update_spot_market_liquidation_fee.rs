use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b0dff35388868b1")]
pub struct UpdateSpotMarketLiquidationFee {
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
}

pub struct UpdateSpotMarketLiquidationFeeInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub spot_market: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotMarketLiquidationFee {
    type ArrangedAccounts = UpdateSpotMarketLiquidationFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketLiquidationFeeInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
        })
    }
}
