use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07000000")]
pub struct AuthorizeNonceAccount {
    pub new_nonce_authority: solana_pubkey::Pubkey,
}

pub struct AuthorizeNonceAccountInstructionAccounts {
    pub nonce_account: solana_pubkey::Pubkey,
    pub nonce_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AuthorizeNonceAccount {
    type ArrangedAccounts = AuthorizeNonceAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nonce_account, nonce_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AuthorizeNonceAccountInstructionAccounts {
            nonce_account: nonce_account.pubkey,
            nonce_authority: nonce_authority.pubkey,
        })
    }
}
