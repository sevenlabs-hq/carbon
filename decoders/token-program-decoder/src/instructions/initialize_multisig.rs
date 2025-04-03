use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct InitializeMultisig {
    pub m: u8,
}

pub struct InitializeMultisigAccounts {
    pub account: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMultisig {
    type ArrangedAccounts = InitializeMultisigAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, rent, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(InitializeMultisigAccounts {
            account: account.pubkey,
            rent: rent.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
