use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub amount: u64,
}

pub struct DepositInstructionAccounts {
    pub zeta_group: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub socialized_loss_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [zeta_group, margin_account, vault, user_token_account, socialized_loss_account, authority, token_program, state, greeks, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInstructionAccounts {
            zeta_group: zeta_group.pubkey,
            margin_account: margin_account.pubkey,
            vault: vault.pubkey,
            user_token_account: user_token_account.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
            authority: authority.pubkey,
            token_program: token_program.pubkey,
            state: state.pubkey,
            greeks: greeks.pubkey,
        })
    }
}
