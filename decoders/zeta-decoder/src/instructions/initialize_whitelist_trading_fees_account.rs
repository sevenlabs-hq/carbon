use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc681d8b9f71d69be")]
pub struct InitializeWhitelistTradingFeesAccount {
    pub nonce: u8,
}

pub struct InitializeWhitelistTradingFeesAccountInstructionAccounts {
    pub whitelist_trading_fees_account: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeWhitelistTradingFeesAccount {
    type ArrangedAccounts = InitializeWhitelistTradingFeesAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whitelist_trading_fees_account, admin, user, system_program, state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeWhitelistTradingFeesAccountInstructionAccounts {
            whitelist_trading_fees_account: whitelist_trading_fees_account.pubkey,
            admin: admin.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            state: state.pubkey,
        })
    }
}
