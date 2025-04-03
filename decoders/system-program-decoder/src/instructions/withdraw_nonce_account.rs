use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05000000")]
pub struct WithdrawNonceAccount {
    pub withdraw_amount: u64,
}

pub struct WithdrawNonceAccountInstructionAccounts {
    pub nonce_account: solana_pubkey::Pubkey,
    pub recipient_account: solana_pubkey::Pubkey,
    pub recent_blockhashes_sysvar: solana_pubkey::Pubkey,
    pub rent_sysvar: solana_pubkey::Pubkey,
    pub nonce_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawNonceAccount {
    type ArrangedAccounts = WithdrawNonceAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nonce_account, recipient_account, recent_blockhashes_sysvar, rent_sysvar, nonce_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawNonceAccountInstructionAccounts {
            nonce_account: nonce_account.pubkey,
            recipient_account: recipient_account.pubkey,
            recent_blockhashes_sysvar: recent_blockhashes_sysvar.pubkey,
            rent_sysvar: rent_sysvar.pubkey,
            nonce_authority: nonce_authority.pubkey,
        })
    }
}
