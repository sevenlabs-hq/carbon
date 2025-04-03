use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct Burn {
    pub amount: u64,
}

pub struct BurnAccounts {
    pub account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for Burn {
    type ArrangedAccounts = BurnAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, owner, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(BurnAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            owner: owner.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
