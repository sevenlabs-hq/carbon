use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x50fc7a3e28da5b64")]
pub struct UpdatePerpMarketMaxSpread {
    pub max_spread: u32,
}

pub struct UpdatePerpMarketMaxSpreadInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub perp_market: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePerpMarketMaxSpread {
    type ArrangedAccounts = UpdatePerpMarketMaxSpreadInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketMaxSpreadInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
        })
    }
}
