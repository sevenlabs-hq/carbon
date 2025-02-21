use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6d4b4599acda9213")]
pub struct DepositV2 {
    pub amount: u64,
}

pub struct DepositV2InstructionAccounts {
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub socialized_loss_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositV2 {
    type ArrangedAccounts = DepositV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [margin_account, vault, user_token_account, socialized_loss_account, authority, token_program, state, pricing, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositV2InstructionAccounts {
            margin_account: margin_account.pubkey,
            vault: vault.pubkey,
            user_token_account: user_token_account.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
            authority: authority.pubkey,
            token_program: token_program.pubkey,
            state: state.pubkey,
            pricing: pricing.pubkey,
        })
    }
}
