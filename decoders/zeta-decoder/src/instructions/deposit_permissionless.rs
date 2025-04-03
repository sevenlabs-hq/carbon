use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xebf709f8cc340932")]
pub struct DepositPermissionless {
    pub amount: u64,
}

pub struct DepositPermissionlessInstructionAccounts {
    pub cross_margin_account: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub deposit_token_acc: solana_pubkey::Pubkey,
    pub socialized_loss_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositPermissionless {
    type ArrangedAccounts = DepositPermissionlessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cross_margin_account, vault, deposit_token_acc, socialized_loss_account, authority, payer, token_program, state, pricing, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositPermissionlessInstructionAccounts {
            cross_margin_account: cross_margin_account.pubkey,
            vault: vault.pubkey,
            deposit_token_acc: deposit_token_acc.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            token_program: token_program.pubkey,
            state: state.pubkey,
            pricing: pricing.pubkey,
        })
    }
}
