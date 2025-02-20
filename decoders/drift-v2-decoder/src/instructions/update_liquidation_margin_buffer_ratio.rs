use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x84e0f3a09a5261d7")]
pub struct UpdateLiquidationMarginBufferRatio {
    pub liquidation_margin_buffer_ratio: u32,
}

pub struct UpdateLiquidationMarginBufferRatioInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateLiquidationMarginBufferRatio {
    type ArrangedAccounts = UpdateLiquidationMarginBufferRatioInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateLiquidationMarginBufferRatioInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
