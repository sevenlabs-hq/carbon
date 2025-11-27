use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw {
    pub market_index: u16,
    pub amount: u64,
    pub reduce_only: bool,
}

pub struct WithdrawInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, user_stats, authority, spot_market_vault, drift_signer, user_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            user_token_account: user_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
