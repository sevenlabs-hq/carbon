use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06000000")]
pub struct InitializeNonceAccount {
    pub nonce_authority: solana_sdk::pubkey::Pubkey,
}

pub struct InitializeNonceAccountInstructionAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
    pub recent_blockhashes_sysvar: solana_sdk::pubkey::Pubkey,
    pub rent_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeNonceAccount {
    type ArrangedAccounts = InitializeNonceAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nonce_account, recent_blockhashes_sysvar, rent_sysvar, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(InitializeNonceAccountInstructionAccounts {
            nonce_account: nonce_account.pubkey,
            recent_blockhashes_sysvar: recent_blockhashes_sysvar.pubkey,
            rent_sysvar: rent_sysvar.pubkey,
        })
    }
}
