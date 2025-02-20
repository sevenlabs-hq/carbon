use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1c9a14f966c04947")]
pub struct UpdateLiquidationDuration {
    pub liquidation_duration: u8,
}

pub struct UpdateLiquidationDurationInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateLiquidationDuration {
    type ArrangedAccounts = UpdateLiquidationDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateLiquidationDurationInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
