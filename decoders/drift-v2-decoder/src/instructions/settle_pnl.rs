use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2b3dea2d0f5f9899")]
pub struct SettlePnl {
    pub market_index: u16,
}

pub struct SettlePnlInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettlePnl {
    type ArrangedAccounts = SettlePnlInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, spot_market_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SettlePnlInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
        })
    }
}
