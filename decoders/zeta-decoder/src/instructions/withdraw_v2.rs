use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf250a300c4ddc2c2")]
pub struct WithdrawV2 {
    pub amount: u64,
}

pub struct WithdrawV2InstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub socialized_loss_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawV2 {
    type ArrangedAccounts = WithdrawV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, vault, margin_account, user_token_account, token_program, authority, socialized_loss_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawV2InstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            vault: vault.pubkey,
            margin_account: margin_account.pubkey,
            user_token_account: user_token_account.pubkey,
            token_program: token_program.pubkey,
            authority: authority.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
        })
    }
}
