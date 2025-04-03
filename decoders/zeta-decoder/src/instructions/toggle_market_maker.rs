use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcbf7549f68fd9450")]
pub struct ToggleMarketMaker {
    pub is_market_maker: bool,
}

pub struct ToggleMarketMakerInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ToggleMarketMaker {
    type ArrangedAccounts = ToggleMarketMakerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, margin_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ToggleMarketMakerInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            margin_account: margin_account.pubkey,
        })
    }
}
