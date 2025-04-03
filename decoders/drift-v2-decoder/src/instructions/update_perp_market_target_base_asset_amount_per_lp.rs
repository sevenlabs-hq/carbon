use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3e5744731d9696a5")]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLp {
    pub target_base_asset_amount_per_lp: i32,
}

pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePerpMarketTargetBaseAssetAmountPerLp {
    type ArrangedAccounts = UpdatePerpMarketTargetBaseAssetAmountPerLpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(
            UpdatePerpMarketTargetBaseAssetAmountPerLpInstructionAccounts {
                admin: admin.pubkey,
                state: state.pubkey,
                perp_market: perp_market.pubkey,
            },
        )
    }
}
