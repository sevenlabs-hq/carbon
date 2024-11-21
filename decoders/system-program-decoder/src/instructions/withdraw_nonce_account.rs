use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05")]
pub struct WithdrawNonceAccount(u64);

pub struct WithdrawNonceAccountAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
    pub recipient_account: solana_sdk::pubkey::Pubkey,
    pub recent_blockhashes_sysvar: solana_sdk::pubkey::Pubkey,
    pub rent_sysvar: solana_sdk::pubkey::Pubkey,
    pub nonce_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawNonceAccount {
    type ArrangedAccounts = WithdrawNonceAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let nonce_account = accounts.get(0)?;
        let recipient_account = accounts.get(1)?;
        let recent_blockhashes_sysvar = accounts.get(2)?;
        let rent_sysvar = accounts.get(3)?;
        let nonce_authority = accounts.get(4)?;

        Some(WithdrawNonceAccountAccounts {
            nonce_account: nonce_account.pubkey,
            recipient_account: recipient_account.pubkey,
            recent_blockhashes_sysvar: recent_blockhashes_sysvar.pubkey,
            rent_sysvar: rent_sysvar.pubkey,
            nonce_authority: nonce_authority.pubkey,
        })
    }
}
