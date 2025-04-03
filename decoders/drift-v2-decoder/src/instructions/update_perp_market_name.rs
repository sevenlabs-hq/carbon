use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd31f15d2406c42c9")]
pub struct UpdatePerpMarketName {
    pub name: [u8; 32],
}

pub struct UpdatePerpMarketNameInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePerpMarketName {
    type ArrangedAccounts = UpdatePerpMarketNameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketNameInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
        })
    }
}
