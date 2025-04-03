use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x18570a73a5be508b")]
pub struct RecenterPerpMarketAmm {
    pub peg_multiplier: u128,
    pub sqrt_k: u128,
}

pub struct RecenterPerpMarketAmmInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RecenterPerpMarketAmm {
    type ArrangedAccounts = RecenterPerpMarketAmmInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RecenterPerpMarketAmmInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
        })
    }
}
