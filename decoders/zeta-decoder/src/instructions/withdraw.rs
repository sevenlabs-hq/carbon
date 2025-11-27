use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw {
    pub amount: u64,
}

pub struct WithdrawInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub socialized_loss_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            zeta_group,
            vault,
            margin_account,
            user_token_account,
            token_program,
            authority,
            greeks,
            oracle,
            oracle_backup_feed,
            oracle_backup_program,
            socialized_loss_account,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            vault: vault.pubkey,
            margin_account: margin_account.pubkey,
            user_token_account: user_token_account.pubkey,
            token_program: token_program.pubkey,
            authority: authority.pubkey,
            greeks: greeks.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
        })
    }
}
