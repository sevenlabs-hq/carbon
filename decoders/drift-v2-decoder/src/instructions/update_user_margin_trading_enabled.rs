use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc25cccdff6bc1fcb")]
pub struct UpdateUserMarginTradingEnabled {
    pub sub_account_id: u16,
    pub margin_trading_enabled: bool,
}

pub struct UpdateUserMarginTradingEnabledInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserMarginTradingEnabled {
    type ArrangedAccounts = UpdateUserMarginTradingEnabledInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserMarginTradingEnabledInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
